use alloc::vec;

use ::fdt::Fdt;

use crate::{
    bus::CommonDeviceInfo,
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

fn register_detected_devices(devices: alloc::vec::Vec<CommonDeviceType>) {
    devices.into_iter().for_each(|ty| match ty {
        CommonDeviceType::Plic(info) => platform::register_platform_device(info, "plic"),
        CommonDeviceType::Uart(info) => platform::register_platform_device(info, "uart"),
        CommonDeviceType::Rtc(info) => platform::register_platform_device(info, "rtc"),
        CommonDeviceType::VirtIo(info) => mmio::register_mmio_device(info),
        CommonDeviceType::Pci(info) => pci::pci_init(info),
        #[cfg(any(feature = "bench", all(plat_vf2, not(plat_vf2_sd))))]
        CommonDeviceType::Ramdisk(info) => platform::register_platform_device(info, "ramdisk"),
        #[cfg(plat_vf2)]
        CommonDeviceType::LoopBack(info) => platform::register_platform_device(info, "loopback"),
        #[cfg(all(plat_vf2, plat_vf2_sd))]
        CommonDeviceType::SdCard(info) => platform::register_platform_device(info, "sdcard"),
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

    register_detected_devices(devices);
    Ok(())
}

#[cfg(feature = "bench")]
static RAMDISK: &'static [u8] = &[0u8; 1024];
#[cfg(all(plat_vf2, not(plat_vf2_sd)))]
static RAMDISK: &'static [u8] = include_bytes!("../../../../build/sdcard.img");
