use acpi::{
    AcpiTables, HpetInfo,
    sdt::madt::{Madt, MadtEntry},
};

use super::{AcpiDeviceEntry, AcpiDeviceInfo, AcpiHost};

pub(super) fn detect_static_acpi_info(tables: Option<&AcpiTables<AcpiHost>>) -> AcpiDeviceInfo {
    let mut info = AcpiDeviceInfo::default();

    if let Some(tables) = tables {
        // MADT 提供 LAPIC/IOAPIC 的真实地址；如果有地址覆盖项，则以覆盖项为准。
        if let Some(madt) = tables.find_table::<Madt>() {
            let madt = madt.get();
            info.lapic_base = madt.local_apic_address as usize;

            for entry in madt.entries() {
                match entry {
                    MadtEntry::IoApic(ioapic) => {
                        info.ioapic_base = ioapic.io_apic_address as usize;
                    }
                    MadtEntry::LocalApicAddressOverride(override_entry) => {
                        info.lapic_base = override_entry.local_apic_address as usize;
                    }
                    _ => {}
                }
            }
        }

        // HPET 单独由 HPET 表给出，和 MADT 不在同一张表里。
        if let Ok(hpet) = HpetInfo::new(tables) {
            info.hpet_base = Some(hpet.base_address);
        }
    } else {
        // 只有在 ACPI 表不可用时，才退回到固定常量。
        info.lapic_base = crate::qemu_x86_64::config::DEVICE_SPACE[0].1;
        info.ioapic_base = crate::qemu_x86_64::config::DEVICE_SPACE[1].1;
        info.hpet_base = Some(crate::qemu_x86_64::config::DEVICE_SPACE[2].1);
    }

    let _ = info.devices.entries.push(AcpiDeviceEntry {
        name: "local_apic",
        base: info.lapic_base,
        size: 0x1000,
    });
    let _ = info.devices.entries.push(AcpiDeviceEntry {
        name: "io_apic",
        base: info.ioapic_base,
        size: 0x1000,
    });

    if let Some(hpet_base) = info.hpet_base {
        // 把静态可见的核心设备都写进列表，后续 bus/domain 只消费结果。
        let _ = info.devices.entries.push(AcpiDeviceEntry {
            name: "hpet",
            base: hpet_base,
            size: 0x1000,
        });
    }

    log::info!(
        "ACPI static info: LAPIC={:#x}, IOAPIC={:#x}, HPET={:#x?}, devices={}",
        info.lapic_base,
        info.ioapic_base,
        info.hpet_base,
        info.devices.entries.len()
    );

    info
}
