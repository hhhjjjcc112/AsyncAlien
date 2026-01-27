use core::ptr::{read_volatile, write_volatile};
use acpi::{AcpiHandler, PhysicalMapping, AcpiTables};
use aml::{AmlContext, AmlTable, AmlValue, DebugVerbosity};

#[derive(Clone, Copy)]
pub struct HpetInfo {
    pub base_address: usize,
    pub period: u32,
}

impl HpetInfo {
    pub fn new(base_address: usize) -> Self {
        let period = unsafe { read_volatile((base_address + 0x4) as *const u32) };
        HpetInfo {
            base_address,
            period,
        }
    }

    pub fn get_main_counter(&self) -> u64 {
        unsafe { read_volatile((self.base_address + 0xf0) as *const u64) }
    }

    pub fn set_main_counter(&self, value: u64) {
        unsafe { write_volatile((self.base_address + 0xf0) as *mut u64, value) }
    }

    pub fn enable(&self) {
        let conf_reg = self.base_address + 0x10;
        let value = unsafe { read_volatile(conf_reg as *const u64) };
        unsafe { write_volatile(conf_reg as *mut u64, value | 1) };
    }
}

#[derive(Clone)]
pub struct AcpiHpetHandler;

impl AcpiHandler for AcpiHpetHandler {
    unsafe fn map_physical_region<T>(
        &self,
        physical_address: usize,
        size: usize,
    ) -> PhysicalMapping<Self, T> {
        PhysicalMapping::new(physical_address, core::ptr::NonNull::new(physical_address as *mut T).unwrap(), size, size, self.clone())
    }

    fn unmap_physical_region<T>(&self, _region: &PhysicalMapping<Self, T>) {}
}

pub fn init_hpet(rsdp_addr: usize) -> Option<HpetInfo> {
    let handler = AcpiHpetHandler;
    let acpi_tables = unsafe { AcpiTables::from_rsdp(handler, rsdp_addr).ok()? };

    let hpet_table = acpi_tables.find_table::<acpi::hpet::HpetTable>().ok()??;

    let hpet_info = HpetInfo::new(hpet_table.address as usize);
    hpet_info.enable();
    hpet_info.set_main_counter(0);

    Some(hpet_info)
}
