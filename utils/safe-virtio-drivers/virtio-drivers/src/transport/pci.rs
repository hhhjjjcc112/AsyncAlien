use alloc::{boxed::Box, sync::Arc};

use crate::error::{VirtIoError, VirtIoResult};
use crate::hal::VirtIoDeviceIo;
use crate::queue::Descriptor;
use crate::transport::mmio::CONFIG_OFFSET as MMIO_CONFIG_OFFSET;
use crate::transport::{DeviceStatus, DeviceType, Transport};
use crate::{align_up, PhysAddr, PAGE_SIZE};
use core::mem::size_of;

pub mod bus;

pub use self::bus::{
    Cam, ConfigurationAccess, DeviceFunction, DeviceFunctionInfo, PciError, PciRoot,
    PCI_CAP_ID_VNDR,
};

/// VirtIO 的 PCI vendor id。
pub const VIRTIO_VENDOR_ID: u16 = 0x1af4;

const PCI_DEVICE_ID_OFFSET: u16 = 0x1040;
const TRANSITIONAL_NETWORK: u16 = 0x1000;
const TRANSITIONAL_BLOCK: u16 = 0x1001;
const TRANSITIONAL_MEMORY_BALLOONING: u16 = 0x1002;
const TRANSITIONAL_CONSOLE: u16 = 0x1003;
const TRANSITIONAL_SCSI_HOST: u16 = 0x1004;
const TRANSITIONAL_ENTROPY_SOURCE: u16 = 0x1005;
const TRANSITIONAL_9P_TRANSPORT: u16 = 0x1009;
const TRANSITIONAL_GPU: u16 = 0x1010;
const TRANSITIONAL_INPUT: u16 = 0x1012;

/// 由 PCI device id 推导 VirtIO 设备类型。
pub fn device_type(pci_device_id: u16) -> Option<DeviceType> {
    match pci_device_id {
        TRANSITIONAL_NETWORK => Some(DeviceType::Network),
        TRANSITIONAL_BLOCK => Some(DeviceType::Block),
        TRANSITIONAL_MEMORY_BALLOONING => Some(DeviceType::MemoryBalloon),
        TRANSITIONAL_CONSOLE => Some(DeviceType::Console),
        TRANSITIONAL_SCSI_HOST => Some(DeviceType::ScsiHost),
        TRANSITIONAL_ENTROPY_SOURCE => Some(DeviceType::EntropySource),
        TRANSITIONAL_9P_TRANSPORT => Some(DeviceType::_9P),
        TRANSITIONAL_GPU => Some(DeviceType::GPU),
        TRANSITIONAL_INPUT => Some(DeviceType::Input),
        id if id >= PCI_DEVICE_ID_OFFSET => {
            let ty = DeviceType::from(id - PCI_DEVICE_ID_OFFSET);
            if ty == DeviceType::Invalid {
                None
            } else {
                Some(ty)
            }
        }
        _ => None,
    }
}

/// 根据 PCI 设备信息识别 VirtIO 设备类型。
pub fn virtio_device_type(device_function_info: &DeviceFunctionInfo) -> Option<DeviceType> {
    if device_function_info.vendor_id == VIRTIO_VENDOR_ID {
        device_type(device_function_info.device_id)
    } else {
        None
    }
}

#[derive(Debug)]
struct LegacyPciConfigIo {
    io_region: Arc<dyn VirtIoDeviceIo>,
}

impl LegacyPciConfigIo {
    // virtio legacy pci: 0x14 起为设备配置区。
    const LEGACY_CONFIG_OFFSET: usize = 0x14;

    #[inline]
    fn map_offset(off: usize) -> usize {
        if off >= MMIO_CONFIG_OFFSET {
            off - MMIO_CONFIG_OFFSET + Self::LEGACY_CONFIG_OFFSET
        } else {
            off
        }
    }
}

impl VirtIoDeviceIo for LegacyPciConfigIo {
    fn read_volatile_u32_at(&self, off: usize) -> VirtIoResult<u32> {
        self.io_region.read_volatile_u32_at(Self::map_offset(off))
    }

    fn read_volatile_u16_at(&self, off: usize) -> VirtIoResult<u16> {
        self.io_region.read_volatile_u16_at(Self::map_offset(off))
    }

    fn read_volatile_u8_at(&self, off: usize) -> VirtIoResult<u8> {
        self.io_region.read_volatile_u8_at(Self::map_offset(off))
    }

    fn write_volatile_u32_at(&self, off: usize, data: u32) -> VirtIoResult<()> {
        self.io_region
            .write_volatile_u32_at(Self::map_offset(off), data)
    }

    fn write_volatile_u16_at(&self, off: usize, data: u16) -> VirtIoResult<()> {
        self.io_region
            .write_volatile_u16_at(Self::map_offset(off), data)
    }

    fn write_volatile_u8_at(&self, off: usize, data: u8) -> VirtIoResult<()> {
        self.io_region
            .write_volatile_u8_at(Self::map_offset(off), data)
    }

    fn paddr(&self) -> PhysAddr {
        self.io_region.paddr()
    }

    fn vaddr(&self) -> usize {
        self.io_region.vaddr()
    }
}

/// 仅实现 legacy virtio-pci I/O 端口路径，满足 x86 最小联调需求。
#[derive(Debug)]
pub struct LegacyPciTransport {
    io_region: Arc<dyn VirtIoDeviceIo>,
    config_region: LegacyPciConfigIo,
    device_type: DeviceType,
}

impl LegacyPciTransport {
    const DEVICE_FEATURES: usize = 0x00;
    const DRIVER_FEATURES: usize = 0x04;
    const QUEUE_PFN: usize = 0x08;
    const QUEUE_SIZE: usize = 0x0c;
    const QUEUE_SELECT: usize = 0x0e;
    const QUEUE_NOTIFY: usize = 0x10;
    const DEVICE_STATUS: usize = 0x12;
    const ISR_STATUS: usize = 0x13;

    pub fn new(io_region: Box<dyn VirtIoDeviceIo>, device_type: DeviceType) -> VirtIoResult<Self> {
        let io_region: Arc<dyn VirtIoDeviceIo> = io_region.into();
        let transport = Self {
            config_region: LegacyPciConfigIo {
                io_region: io_region.clone(),
            },
            io_region,
            device_type,
        };
        // 至少验证 queue0 对设备有响应。
        transport.write_u16(Self::QUEUE_SELECT, 0)?;
        let qsize = transport.read_u16(Self::QUEUE_SIZE)?;
        log::info!("[virtio-pci] queue0 size={}", qsize);
        if qsize == 0 {
            return Err(VirtIoError::NotReady);
        }
        Ok(transport)
    }

    #[inline]
    fn read_u8(&self, off: usize) -> VirtIoResult<u8> {
        self.io_region.read_volatile_u8_at(off)
    }

    #[inline]
    fn write_u8(&self, off: usize, val: u8) -> VirtIoResult<()> {
        self.io_region.write_volatile_u8_at(off, val)
    }

    #[inline]
    fn read_u16(&self, off: usize) -> VirtIoResult<u16> {
        self.io_region.read_volatile_u16_at(off)
    }

    #[inline]
    fn write_u16(&self, off: usize, val: u16) -> VirtIoResult<()> {
        self.io_region.write_volatile_u16_at(off, val)
    }

    #[inline]
    fn read_u32(&self, off: usize) -> VirtIoResult<u32> {
        self.io_region.read_volatile_u32_at(off)
    }

    #[inline]
    fn write_u32(&self, off: usize, val: u32) -> VirtIoResult<()> {
        self.io_region.write_volatile_u32_at(off, val)
    }
}

impl Transport for LegacyPciTransport {
    fn device_type(&self) -> VirtIoResult<DeviceType> {
        Ok(self.device_type)
    }

    fn read_device_features(&mut self) -> VirtIoResult<u64> {
        Ok(self.read_u32(Self::DEVICE_FEATURES)? as u64)
    }

    fn write_driver_features(&mut self, driver_features: u64) -> VirtIoResult<()> {
        if (driver_features >> 32) != 0 {
            log::warn!(
                "legacy virtio-pci ignores upper driver_features bits: {:#x}",
                driver_features >> 32
            );
        }
        self.write_u32(Self::DRIVER_FEATURES, driver_features as u32)
    }

    fn max_queue_size(&mut self, queue: u16) -> VirtIoResult<u32> {
        self.write_u16(Self::QUEUE_SELECT, queue)?;
        Ok(self.read_u16(Self::QUEUE_SIZE)? as u32)
    }

    fn notify(&mut self, queue: u16) -> VirtIoResult<()> {
        self.write_u16(Self::QUEUE_NOTIFY, queue)
    }

    fn get_status(&self) -> VirtIoResult<DeviceStatus> {
        Ok(DeviceStatus::from_bits_truncate(
            self.read_u8(Self::DEVICE_STATUS)? as u32,
        ))
    }

    fn set_status(&mut self, status: DeviceStatus) -> VirtIoResult<()> {
        self.write_u8(Self::DEVICE_STATUS, status.bits() as u8)
    }

    fn set_guest_page_size(&mut self, _guest_page_size: u32) -> VirtIoResult<()> {
        Ok(())
    }

    fn requires_legacy_layout(&self) -> bool {
        true
    }

    fn queue_set(
        &mut self,
        queue: u16,
        size: u32,
        descriptors: PhysAddr,
        driver_area: PhysAddr,
        device_area: PhysAddr,
    ) -> VirtIoResult<()> {
        assert_eq!(
            driver_area - descriptors,
            size_of::<Descriptor>() * size as usize
        );
        assert_eq!(
            device_area - descriptors,
            align_up(size_of::<Descriptor>() * size as usize + size_of::<u16>() * (size as usize + 3))
        );

        if descriptors % PAGE_SIZE != 0 {
            return Err(VirtIoError::InvalidParam);
        }
        let pfn = (descriptors / PAGE_SIZE) as u32;

        self.write_u16(Self::QUEUE_SELECT, queue)?;
        let max_q = self.read_u16(Self::QUEUE_SIZE)? as u32;
        if size > max_q {
            return Err(VirtIoError::InvalidParam);
        }
        // 兼容实现中 Queue Size 可编程；即便是只读实现，写回也不会破坏行为。
        self.write_u16(Self::QUEUE_SIZE, size as u16)?;
        self.write_u32(Self::QUEUE_PFN, pfn)
    }

    fn queue_unset(&mut self, queue: u16) -> VirtIoResult<()> {
        self.write_u16(Self::QUEUE_SELECT, queue)?;
        self.write_u32(Self::QUEUE_PFN, 0)
    }

    fn queue_used(&mut self, queue: u16) -> VirtIoResult<bool> {
        self.write_u16(Self::QUEUE_SELECT, queue)?;
        Ok(self.read_u32(Self::QUEUE_PFN)? != 0)
    }

    fn ack_interrupt(&mut self) -> VirtIoResult<bool> {
        Ok(self.read_u8(Self::ISR_STATUS)? & 0x3 != 0)
    }

    fn io_region(&self) -> &dyn VirtIoDeviceIo {
        &self.config_region
    }
}

#[derive(Debug)]
struct ModernPciConfigIo {
    device_cfg: Arc<dyn VirtIoDeviceIo>,
}

impl ModernPciConfigIo {
    #[inline]
    fn map_offset(off: usize) -> usize {
        if off >= MMIO_CONFIG_OFFSET {
            off - MMIO_CONFIG_OFFSET
        } else {
            off
        }
    }
}

impl VirtIoDeviceIo for ModernPciConfigIo {
    fn read_volatile_u32_at(&self, off: usize) -> VirtIoResult<u32> {
        self.device_cfg.read_volatile_u32_at(Self::map_offset(off))
    }

    fn read_volatile_u16_at(&self, off: usize) -> VirtIoResult<u16> {
        self.device_cfg.read_volatile_u16_at(Self::map_offset(off))
    }

    fn read_volatile_u8_at(&self, off: usize) -> VirtIoResult<u8> {
        self.device_cfg.read_volatile_u8_at(Self::map_offset(off))
    }

    fn write_volatile_u32_at(&self, off: usize, data: u32) -> VirtIoResult<()> {
        self.device_cfg
            .write_volatile_u32_at(Self::map_offset(off), data)
    }

    fn write_volatile_u16_at(&self, off: usize, data: u16) -> VirtIoResult<()> {
        self.device_cfg
            .write_volatile_u16_at(Self::map_offset(off), data)
    }

    fn write_volatile_u8_at(&self, off: usize, data: u8) -> VirtIoResult<()> {
        self.device_cfg
            .write_volatile_u8_at(Self::map_offset(off), data)
    }

    fn paddr(&self) -> PhysAddr {
        self.device_cfg.paddr()
    }

    fn vaddr(&self) -> usize {
        self.device_cfg.vaddr()
    }
}

/// modern virtio-pci MMIO capability 路径。
#[derive(Debug)]
pub struct ModernPciTransport {
    common_cfg: Arc<dyn VirtIoDeviceIo>,
    notify_cfg: Arc<dyn VirtIoDeviceIo>,
    isr_cfg: Arc<dyn VirtIoDeviceIo>,
    config_region: ModernPciConfigIo,
    notify_off_multiplier: u32,
    device_type: DeviceType,
}

impl ModernPciTransport {
    const DEVICE_FEATURE_SELECT: usize = 0x00;
    const DEVICE_FEATURE: usize = 0x04;
    const DRIVER_FEATURE_SELECT: usize = 0x08;
    const DRIVER_FEATURE: usize = 0x0c;
    const DEVICE_STATUS: usize = 0x14;
    const QUEUE_SELECT: usize = 0x16;
    const QUEUE_SIZE: usize = 0x18;
    const QUEUE_ENABLE: usize = 0x1c;
    const QUEUE_NOTIFY_OFF: usize = 0x1e;
    const QUEUE_DESC: usize = 0x20;
    const QUEUE_DRIVER: usize = 0x28;
    const QUEUE_DEVICE: usize = 0x30;

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        common_cfg: Box<dyn VirtIoDeviceIo>,
        notify_cfg: Box<dyn VirtIoDeviceIo>,
        isr_cfg: Box<dyn VirtIoDeviceIo>,
        device_cfg: Box<dyn VirtIoDeviceIo>,
        notify_off_multiplier: u32,
        device_type: DeviceType,
    ) -> VirtIoResult<Self> {
        let common_cfg: Arc<dyn VirtIoDeviceIo> = common_cfg.into();
        let transport = Self {
            notify_cfg: notify_cfg.into(),
            isr_cfg: isr_cfg.into(),
            config_region: ModernPciConfigIo {
                device_cfg: device_cfg.into(),
            },
            notify_off_multiplier,
            device_type,
            common_cfg,
        };
        transport.write_common_u16(Self::QUEUE_SELECT, 0)?;
        let qsize = transport.read_common_u16(Self::QUEUE_SIZE)?;
        log::info!("[virtio-pci] queue0 size={}", qsize);
        if qsize == 0 {
            return Err(VirtIoError::NotReady);
        }
        Ok(transport)
    }

    #[inline]
    fn read_common_u8(&self, off: usize) -> VirtIoResult<u8> {
        self.common_cfg.read_volatile_u8_at(off)
    }

    #[inline]
    fn write_common_u8(&self, off: usize, val: u8) -> VirtIoResult<()> {
        self.common_cfg.write_volatile_u8_at(off, val)
    }

    #[inline]
    fn read_common_u16(&self, off: usize) -> VirtIoResult<u16> {
        self.common_cfg.read_volatile_u16_at(off)
    }

    #[inline]
    fn write_common_u16(&self, off: usize, val: u16) -> VirtIoResult<()> {
        self.common_cfg.write_volatile_u16_at(off, val)
    }

    #[inline]
    fn read_common_u32(&self, off: usize) -> VirtIoResult<u32> {
        self.common_cfg.read_volatile_u32_at(off)
    }

    #[inline]
    fn write_common_u32(&self, off: usize, val: u32) -> VirtIoResult<()> {
        self.common_cfg.write_volatile_u32_at(off, val)
    }

    #[inline]
    fn write_common_u64(&self, off: usize, val: u64) -> VirtIoResult<()> {
        self.write_common_u32(off, val as u32)?;
        self.write_common_u32(off + 4, (val >> 32) as u32)
    }
}

impl Transport for ModernPciTransport {
    fn device_type(&self) -> VirtIoResult<DeviceType> {
        Ok(self.device_type)
    }

    fn read_device_features(&mut self) -> VirtIoResult<u64> {
        self.write_common_u32(Self::DEVICE_FEATURE_SELECT, 0)?;
        let low = self.read_common_u32(Self::DEVICE_FEATURE)? as u64;
        self.write_common_u32(Self::DEVICE_FEATURE_SELECT, 1)?;
        let high = self.read_common_u32(Self::DEVICE_FEATURE)? as u64;
        Ok(low | (high << 32))
    }

    fn write_driver_features(&mut self, driver_features: u64) -> VirtIoResult<()> {
        self.write_common_u32(Self::DRIVER_FEATURE_SELECT, 0)?;
        self.write_common_u32(Self::DRIVER_FEATURE, driver_features as u32)?;
        self.write_common_u32(Self::DRIVER_FEATURE_SELECT, 1)?;
        self.write_common_u32(Self::DRIVER_FEATURE, (driver_features >> 32) as u32)
    }

    fn max_queue_size(&mut self, queue: u16) -> VirtIoResult<u32> {
        self.write_common_u16(Self::QUEUE_SELECT, queue)?;
        Ok(self.read_common_u16(Self::QUEUE_SIZE)? as u32)
    }

    fn notify(&mut self, queue: u16) -> VirtIoResult<()> {
        self.write_common_u16(Self::QUEUE_SELECT, queue)?;
        let notify_off = self.read_common_u16(Self::QUEUE_NOTIFY_OFF)? as usize;
        let byte_off = notify_off
            .checked_mul(self.notify_off_multiplier as usize)
            .ok_or(VirtIoError::InvalidParam)?;
        self.notify_cfg.write_volatile_u8_at(byte_off, (queue & 0xff) as u8)?;
        self.notify_cfg
            .write_volatile_u8_at(byte_off + 1, (queue >> 8) as u8)
    }

    fn get_status(&self) -> VirtIoResult<DeviceStatus> {
        Ok(DeviceStatus::from_bits_truncate(
            self.read_common_u8(Self::DEVICE_STATUS)? as u32,
        ))
    }

    fn set_status(&mut self, status: DeviceStatus) -> VirtIoResult<()> {
        self.write_common_u8(Self::DEVICE_STATUS, status.bits() as u8)
    }

    fn set_guest_page_size(&mut self, _guest_page_size: u32) -> VirtIoResult<()> {
        Ok(())
    }

    fn requires_legacy_layout(&self) -> bool {
        false
    }

    fn queue_set(
        &mut self,
        queue: u16,
        size: u32,
        descriptors: PhysAddr,
        driver_area: PhysAddr,
        device_area: PhysAddr,
    ) -> VirtIoResult<()> {
        if size > u16::MAX as u32 {
            return Err(VirtIoError::InvalidParam);
        }
        self.write_common_u16(Self::QUEUE_SELECT, queue)?;
        self.write_common_u16(Self::QUEUE_SIZE, size as u16)?;
        self.write_common_u64(Self::QUEUE_DESC, descriptors as u64)?;
        self.write_common_u64(Self::QUEUE_DRIVER, driver_area as u64)?;
        self.write_common_u64(Self::QUEUE_DEVICE, device_area as u64)?;
        self.write_common_u16(Self::QUEUE_ENABLE, 1)
    }

    fn queue_unset(&mut self, queue: u16) -> VirtIoResult<()> {
        self.write_common_u16(Self::QUEUE_SELECT, queue)?;
        self.write_common_u16(Self::QUEUE_ENABLE, 0)?;
        self.write_common_u64(Self::QUEUE_DESC, 0)?;
        self.write_common_u64(Self::QUEUE_DRIVER, 0)?;
        self.write_common_u64(Self::QUEUE_DEVICE, 0)
    }

    fn queue_used(&mut self, queue: u16) -> VirtIoResult<bool> {
        self.write_common_u16(Self::QUEUE_SELECT, queue)?;
        Ok(self.read_common_u16(Self::QUEUE_ENABLE)? != 0)
    }

    fn ack_interrupt(&mut self) -> VirtIoResult<bool> {
        Ok(self.isr_cfg.read_volatile_u8_at(0)? & 0x3 != 0)
    }

    fn io_region(&self) -> &dyn VirtIoDeviceIo {
        &self.config_region
    }
}

impl Drop for LegacyPciTransport {
    fn drop(&mut self) {
        let _ = self.set_status(DeviceStatus::empty());
    }
}

impl Drop for ModernPciTransport {
    fn drop(&mut self) {
        let _ = self.set_status(DeviceStatus::empty());
    }
}
