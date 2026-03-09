extern crate alloc;

use alloc::boxed::Box;
use core::{
    mem::size_of,
    ptr::NonNull,
    slice,
    sync::atomic::{AtomicU32, Ordering},
};

use acpi::{
    AcpiTables, Handle, Handler as AcpiHandler, HpetInfo, PhysicalMapping, PciAddress,
    rsdp::Rsdp,
    sdt::{SdtHeader, madt::{Madt, MadtEntry}},
};
use aml::{AmlContext, DebugVerbosity, Handler as AmlHandler};
use spin::{Mutex, Once};

use crate::common_x86_64::boot::PHYS_VIRT_OFFSET;

const DEFAULT_LAPIC_BASE: usize = 0xfee0_0000;
const DEFAULT_IOAPIC_BASE: usize = 0xfec0_0000;

#[derive(Clone, Debug)]
pub struct AcpiDeviceInfo {
    pub lapic_base: usize,
    pub ioapic_base: usize,
    pub hpet_base: Option<usize>,
    pub devices: AcpiDeviceList,
}

impl Default for AcpiDeviceInfo {
    fn default() -> Self {
        Self {
            lapic_base: DEFAULT_LAPIC_BASE,
            ioapic_base: DEFAULT_IOAPIC_BASE,
            hpet_base: None,
            devices: AcpiDeviceList::default(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AcpiDeviceEntry {
    pub name: &'static str,
    pub base: usize,
    pub size: usize,
}

#[derive(Clone, Debug)]
pub struct AcpiDeviceList {
    pub entries: heapless::Vec<AcpiDeviceEntry, 16>,
}

impl Default for AcpiDeviceList {
    fn default() -> Self {
        Self {
            entries: heapless::Vec::new(),
        }
    }
}

pub fn device_list() -> AcpiDeviceList {
    device_info().devices.clone()
}

static ACPI_INFO: Once<AcpiDeviceInfo> = Once::new();

pub fn init() {
    ACPI_INFO.call_once(detect_acpi_info);
}

pub fn device_info() -> AcpiDeviceInfo {
    init();
    ACPI_INFO.get().cloned().unwrap_or_default()
}

#[derive(Clone, Copy)]
struct AcpiHost;

impl AcpiHost {
    #[inline]
    fn phys_to_virt(paddr: usize) -> usize {
        paddr + PHYS_VIRT_OFFSET as usize
    }

    #[inline]
    unsafe fn map_ptr<T>(&self, paddr: usize) -> NonNull<T> {
        // ACPI tables are physical-memory backed and permanently mapped by the kernel.
        unsafe { NonNull::new_unchecked(Self::phys_to_virt(paddr) as *mut T) }
    }
}

impl AcpiHandler for AcpiHost {
    unsafe fn map_physical_region<T>(&self, physical_address: usize, size: usize) -> PhysicalMapping<Self, T> {
        PhysicalMapping {
            physical_start: physical_address,
            virtual_start: unsafe { self.map_ptr(physical_address) },
            region_length: size,
            mapped_length: size,
            handler: *self,
        }
    }

    fn unmap_physical_region<T>(_region: &PhysicalMapping<Self, T>) {}

    fn read_u8(&self, address: usize) -> u8 {
        unsafe { (Self::phys_to_virt(address) as *const u8).read_volatile() }
    }

    fn read_u16(&self, address: usize) -> u16 {
        unsafe { (Self::phys_to_virt(address) as *const u16).read_volatile() }
    }

    fn read_u32(&self, address: usize) -> u32 {
        unsafe { (Self::phys_to_virt(address) as *const u32).read_volatile() }
    }

    fn read_u64(&self, address: usize) -> u64 {
        unsafe { (Self::phys_to_virt(address) as *const u64).read_volatile() }
    }

    fn write_u8(&self, address: usize, value: u8) {
        unsafe { (Self::phys_to_virt(address) as *mut u8).write_volatile(value) }
    }

    fn write_u16(&self, address: usize, value: u16) {
        unsafe { (Self::phys_to_virt(address) as *mut u16).write_volatile(value) }
    }

    fn write_u32(&self, address: usize, value: u32) {
        unsafe { (Self::phys_to_virt(address) as *mut u32).write_volatile(value) }
    }

    fn write_u64(&self, address: usize, value: u64) {
        unsafe { (Self::phys_to_virt(address) as *mut u64).write_volatile(value) }
    }

    fn read_io_u8(&self, port: u16) -> u8 {
        unsafe { x86::io::inb(port) }
    }

    fn read_io_u16(&self, port: u16) -> u16 {
        unsafe { x86::io::inw(port) }
    }

    fn read_io_u32(&self, port: u16) -> u32 {
        unsafe { x86::io::inl(port) }
    }

    fn write_io_u8(&self, port: u16, value: u8) {
        unsafe { x86::io::outb(port, value) }
    }

    fn write_io_u16(&self, port: u16, value: u16) {
        unsafe { x86::io::outw(port, value) }
    }

    fn write_io_u32(&self, port: u16, value: u32) {
        unsafe { x86::io::outl(port, value) }
    }

    fn read_pci_u8(&self, address: PciAddress, offset: u16) -> u8 {
        ((self.read_pci_u32(address, offset & !0x3) >> ((offset & 0x3) * 8)) & 0xff) as u8
    }

    fn read_pci_u16(&self, address: PciAddress, offset: u16) -> u16 {
        ((self.read_pci_u32(address, offset & !0x3) >> ((offset & 0x2) * 8)) & 0xffff) as u16
    }

    fn read_pci_u32(&self, address: PciAddress, offset: u16) -> u32 {
        pci_cfg_read32(address, offset)
    }

    fn write_pci_u8(&self, address: PciAddress, offset: u16, value: u8) {
        let aligned = offset & !0x3;
        let mut cur = self.read_pci_u32(address, aligned);
        let shift = (offset & 0x3) * 8;
        cur = (cur & !(0xff << shift)) | ((value as u32) << shift);
        self.write_pci_u32(address, aligned, cur);
    }

    fn write_pci_u16(&self, address: PciAddress, offset: u16, value: u16) {
        let aligned = offset & !0x3;
        let mut cur = self.read_pci_u32(address, aligned);
        let shift = (offset & 0x2) * 8;
        cur = (cur & !(0xffff << shift)) | ((value as u32) << shift);
        self.write_pci_u32(address, aligned, cur);
    }

    fn write_pci_u32(&self, address: PciAddress, offset: u16, value: u32) {
        pci_cfg_write32(address, offset, value)
    }

    fn nanos_since_boot(&self) -> u64 {
        crate::common_x86_64::time::current_time_nanos()
    }

    fn stall(&self, microseconds: u64) {
        let deadline = self.nanos_since_boot().saturating_add(microseconds.saturating_mul(1_000));
        while self.nanos_since_boot() < deadline {
            core::hint::spin_loop();
        }
    }

    fn sleep(&self, milliseconds: u64) {
        self.stall(milliseconds.saturating_mul(1_000));
    }

    fn create_mutex(&self) -> Handle {
        Handle(NEXT_MUTEX_ID.fetch_add(1, Ordering::Relaxed))
    }

    fn acquire(&self, mutex: Handle, timeout: u16) -> Result<(), acpi::aml::AmlError> {
        let idx = (mutex.0 as usize) % AML_MUTEX_MAX;
        if timeout == 0 {
            let mut states = AML_MUTEX_STATE.lock();
            if states[idx] {
                return Err(acpi::aml::AmlError::MutexAcquireTimeout);
            }
            states[idx] = true;
            return Ok(());
        }

        let deadline = if timeout == 0xffff {
            u64::MAX
        } else {
            self.nanos_since_boot().saturating_add((timeout as u64).saturating_mul(1_000_000))
        };

        loop {
            {
                let mut states = AML_MUTEX_STATE.lock();
                if !states[idx] {
                    states[idx] = true;
                    return Ok(());
                }
            }

            if timeout != 0xffff && self.nanos_since_boot() >= deadline {
                return Err(acpi::aml::AmlError::MutexAcquireTimeout);
            }

            core::hint::spin_loop();
        }
    }

    fn release(&self, mutex: Handle) {
        let idx = (mutex.0 as usize) % AML_MUTEX_MAX;
        let mut states = AML_MUTEX_STATE.lock();
        states[idx] = false;
    }
}

struct AmlHost;

impl AmlHandler for AmlHost {
    fn read_u8(&self, address: usize) -> u8 {
        unsafe { (AcpiHost::phys_to_virt(address) as *const u8).read_volatile() }
    }

    fn read_u16(&self, address: usize) -> u16 {
        unsafe { (AcpiHost::phys_to_virt(address) as *const u16).read_volatile() }
    }

    fn read_u32(&self, address: usize) -> u32 {
        unsafe { (AcpiHost::phys_to_virt(address) as *const u32).read_volatile() }
    }

    fn read_u64(&self, address: usize) -> u64 {
        unsafe { (AcpiHost::phys_to_virt(address) as *const u64).read_volatile() }
    }

    fn write_u8(&mut self, address: usize, value: u8) {
        unsafe { (AcpiHost::phys_to_virt(address) as *mut u8).write_volatile(value) }
    }

    fn write_u16(&mut self, address: usize, value: u16) {
        unsafe { (AcpiHost::phys_to_virt(address) as *mut u16).write_volatile(value) }
    }

    fn write_u32(&mut self, address: usize, value: u32) {
        unsafe { (AcpiHost::phys_to_virt(address) as *mut u32).write_volatile(value) }
    }

    fn write_u64(&mut self, address: usize, value: u64) {
        unsafe { (AcpiHost::phys_to_virt(address) as *mut u64).write_volatile(value) }
    }

    fn read_io_u8(&self, port: u16) -> u8 {
        unsafe { x86::io::inb(port) }
    }

    fn read_io_u16(&self, port: u16) -> u16 {
        unsafe { x86::io::inw(port) }
    }

    fn read_io_u32(&self, port: u16) -> u32 {
        unsafe { x86::io::inl(port) }
    }

    fn write_io_u8(&self, port: u16, value: u8) {
        unsafe { x86::io::outb(port, value) }
    }

    fn write_io_u16(&self, port: u16, value: u16) {
        unsafe { x86::io::outw(port, value) }
    }

    fn write_io_u32(&self, port: u16, value: u32) {
        unsafe { x86::io::outl(port, value) }
    }

    fn read_pci_u8(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16) -> u8 {
        let addr = PciAddress::new(segment, bus, device, function);
        ((pci_cfg_read32(addr, offset & !0x3) >> ((offset & 0x3) * 8)) & 0xff) as u8
    }

    fn read_pci_u16(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16) -> u16 {
        let addr = PciAddress::new(segment, bus, device, function);
        ((pci_cfg_read32(addr, offset & !0x3) >> ((offset & 0x2) * 8)) & 0xffff) as u16
    }

    fn read_pci_u32(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16) -> u32 {
        let addr = PciAddress::new(segment, bus, device, function);
        pci_cfg_read32(addr, offset)
    }

    fn write_pci_u8(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16, value: u8) {
        let addr = PciAddress::new(segment, bus, device, function);
        let aligned = offset & !0x3;
        let mut cur = pci_cfg_read32(addr, aligned);
        let shift = (offset & 0x3) * 8;
        cur = (cur & !(0xff << shift)) | ((value as u32) << shift);
        pci_cfg_write32(addr, aligned, cur);
    }

    fn write_pci_u16(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16, value: u16) {
        let addr = PciAddress::new(segment, bus, device, function);
        let aligned = offset & !0x3;
        let mut cur = pci_cfg_read32(addr, aligned);
        let shift = (offset & 0x2) * 8;
        cur = (cur & !(0xffff << shift)) | ((value as u32) << shift);
        pci_cfg_write32(addr, aligned, cur);
    }

    fn write_pci_u32(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16, value: u32) {
        let addr = PciAddress::new(segment, bus, device, function);
        pci_cfg_write32(addr, offset, value);
    }
}

#[inline]
fn pci_cfg_address(address: PciAddress, offset: u16) -> u32 {
    // Legacy config mechanism #1. Segment > 0 is not reachable via ports.
    let _segment = address.segment();
    (1u32 << 31)
        | ((address.bus() as u32) << 16)
        | ((address.device() as u32) << 11)
        | ((address.function() as u32) << 8)
        | ((offset as u32) & 0xfc)
}

fn pci_cfg_read32(address: PciAddress, offset: u16) -> u32 {
    unsafe {
        x86::io::outl(0xcf8, pci_cfg_address(address, offset));
        x86::io::inl(0xcfc)
    }
}

fn pci_cfg_write32(address: PciAddress, offset: u16, value: u32) {
    unsafe {
        x86::io::outl(0xcf8, pci_cfg_address(address, offset));
        x86::io::outl(0xcfc, value);
    }
}

fn detect_acpi_info() -> AcpiDeviceInfo {
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

    if let Some(madt) = tables.find_table::<Madt>() {
        let madt = madt.get();
        info.lapic_base = madt.local_apic_address as usize;

        for entry in madt.entries() {
            match entry {
                MadtEntry::IoApic(ioapic) => {
                    info.ioapic_base = ioapic.io_apic_address as usize;
                }
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

    if let Ok(hpet) = HpetInfo::new(&tables) {
        info.hpet_base = Some(hpet.base_address);
        let _ = info.devices.entries.push(AcpiDeviceEntry {
            name: "hpet",
            base: hpet.base_address,
            size: 0x1000,
        });
    }

    if let Ok(pci_regions) = acpi::platform::PciConfigRegions::new(&tables) {
        for region in pci_regions.regions.iter() {
            let bus_count = (region.bus_number_end - region.bus_number_start) as usize + 1;
            let size = bus_count << 20; // 1MiB per bus in ECAM
            let _ = info.devices.entries.push(AcpiDeviceEntry {
                name: "pci_ecam",
                base: region.base_address as usize,
                size,
            });
        }
    }

    // Parse DSDT/SSDT AML tables to ensure ACPI namespace is available early.
    let mut aml_ctx = AmlContext::new(Box::new(AmlHost), DebugVerbosity::None);
    let mut aml_loaded = 0usize;

    if let Ok(dsdt) = tables.dsdt() {
        if load_aml_table(&mut aml_ctx, dsdt).is_ok() {
            aml_loaded += 1;
        }
    }

    for ssdt in tables.ssdts() {
        if load_aml_table(&mut aml_ctx, ssdt).is_ok() {
            aml_loaded += 1;
        }
    }

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

fn load_aml_table(ctx: &mut AmlContext, table: acpi::AmlTable) -> Result<(), aml::AmlError> {
    let raw = unsafe {
        slice::from_raw_parts(
            AcpiHost::phys_to_virt(table.phys_address) as *const u8,
            table.length as usize,
        )
    };

    if raw.len() <= size_of::<SdtHeader>() {
        return Ok(());
    }

    let aml_stream = &raw[size_of::<SdtHeader>()..];
    ctx.parse_table(aml_stream)
}

const AML_MUTEX_MAX: usize = 64;
static NEXT_MUTEX_ID: AtomicU32 = AtomicU32::new(1);
static AML_MUTEX_STATE: Mutex<[bool; AML_MUTEX_MAX]> = Mutex::new([false; AML_MUTEX_MAX]);
