extern crate alloc;

use alloc::boxed::Box;
use core::{mem::size_of, slice};

use acpi::{
    AcpiTables, HpetInfo,
    rsdp::Rsdp,
    sdt::{SdtHeader, madt::{Madt, MadtEntry}},
};
use aml::{AmlContext, DebugVerbosity};

use super::{AcpiDeviceEntry, AcpiDeviceInfo, acpi_handler::AcpiHost, aml_handler::AmlHost, support::phys_to_virt};

pub(super) fn detect_acpi_info() -> AcpiDeviceInfo {
    let mut info = AcpiDeviceInfo::default();
    let host = AcpiHost;

    let rsdp = match unsafe { Rsdp::search_for_on_bios(host) } {
        Ok(rsdp) => rsdp,
        Err(e) => {
            log::warn!("ACPI RSDP not found ({:?}), using default APIC addresses", e);
            return info;
        }
    };

    let tables = match unsafe { AcpiTables::from_rsdp(host, rsdp.physical_start) } {
        Ok(tables) => tables,
        Err(e) => {
            log::warn!("Failed to parse ACPI tables ({:?}), using defaults", e);
            return info;
        }
    };

    fill_madt_info(&tables, &mut info);
    fill_hpet_info(&tables, &mut info);
    fill_pci_info(&tables, &mut info);

    let aml_loaded = load_aml_tables(&tables);

    log::info!(
        "ACPI device info: LAPIC={:#x}, IOAPIC={:#x}, HPET={:#x?}, AML tables loaded={}, devices={}",
        info.lapic_base,
        info.ioapic_base,
        info.hpet_base,
        aml_loaded,
        info.devices.entries.len()
    );

    info
}

fn fill_madt_info(tables: &AcpiTables<AcpiHost>, info: &mut AcpiDeviceInfo) {
    if let Some(madt) = tables.find_table::<Madt>() {
        let madt = madt.get();
        info.lapic_base = madt.local_apic_address as usize;

        for entry in madt.entries() {
            match entry {
                MadtEntry::IoApic(ioapic) => info.ioapic_base = ioapic.io_apic_address as usize,
                MadtEntry::LocalApicAddressOverride(ovr) => {
                    info.lapic_base = ovr.local_apic_address as usize;
                }
                _ => {}
            }
        }
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
}

fn fill_hpet_info(tables: &AcpiTables<AcpiHost>, info: &mut AcpiDeviceInfo) {
    if let Ok(hpet) = HpetInfo::new(tables) {
        info.hpet_base = Some(hpet.base_address);
        let _ = info.devices.entries.push(AcpiDeviceEntry {
            name: "hpet",
            base: hpet.base_address,
            size: 0x1000,
        });
    }
}

fn fill_pci_info(tables: &AcpiTables<AcpiHost>, info: &mut AcpiDeviceInfo) {
    if let Ok(pci_regions) = acpi::platform::PciConfigRegions::new(tables) {
        for region in pci_regions.regions.iter() {
            let bus_count = (region.bus_number_end - region.bus_number_start) as usize + 1;
            let size = bus_count << 20;
            let _ = info.devices.entries.push(AcpiDeviceEntry {
                name: "pci_ecam",
                base: region.base_address as usize,
                size,
            });
        }
    }
}

fn load_aml_tables(tables: &AcpiTables<AcpiHost>) -> usize {
    let mut aml_ctx = AmlContext::new(Box::new(AmlHost), DebugVerbosity::None);
    let mut aml_loaded = 0usize;

    if let Ok(dsdt) = tables.dsdt()
        && load_aml_table(&mut aml_ctx, dsdt).is_ok()
    {
        aml_loaded += 1;
    }

    for ssdt in tables.ssdts() {
        if load_aml_table(&mut aml_ctx, ssdt).is_ok() {
            aml_loaded += 1;
        }
    }

    aml_loaded
}

fn load_aml_table(ctx: &mut AmlContext, table: acpi::AmlTable) -> Result<(), aml::AmlError> {
    let raw = unsafe {
        slice::from_raw_parts(phys_to_virt(table.phys_address) as *const u8, table.length as usize)
    };

    if raw.len() <= size_of::<SdtHeader>() {
        return Ok(());
    }

    let aml_stream = &raw[size_of::<SdtHeader>()..];
    ctx.parse_table(aml_stream)
}