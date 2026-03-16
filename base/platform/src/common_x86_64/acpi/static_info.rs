use super::{AcpiDeviceEntry, AcpiDeviceInfo};

pub(super) fn detect_static_acpi_info() -> AcpiDeviceInfo {
    let mut info = AcpiDeviceInfo::default();

    for &(name, base, size) in crate::qemu_x86_64::config::DEVICE_SPACE {
        let _ = info.devices.entries.push(AcpiDeviceEntry { name, base, size });
        match name {
            "local_apic" => info.lapic_base = base,
            "io_apic" => info.ioapic_base = base,
            "hpet" => info.hpet_base = Some(base),
            _ => {}
        }
    }

    log::info!(
        "ACPI static device info: LAPIC={:#x}, IOAPIC={:#x}, HPET={:#x?}, devices={}",
        info.lapic_base,
        info.ioapic_base,
        info.hpet_base,
        info.devices.entries.len()
    );

    info
}