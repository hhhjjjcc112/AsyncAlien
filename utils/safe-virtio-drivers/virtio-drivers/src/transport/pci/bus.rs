//! PCI 总线最小抽象（无 unsafe）。

use bitflags::bitflags;
use core::{
    array,
    convert::TryFrom,
    fmt::{self, Display, Formatter},
};
use log::warn;

const INVALID_READ: u32 = 0xffff_ffff;
const MAX_DEVICES: u8 = 32;
const MAX_FUNCTIONS: u8 = 8;
const STATUS_COMMAND_OFFSET: u8 = 0x04;
const BAR0_OFFSET: u8 = 0x10;

/// 厂商自定义 capability ID。
pub const PCI_CAP_ID_VNDR: u8 = 0x09;

bitflags! {
    /// PCI 状态寄存器。
    #[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
    pub struct Status: u16 {
        const INTERRUPT_STATUS = 1 << 3;
        const CAPABILITIES_LIST = 1 << 4;
        const MHZ_66_CAPABLE = 1 << 5;
        const FAST_BACK_TO_BACK_CAPABLE = 1 << 7;
        const MASTER_DATA_PARITY_ERROR = 1 << 8;
        const SIGNALED_TARGET_ABORT = 1 << 11;
        const RECEIVED_TARGET_ABORT = 1 << 12;
        const RECEIVED_MASTER_ABORT = 1 << 13;
        const SIGNALED_SYSTEM_ERROR = 1 << 14;
        const DETECTED_PARITY_ERROR = 1 << 15;
    }
}

bitflags! {
    /// PCI 命令寄存器。
    #[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
    pub struct Command: u16 {
        const IO_SPACE = 1 << 0;
        const MEMORY_SPACE = 1 << 1;
        const BUS_MASTER = 1 << 2;
        const SPECIAL_CYCLES = 1 << 3;
        const MEMORY_WRITE_AND_INVALIDATE_ENABLE = 1 << 4;
        const VGA_PALETTE_SNOOP = 1 << 5;
        const PARITY_ERROR_RESPONSE = 1 << 6;
        const SERR_ENABLE = 1 << 8;
        const FAST_BACK_TO_BACK_ENABLE = 1 << 9;
        const INTERRUPT_DISABLE = 1 << 10;
    }
}

/// PCI 访问错误。
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PciError {
    InvalidBarType,
}

/// 配置访问机制。
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Cam {
    MmioCam,
    Ecam,
}

impl Cam {
    /// CAM 总大小（字节）。
    pub const fn size(self) -> u32 {
        match self {
            Self::MmioCam => 0x0100_0000,
            Self::Ecam => 0x1000_0000,
        }
    }

    /// 计算给定 BDF/寄存器在 CAM 中的偏移（字节）。
    pub fn cam_offset(self, device_function: DeviceFunction, register_offset: u8) -> u32 {
        assert!(device_function.valid());
        let bdf = ((device_function.bus as u32) << 8)
            | ((device_function.device as u32) << 3)
            | (device_function.function as u32);
        let shift = match self {
            Cam::MmioCam => 8,
            Cam::Ecam => 12,
        };
        let address = (bdf << shift) | (register_offset as u32);
        assert!(address < self.size());
        assert_eq!(address & 0x3, 0);
        address
    }
}

/// PCI 配置空间访问抽象。
pub trait ConfigurationAccess: Clone {
    fn read_word(&self, device_function: DeviceFunction, register_offset: u8) -> u32;
    fn write_word(&mut self, device_function: DeviceFunction, register_offset: u8, data: u32);
}

/// PCI Root。
pub struct PciRoot<C: ConfigurationAccess> {
    pub configuration_access: C,
}

impl<C: ConfigurationAccess> PciRoot<C> {
    pub fn new(configuration_access: C) -> Self {
        Self {
            configuration_access,
        }
    }

    /// 枚举指定总线上的所有设备函数。
    pub fn enumerate_bus(&self, bus: u8) -> BusDeviceIterator<C> {
        BusDeviceIterator {
            configuration_access: self.configuration_access.clone(),
            next: DeviceFunction {
                bus,
                device: 0,
                function: 0,
            },
        }
    }

    /// 读取 status/command。
    pub fn get_status_command(&self, device_function: DeviceFunction) -> (Status, Command) {
        let status_command = self
            .configuration_access
            .read_word(device_function, STATUS_COMMAND_OFFSET);
        let status = Status::from_bits_truncate((status_command >> 16) as u16);
        let command = Command::from_bits_truncate(status_command as u16);
        (status, command)
    }

    /// 写 command。
    pub fn set_command(&mut self, device_function: DeviceFunction, command: Command) {
        self.configuration_access.write_word(
            device_function,
            STATUS_COMMAND_OFFSET,
            command.bits() as u32,
        );
    }

    /// capability 迭代器。
    pub fn capabilities(&self, device_function: DeviceFunction) -> CapabilityIterator<'_, C> {
        CapabilityIterator {
            configuration_access: &self.configuration_access,
            device_function,
            next_capability_offset: self.capabilities_offset(device_function),
        }
    }

    /// 读取全部 BAR 信息。
    pub fn bars(
        &mut self,
        device_function: DeviceFunction,
    ) -> Result<[Option<BarInfo>; 6], PciError> {
        let mut bars = array::from_fn(|_| None);
        let mut bar_index = 0u8;
        while bar_index < 6 {
            let info = self.bar_info(device_function, bar_index)?;
            let step = if info.as_ref().is_some_and(BarInfo::takes_two_entries) {
                2
            } else {
                1
            };
            bars[bar_index as usize] = info;
            bar_index += step;
        }
        Ok(bars)
    }

    /// 读取单个 BAR 信息。
    pub fn bar_info(
        &mut self,
        device_function: DeviceFunction,
        bar_index: u8,
    ) -> Result<Option<BarInfo>, PciError> {
        if bar_index >= 6 {
            return Ok(None);
        }

        let (_status, command_orig) = self.get_status_command(device_function);
        let command_disable_decode = command_orig & !(Command::IO_SPACE | Command::MEMORY_SPACE);
        if command_disable_decode != command_orig {
            self.set_command(device_function, command_disable_decode);
        }

        let bar_offset = BAR0_OFFSET + 4 * bar_index;
        let bar_orig = self.configuration_access.read_word(device_function, bar_offset);
        let io_space = (bar_orig & 0x1) == 0x1;

        self.configuration_access
            .write_word(device_function, bar_offset, 0xffff_ffff);
        let mut size_mask = self.configuration_access.read_word(device_function, bar_offset) as u64;

        let (address_top, size_top) = if (bar_orig & 0b111) == 0b100 {
            if bar_index >= 5 {
                if command_disable_decode != command_orig {
                    self.set_command(device_function, command_orig);
                }
                return Err(PciError::InvalidBarType);
            }
            let bar_top_offset = BAR0_OFFSET + 4 * (bar_index + 1);
            let bar_top_orig = self
                .configuration_access
                .read_word(device_function, bar_top_offset);
            self.configuration_access
                .write_word(device_function, bar_top_offset, 0xffff_ffff);
            let size_top = self
                .configuration_access
                .read_word(device_function, bar_top_offset);
            self.configuration_access
                .write_word(device_function, bar_top_offset, bar_top_orig);
            (bar_top_orig, size_top)
        } else {
            let size_top = if size_mask == 0 { 0 } else { 0xffff_ffff };
            (0, size_top)
        };
        size_mask |= (size_top as u64) << 32;

        let flag_bits = if io_space { 0b11u64 } else { 0b1111u64 };
        let size = (!(size_mask & !flag_bits)).wrapping_add(1);

        self.configuration_access
            .write_word(device_function, bar_offset, bar_orig);
        if command_disable_decode != command_orig {
            self.set_command(device_function, command_orig);
        }

        if size_mask == 0 {
            Ok(None)
        } else if io_space {
            Ok(Some(BarInfo::IO {
                address: bar_orig & 0xffff_fffc,
                size: size as u32,
            }))
        } else {
            let address = ((address_top as u64) << 32) | ((bar_orig & 0xffff_fff0) as u64);
            let prefetchable = (bar_orig & 0x8) != 0;
            let address_type = MemoryBarType::try_from(((bar_orig & 0x6) >> 1) as u8)?;
            Ok(Some(BarInfo::Memory {
                address_type,
                prefetchable,
                address,
                size,
            }))
        }
    }

    /// 设置 32 位 BAR。
    pub fn set_bar_32(&mut self, device_function: DeviceFunction, bar_index: u8, address: u32) {
        self.configuration_access
            .write_word(device_function, BAR0_OFFSET + 4 * bar_index, address);
    }

    /// 设置 64 位 BAR。
    pub fn set_bar_64(&mut self, device_function: DeviceFunction, bar_index: u8, address: u64) {
        self.configuration_access.write_word(
            device_function,
            BAR0_OFFSET + 4 * bar_index,
            address as u32,
        );
        self.configuration_access.write_word(
            device_function,
            BAR0_OFFSET + 4 * (bar_index + 1),
            (address >> 32) as u32,
        );
    }

    fn capabilities_offset(&self, device_function: DeviceFunction) -> Option<u8> {
        let (status, _) = self.get_status_command(device_function);
        if status.contains(Status::CAPABILITIES_LIST) {
            Some((self.configuration_access.read_word(device_function, 0x34) & 0xfc) as u8)
        } else {
            None
        }
    }
}

/// BAR 信息。
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BarInfo {
    Memory {
        address_type: MemoryBarType,
        prefetchable: bool,
        address: u64,
        size: u64,
    },
    IO {
        address: u32,
        size: u32,
    },
}

impl BarInfo {
    pub fn takes_two_entries(&self) -> bool {
        matches!(
            self,
            BarInfo::Memory {
                address_type: MemoryBarType::Width64,
                ..
            }
        )
    }

    pub fn memory_address_size(&self) -> Option<(u64, u64)> {
        if let Self::Memory { address, size, .. } = self {
            Some((*address, *size))
        } else {
            None
        }
    }
}

impl Display for BarInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Memory {
                address_type,
                prefetchable,
                address,
                size,
            } => write!(
                f,
                "Memory space at {:#010x}, size {}, type {:?}, prefetchable {}",
                address, size, address_type, prefetchable
            ),
            Self::IO { address, size } => {
                write!(f, "I/O space at {:#010x}, size {}", address, size)
            }
        }
    }
}

/// 内存 BAR 类型。
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MemoryBarType {
    Width32,
    Below1MiB,
    Width64,
}

impl From<MemoryBarType> for u8 {
    fn from(bar_type: MemoryBarType) -> Self {
        match bar_type {
            MemoryBarType::Width32 => 0,
            MemoryBarType::Below1MiB => 1,
            MemoryBarType::Width64 => 2,
        }
    }
}

impl TryFrom<u8> for MemoryBarType {
    type Error = PciError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Width32),
            1 => Ok(Self::Below1MiB),
            2 => Ok(Self::Width64),
            _ => Err(PciError::InvalidBarType),
        }
    }
}

/// capability 迭代器。
#[derive(Debug)]
pub struct CapabilityIterator<'a, C: ConfigurationAccess> {
    configuration_access: &'a C,
    device_function: DeviceFunction,
    next_capability_offset: Option<u8>,
}

impl<C: ConfigurationAccess> Iterator for CapabilityIterator<'_, C> {
    type Item = CapabilityInfo;

    fn next(&mut self) -> Option<Self::Item> {
        let offset = self.next_capability_offset?;
        let capability_header = self
            .configuration_access
            .read_word(self.device_function, offset);
        let id = capability_header as u8;
        let next_offset = (capability_header >> 8) as u8;
        let private_header = (capability_header >> 16) as u16;

        self.next_capability_offset = if next_offset == 0 {
            None
        } else if next_offset < 64 || (next_offset & 0x3) != 0 {
            warn!("Invalid next capability offset {:#04x}", next_offset);
            None
        } else {
            Some(next_offset)
        };

        Some(CapabilityInfo {
            offset,
            id,
            private_header,
        })
    }
}

/// capability 摘要。
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CapabilityInfo {
    pub offset: u8,
    pub id: u8,
    pub private_header: u16,
}

/// 指定总线上的设备函数枚举器。
#[derive(Debug)]
pub struct BusDeviceIterator<C: ConfigurationAccess> {
    configuration_access: C,
    next: DeviceFunction,
}

impl<C: ConfigurationAccess> Iterator for BusDeviceIterator<C> {
    type Item = (DeviceFunction, DeviceFunctionInfo);

    fn next(&mut self) -> Option<Self::Item> {
        while self.next.device < MAX_DEVICES {
            let current = self.next;
            let device_vendor = self.configuration_access.read_word(current, 0);

            self.next.function += 1;
            if self.next.function >= MAX_FUNCTIONS {
                self.next.function = 0;
                self.next.device += 1;
            }

            if device_vendor != INVALID_READ {
                let class_revision = self.configuration_access.read_word(current, 8);
                let device_id = (device_vendor >> 16) as u16;
                let vendor_id = device_vendor as u16;
                let class = (class_revision >> 24) as u8;
                let subclass = (class_revision >> 16) as u8;
                let prog_if = (class_revision >> 8) as u8;
                let revision = class_revision as u8;
                let bist_type_latency_cache = self.configuration_access.read_word(current, 12);
                let header_type = HeaderType::from((bist_type_latency_cache >> 16) as u8 & 0x7f);
                return Some((
                    current,
                    DeviceFunctionInfo {
                        vendor_id,
                        device_id,
                        class,
                        subclass,
                        prog_if,
                        revision,
                        header_type,
                    },
                ));
            }
        }
        None
    }
}

/// BDF 标识。
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct DeviceFunction {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
}

impl DeviceFunction {
    pub fn valid(&self) -> bool {
        self.device < 32 && self.function < 8
    }
}

impl Display for DeviceFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:02x}:{:02x}.{}", self.bus, self.device, self.function)
    }
}

/// 设备函数信息。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeviceFunctionInfo {
    pub vendor_id: u16,
    pub device_id: u16,
    pub class: u8,
    pub subclass: u8,
    pub prog_if: u8,
    pub revision: u8,
    pub header_type: HeaderType,
}

impl Display for DeviceFunctionInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:04x}:{:04x} (class {:02x}.{:02x}, rev {:02x}) {:?}",
            self.vendor_id,
            self.device_id,
            self.class,
            self.subclass,
            self.revision,
            self.header_type,
        )
    }
}

/// PCI 头类型。
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum HeaderType {
    Standard,
    PciPciBridge,
    PciCardbusBridge,
    Unrecognised(u8),
}

impl From<u8> for HeaderType {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::Standard,
            0x01 => Self::PciPciBridge,
            0x02 => Self::PciCardbusBridge,
            _ => Self::Unrecognised(value),
        }
    }
}
