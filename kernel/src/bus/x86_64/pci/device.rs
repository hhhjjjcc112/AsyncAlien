use acpi::PciAddress;
use alloc::collections::VecDeque;
use core::ops::Range;

use basic::io::SafeIORegion;
use mem::PhysAddr;

use crate::bus::CommonDeviceInfo;

pub struct PciBus {
    /// 所有已发现的 PCI ECAM 区域。
    common_devices: VecDeque<PciCommonDevice>,
    /// 从 ECAM 区域里扫描出来的具体 PCI function。
    endpoint_devices: VecDeque<PciEndpointDevice>,
}

#[derive(Debug, Clone, Copy)]
pub struct PciEndpointDevice {
    /// 端点在 PCI 拓扑里的位置，用 BDF 标识。
    address: PciAddress,
    /// 设备厂商 ID。
    vendor_id: u16,
    /// 设备 ID。
    device_id: u16,
    /// class / subclass / prog_if / revision 四个字段打包后的原始值。
    class_revision: u32,
    /// 头类型，决定这是普通设备、桥设备还是其他类型。
    header_type: u8,
}

#[derive(Debug)]
pub struct PciCommonDevice {
    io_region: SafeIORegion,
    info: CommonDeviceInfo,
}

impl PciBus {
    pub(super) const fn new() -> Self {
        Self {
            common_devices: VecDeque::new(),
            endpoint_devices: VecDeque::new(),
        }
    }

    /// 注册一个 ECAM 区域时，顺手扫描其中的所有 endpoint。
    /// 这样上层既能拿到“区域”，也能拿到“具体设备”。
    pub(super) fn register_common_device(&mut self, device: PciCommonDevice) {
        let mut endpoints = device.scan_endpoints();
        #[cfg(target_arch = "x86_64")]
        if endpoints.is_empty() {
            // 某些平台（如传统 i440fx）没有可用 ECAM，这里回退到 CF8/CFC 机制。
            endpoints = legacy_scan_endpoints();
            if !endpoints.is_empty() {
                println!(
                    "[bus][x86_64][pci] ECAM empty, fallback CF8/CFC found {} endpoint(s)",
                    endpoints.len()
                );
            }
        }
        if !endpoints.is_empty() {
            info!("[PciBus]: discovered {} PCI endpoint(s)", endpoints.len());
        }
        self.endpoint_devices.extend(endpoints);
        self.common_devices.push_back(device);
    }

/// 函数说明：执行对应的总线处理步骤。
    pub fn register_driver(&mut self) {
        // self.drivers.push(driver);
    }

/// 函数说明：执行对应的总线处理步骤。
    pub fn common_devices(&self) -> &VecDeque<PciCommonDevice> {
        &self.common_devices
    }

/// 函数说明：执行对应的总线处理步骤。
    pub fn endpoint_devices(&self) -> &VecDeque<PciEndpointDevice> {
        &self.endpoint_devices
    }
}

#[cfg(target_arch = "x86_64")]
/// 函数说明：执行对应的总线处理步骤。
fn legacy_cfg_address(address: PciAddress, offset: u16) -> u32 {
    (1u32 << 31)
        | ((address.bus() as u32) << 16)
        | ((address.device() as u32) << 11)
        | ((address.function() as u32) << 8)
        | ((offset as u32) & 0xfc)
}

#[cfg(target_arch = "x86_64")]
/// 函数说明：执行对应的总线处理步骤。
fn legacy_cfg_read32(address: PciAddress, offset: u16) -> u32 {
    unsafe {
        x86::io::outl(0xcf8, legacy_cfg_address(address, offset));
        x86::io::inl(0xcfc)
    }
}

#[cfg(target_arch = "x86_64")]
/// 函数说明：执行对应的总线处理步骤。
fn legacy_cfg_read16(address: PciAddress, offset: u16) -> u16 {
    let aligned = offset & !0x3;
    let v = legacy_cfg_read32(address, aligned);
    ((v >> ((offset & 0x2) * 8)) & 0xffff) as u16
}

#[cfg(target_arch = "x86_64")]
/// 函数说明：执行对应的总线处理步骤。
fn legacy_cfg_read8(address: PciAddress, offset: u16) -> u8 {
    let aligned = offset & !0x3;
    let v = legacy_cfg_read32(address, aligned);
    ((v >> ((offset & 0x3) * 8)) & 0xff) as u8
}

#[cfg(target_arch = "x86_64")]
/// 函数说明：执行对应的总线处理步骤。
fn legacy_scan_endpoints() -> VecDeque<PciEndpointDevice> {
    let mut endpoints = VecDeque::new();

    for bus in 0..=0xffu8 {
        for device in 0..32u8 {
            let addr0 = PciAddress::new(0, bus, device, 0);
            let vendor0 = legacy_cfg_read16(addr0, 0x00);
            if vendor0 == 0xffff {
                continue;
            }

            let header0 = legacy_cfg_read8(addr0, 0x0e);
            let multifunction = header0 & 0x80 != 0;

            for function in 0..8u8 {
                if function != 0 && !multifunction {
                    break;
                }

                let addr = PciAddress::new(0, bus, device, function);
                let vendor = legacy_cfg_read16(addr, 0x00);
                if vendor == 0xffff {
                    continue;
                }

                let device_id = legacy_cfg_read16(addr, 0x02);
                let class_revision = legacy_cfg_read32(addr, 0x08);
                let header_type = legacy_cfg_read8(addr, 0x0e) & 0x7f;

                endpoints.push_back(PciEndpointDevice {
                    address: addr,
                    vendor_id: vendor,
                    device_id,
                    class_revision,
                    header_type,
                });
            }
        }
    }

    endpoints
}

impl PciCommonDevice {
    /// 新建一个 PCI ECAM 区域对象。
    /// `SafeIORegion` 提供边界检查后的 MMIO 访问，避免上层直接操作裸指针。
    pub(super) fn new(io_region: SafeIORegion, info: CommonDeviceInfo) -> Self {
        let res = Self { io_region, info };
        info!(
            "[PciCommonDevice]: Found PCI ECAM region, addr: {:#x?}",
            res.address_range()
        );
        res
    }

/// 函数说明：执行对应的总线处理步骤。
    pub fn address(&self) -> PhysAddr {
        self.io_region.phys_addr()
    }

/// 函数说明：执行对应的总线处理步骤。
    pub fn address_range(&self) -> Range<PhysAddr> {
        self.io_region.phys_addr_range()
    }

/// 函数说明：执行对应的总线处理步骤。
    pub fn io_region(&self) -> &SafeIORegion {
        &self.io_region
    }

/// 函数说明：执行对应的总线处理步骤。
    pub fn irq(&self) -> Option<u32> {
        self.info.irq
    }

/// 函数说明：执行对应的总线处理步骤。
    pub fn compatible(&self) -> Option<&str> {
        self.info.compatible.as_deref()
    }

    /// 一个 ECAM 区域通常对应多个 bus，每个 bus 预留 1 MiB 配置空间。
    fn bus_count(&self) -> usize {
        self.io_region.size() >> 20
    }

    /// ECAM 的地址偏移规则：bus / device / function / register 各占固定位段。
    fn config_offset(bus: u8, device: u8, function: u8, register_offset: usize) -> usize {
        ((bus as usize) << 20)
            | ((device as usize) << 15)
            | ((function as usize) << 12)
            | register_offset
    }

    /// 读 8 位配置寄存器。
    fn read_config_u8(&self, bus: u8, device: u8, function: u8, register_offset: usize) -> Option<u8> {
        self.io_region
            .read_at::<u8>(Self::config_offset(bus, device, function, register_offset))
            .ok()
    }

    /// 读 16 位配置寄存器。
    fn read_config_u16(&self, bus: u8, device: u8, function: u8, register_offset: usize) -> Option<u16> {
        self.io_region
            .read_at::<u16>(Self::config_offset(bus, device, function, register_offset))
            .ok()
    }

    /// 读 32 位配置寄存器。
    fn read_config_u32(&self, bus: u8, device: u8, function: u8, register_offset: usize) -> Option<u32> {
        self.io_region
            .read_at::<u32>(Self::config_offset(bus, device, function, register_offset))
            .ok()
    }

    /// 扫描整个 ECAM 区域，提取有响应的 PCI endpoint。
    ///
    /// 扫描顺序是 bus -> device -> function：
    /// - 先看 bus 数量，避免越过当前 ECAM 覆盖范围；
    /// - 再扫 device 0..31；
    /// - 最后根据 multifunction 位决定是否继续扫 function 1..7。
    fn scan_endpoints(&self) -> VecDeque<PciEndpointDevice> {
        let mut endpoints = VecDeque::new();

        // 目前先按 ECAM 区域大小推导可访问的总线数，QEMU/常见 ACPI 场景下足够使用。
        for bus in 0..self.bus_count() {
            let Ok(bus) = u8::try_from(bus) else {
                break;
            };

            for device in 0..32u8 {
                // function 0 先读 vendor_id；如果这里就是 0xffff，说明这个 device 位置不存在。
                let Some(vendor_id) = self.read_config_u16(bus, device, 0, 0x00) else {
                    continue;
                };

                if vendor_id == 0xffff || vendor_id == 0x0000 {
                    continue;
                }

                // header_type 的 bit7 表示 multifunction。只有置位时才需要继续扫 function 1..7。
                let header_type = self.read_config_u8(bus, device, 0, 0x0e).unwrap_or(0);
                let multifunction = header_type & 0x80 != 0;

                for function in 0..8u8 {
                    if function != 0 && !multifunction {
                        break;
                    }

                    // 再次确认当前 function 是否真的存在，避免把未实现的功能号误判成设备。
                    let Some(vendor_id) = self.read_config_u16(bus, device, function, 0x00) else {
                        continue;
                    };

                    if vendor_id == 0xffff || vendor_id == 0x0000 {
                        continue;
                    }

                    // device_id 用来区分同厂商的不同硬件型号。
                    let Some(device_id) = self.read_config_u16(bus, device, function, 0x02) else {
                        continue;
                    };
                    // class_revision 里打包了 class / subclass / prog_if / revision。
                    let Some(class_revision) = self.read_config_u32(bus, device, function, 0x08) else {
                        continue;
                    };
                    // header_type 再读一次，去掉 multifunction 位后保留真实头类型。
                    let header_type = self.read_config_u8(bus, device, function, 0x0e).unwrap_or(0) & 0x7f;

                    endpoints.push_back(PciEndpointDevice {
                        // 这里把 BDF 保存下来，后续驱动可以直接定位这个 function。
                        address: PciAddress::new(0, bus, device, function),
                        vendor_id,
                        device_id,
                        class_revision,
                        header_type,
                    });
                }
            }
        }

        endpoints
    }
}

impl PciEndpointDevice {
    /// 端点的 PCI 地址。
    pub fn address(&self) -> PciAddress {
        self.address
    }

    /// 厂商 ID。
    pub fn vendor_id(&self) -> u16 {
        self.vendor_id
    }

    /// 设备 ID。
    pub fn device_id(&self) -> u16 {
        self.device_id
    }

    /// class code，表示这是什么类别的 PCI 设备。
    pub fn class_code(&self) -> u8 {
        (self.class_revision >> 24) as u8
    }

    /// subclass，表示类别下面更细的分组。
    pub fn subclass(&self) -> u8 {
        (self.class_revision >> 16) as u8
    }

    /// program interface，表示同一 subclass 下的具体接口差异。
    pub fn prog_if(&self) -> u8 {
        (self.class_revision >> 8) as u8
    }

    /// revision id，表示硬件修订版本。
    pub fn revision_id(&self) -> u8 {
        self.class_revision as u8
    }

    /// 设备头类型，用来判断这是普通 endpoint 还是桥设备。
    pub fn header_type(&self) -> u8 {
        self.header_type
    }

    /// 当前阶段只关心 virtio 的 block/net/input 三类设备。
    pub fn virtio_kind(&self) -> Option<&'static str> {
        // Virtio PCI vendor id 固定为 0x1af4。
        if self.vendor_id != 0x1af4 {
            return None;
        }

        // 现代 virtio-pci：0x1040 + device_type。
        if self.device_id >= 0x1040 {
            let ty = self.device_id - 0x1040;
            return match ty {
                1 => Some("virtio-net"),
                2 => Some("virtio-blk"),
                18 => Some("virtio-input"),
                _ => None,
            };
        }

        // 兼容旧式（transitional）设备号，至少覆盖 net/blk/input。
        match self.device_id {
            0x1000 => Some("virtio-net"),
            0x1001 => Some("virtio-blk"),
            0x1012 => Some("virtio-input"),
            _ => None,
        }
    }
}
