use alloc::vec;

use ::fdt::Fdt;

use crate::{
    bus::{
        CommonDeviceInfo, DeviceClass, DeviceLocator, DeviceTransport, DiscoveredDevice,
        FirmwareSource,
    },
    error::AlienResult,
};

use self::fdt::Probe;

pub mod fdt;
pub mod mmio;
pub mod pci;
pub mod platform;

#[derive(Debug, Clone)]
pub enum CommonDeviceType {
    Plic(CommonDeviceInfo),
    Uart(CommonDeviceInfo),
    Rtc(CommonDeviceInfo),
    VirtIo(CommonDeviceInfo),
    Pci(CommonDeviceInfo),
    #[cfg(any(feature = "bench", all(plat_vf2, not(plat_vf2_sd))))]
    Ramdisk(CommonDeviceInfo),
    #[cfg(plat_vf2)]
    LoopBack(CommonDeviceInfo),
    #[cfg(all(plat_vf2, plat_vf2_sd))]
    SdCard(CommonDeviceInfo),
}

fn from_common_device(ty: CommonDeviceType) -> DiscoveredDevice {
    match ty {
        CommonDeviceType::Plic(info) => DiscoveredDevice {
            class: DeviceClass::Plic,
            locator: DeviceLocator::Mmio(info.address_range),
            transport: DeviceTransport::Platform,
            irq: info.irq,
            compatible: info.compatible,
            fw_source: FirmwareSource::Fdt,
        },
        CommonDeviceType::Uart(info) => DiscoveredDevice {
            class: DeviceClass::Uart,
            locator: DeviceLocator::Mmio(info.address_range),
            transport: DeviceTransport::Platform,
            irq: info.irq,
            compatible: info.compatible,
            fw_source: FirmwareSource::Fdt,
        },
        CommonDeviceType::Rtc(info) => DiscoveredDevice {
            class: DeviceClass::Rtc,
            locator: DeviceLocator::Mmio(info.address_range),
            transport: DeviceTransport::Platform,
            irq: info.irq,
            compatible: info.compatible,
            fw_source: FirmwareSource::Fdt,
        },
        CommonDeviceType::VirtIo(info) => DiscoveredDevice {
            class: DeviceClass::VirtioMmio,
            locator: DeviceLocator::Mmio(info.address_range),
            transport: DeviceTransport::Mmio,
            irq: info.irq,
            compatible: info.compatible,
            fw_source: FirmwareSource::Fdt,
        },
        CommonDeviceType::Pci(info) => DiscoveredDevice {
            class: DeviceClass::PciHost,
            locator: DeviceLocator::Mmio(info.address_range),
            transport: DeviceTransport::Pci,
            irq: info.irq,
            compatible: info.compatible,
            fw_source: FirmwareSource::Fdt,
        },
        #[cfg(any(feature = "bench", all(plat_vf2, not(plat_vf2_sd))))]
        CommonDeviceType::Ramdisk(info) => DiscoveredDevice {
            class: DeviceClass::Ramdisk,
            locator: DeviceLocator::Mmio(info.address_range),
            transport: DeviceTransport::Platform,
            irq: info.irq,
            compatible: info.compatible,
            fw_source: FirmwareSource::Synthetic,
        },
        #[cfg(plat_vf2)]
        CommonDeviceType::LoopBack(info) => DiscoveredDevice {
            class: DeviceClass::LoopBack,
            locator: DeviceLocator::Mmio(info.address_range),
            transport: DeviceTransport::Platform,
            irq: info.irq,
            compatible: info.compatible,
            fw_source: FirmwareSource::Synthetic,
        },
        #[cfg(all(plat_vf2, plat_vf2_sd))]
        CommonDeviceType::SdCard(info) => DiscoveredDevice {
            class: DeviceClass::SdCard,
            locator: DeviceLocator::Mmio(info.address_range),
            transport: DeviceTransport::Platform,
            irq: info.irq,
            compatible: info.compatible,
            fw_source: FirmwareSource::Fdt,
        },
    }
}

fn locator_to_info(locator: &DeviceLocator, irq: Option<u32>, compatible: Option<alloc::string::String>) -> Option<CommonDeviceInfo> {
    match locator {
        DeviceLocator::Mmio(range) => Some(CommonDeviceInfo {
            address_range: range.clone(),
            irq,
            compatible,
        }),
        _ => None,
    }
}

fn register_discovered_devices(devices: alloc::vec::Vec<DiscoveredDevice>) {
    devices.into_iter().for_each(|dev| match dev.class {
        DeviceClass::Plic => {
            if let Some(info) = locator_to_info(&dev.locator, dev.irq, dev.compatible) {
                platform::register_platform_device(info, "plic");
            }
        }
        DeviceClass::Uart => {
            if let Some(info) = locator_to_info(&dev.locator, dev.irq, dev.compatible) {
                platform::register_platform_device(info, "uart");
            }
        }
        DeviceClass::Rtc => {
            if let Some(info) = locator_to_info(&dev.locator, dev.irq, dev.compatible) {
                platform::register_platform_device(info, "rtc");
            }
        }
        DeviceClass::VirtioMmio => {
            if let Some(info) = locator_to_info(&dev.locator, dev.irq, dev.compatible) {
                mmio::register_mmio_device(info);
            }
        }
        DeviceClass::PciHost => {
            if let Some(info) = locator_to_info(&dev.locator, dev.irq, dev.compatible) {
                pci::pci_init(info);
            }
        }
        #[cfg(any(feature = "bench", all(plat_vf2, not(plat_vf2_sd))))]
        DeviceClass::Ramdisk => {
            if let Some(info) = locator_to_info(&dev.locator, dev.irq, dev.compatible) {
                platform::register_platform_device(info, "ramdisk");
            }
        }
        #[cfg(plat_vf2)]
        DeviceClass::LoopBack => {
            if let Some(info) = locator_to_info(&dev.locator, dev.irq, dev.compatible) {
                platform::register_platform_device(info, "loopback");
            }
        }
        #[cfg(all(plat_vf2, plat_vf2_sd))]
        DeviceClass::SdCard => {
            if let Some(info) = locator_to_info(&dev.locator, dev.irq, dev.compatible) {
                platform::register_platform_device(info, "sdcard");
            }
        }
        _ => {}
    });
}

pub fn init_with_dtb() -> AlienResult<()> {
    let ptr = ::platform::platform_boot_info_ptr();
    let dtb = unsafe { Fdt::from_ptr(ptr as *const u8) }.unwrap();

    let mut devices = vec![];
    if let Some(ty) = dtb.probe_rtc() {
        devices.push(ty);
    }
    if let Some(ty) = dtb.probe_uart() {
        devices.push(ty);
    }
    if let Some(ty) = dtb.probe_plic() {
        devices.push(ty);
    }
    if let Some(ty) = dtb.probe_pci() {
        devices.push(ty);
    }
    let virtio = dtb.probe_virtio();
    if let Some(virtio) = virtio {
        for ty in virtio {
            devices.push(ty);
        }
    }

    #[cfg(feature = "bench")]
    {
        let ramdisk_start = RAMDISK.as_ptr() as usize;
        let len = RAMDISK.len();
        let info = CommonDeviceInfo {
            address_range: mem::PhysAddr::from(ramdisk_start)..mem::PhysAddr::from(ramdisk_start + len),
            irq: None,
            compatible: None,
        };
        devices.push(CommonDeviceType::Ramdisk(info));
    }

    #[cfg(plat_vf2)]
    {
        #[cfg(not(plat_vf2_sd))]
        {
            let ramdisk_start = RAMDISK.as_ptr() as usize;
            let len = RAMDISK.len();
            let info = CommonDeviceInfo {
                address_range: mem::PhysAddr::from(ramdisk_start)
                    ..mem::PhysAddr::from(ramdisk_start + len),
                irq: None,
                compatible: None,
            };
            devices.push(CommonDeviceType::Ramdisk(info));
        }

        let fake_nic = CommonDeviceInfo {
            address_range: mem::PhysAddr::from(0)..mem::PhysAddr::from(0),
            irq: Some(0),
            compatible: None,
        };
        devices.push(CommonDeviceType::LoopBack(fake_nic));
    }

    #[cfg(all(plat_vf2, plat_vf2_sd))]
    dtb.probe_sd().map(|ty| {
        devices.push(ty);
    });

    let discovered = devices.into_iter().map(from_common_device).collect();
    register_discovered_devices(discovered);
    Ok(())
}

#[cfg(feature = "bench")]
static RAMDISK: &'static [u8] = &[0u8; 1024];
#[cfg(all(plat_vf2, not(plat_vf2_sd)))]
static RAMDISK: &'static [u8] = include_bytes!("../../../../build/sdcard.img");
