use core::{ptr::NonNull, sync::atomic::{AtomicU32, Ordering}};

use acpi::{Handle, Handler as AcpiHandler, PhysicalMapping, PciAddress};
use spin::Mutex;

use platform::TimeIf;

use super::aml_handler::{pci_cfg_read32, pci_cfg_write32, phys_to_virt};

const AML_MUTEX_MAX: usize = 64;

static NEXT_MUTEX_ID: AtomicU32 = AtomicU32::new(1);
static AML_MUTEX_STATE: Mutex<[bool; AML_MUTEX_MAX]> = Mutex::new([false; AML_MUTEX_MAX]);

#[derive(Clone, Copy)]
pub(super) struct AcpiHost;

#[inline]
/// 函数说明：执行对应的总线处理步骤。
fn uptime_nanos() -> u64 {
    <platform::Platform as TimeIf>::monotonic_time_nanos()
}

impl AcpiHandler for AcpiHost {
    unsafe fn map_physical_region<T>(
        &self,
        physical_address: usize,
        size: usize,
    ) -> PhysicalMapping<Self, T> {
        PhysicalMapping {
            physical_start: physical_address,
            virtual_start: unsafe { NonNull::new_unchecked(phys_to_virt(physical_address) as *mut T) },
            region_length: size,
            mapped_length: size,
            handler: *self,
        }
    }

/// 函数说明：执行对应的总线处理步骤。
    fn unmap_physical_region<T>(_region: &PhysicalMapping<Self, T>) {}

/// 函数说明：执行对应的总线处理步骤。
    fn read_u8(&self, address: usize) -> u8 {
        unsafe { (phys_to_virt(address) as *const u8).read_volatile() }
    }

/// 函数说明：执行对应的总线处理步骤。
    fn read_u16(&self, address: usize) -> u16 {
        unsafe { (phys_to_virt(address) as *const u16).read_volatile() }
    }

/// 函数说明：执行对应的总线处理步骤。
    fn read_u32(&self, address: usize) -> u32 {
        unsafe { (phys_to_virt(address) as *const u32).read_volatile() }
    }

/// 函数说明：执行对应的总线处理步骤。
    fn read_u64(&self, address: usize) -> u64 {
        unsafe { (phys_to_virt(address) as *const u64).read_volatile() }
    }

/// 函数说明：执行对应的总线处理步骤。
    fn write_u8(&self, address: usize, value: u8) {
        unsafe { (phys_to_virt(address) as *mut u8).write_volatile(value) }
    }

/// 函数说明：执行对应的总线处理步骤。
    fn write_u16(&self, address: usize, value: u16) {
        unsafe { (phys_to_virt(address) as *mut u16).write_volatile(value) }
    }

/// 函数说明：执行对应的总线处理步骤。
    fn write_u32(&self, address: usize, value: u32) {
        unsafe { (phys_to_virt(address) as *mut u32).write_volatile(value) }
    }

/// 函数说明：执行对应的总线处理步骤。
    fn write_u64(&self, address: usize, value: u64) {
        unsafe { (phys_to_virt(address) as *mut u64).write_volatile(value) }
    }

/// 函数说明：执行对应的总线处理步骤。
    fn read_io_u8(&self, port: u16) -> u8 {
        unsafe { x86::io::inb(port) }
    }

/// 函数说明：执行对应的总线处理步骤。
    fn read_io_u16(&self, port: u16) -> u16 {
        unsafe { x86::io::inw(port) }
    }

/// 函数说明：执行对应的总线处理步骤。
    fn read_io_u32(&self, port: u16) -> u32 {
        unsafe { x86::io::inl(port) }
    }

/// 函数说明：执行对应的总线处理步骤。
    fn write_io_u8(&self, port: u16, value: u8) {
        unsafe { x86::io::outb(port, value) }
    }

/// 函数说明：执行对应的总线处理步骤。
    fn write_io_u16(&self, port: u16, value: u16) {
        unsafe { x86::io::outw(port, value) }
    }

/// 函数说明：执行对应的总线处理步骤。
    fn write_io_u32(&self, port: u16, value: u32) {
        unsafe { x86::io::outl(port, value) }
    }

/// 函数说明：执行对应的总线处理步骤。
    fn read_pci_u8(&self, address: PciAddress, offset: u16) -> u8 {
        ((pci_cfg_read32(address, offset & !0x3) >> ((offset & 0x3) * 8)) & 0xff) as u8
    }

/// 函数说明：执行对应的总线处理步骤。
    fn read_pci_u16(&self, address: PciAddress, offset: u16) -> u16 {
        ((pci_cfg_read32(address, offset & !0x3) >> ((offset & 0x2) * 8)) & 0xffff) as u16
    }

/// 函数说明：执行对应的总线处理步骤。
    fn read_pci_u32(&self, address: PciAddress, offset: u16) -> u32 {
        pci_cfg_read32(address, offset)
    }

/// 函数说明：执行对应的总线处理步骤。
    fn write_pci_u8(&self, address: PciAddress, offset: u16, value: u8) {
        let aligned = offset & !0x3;
        let mut cur = pci_cfg_read32(address, aligned);
        let shift = (offset & 0x3) * 8;
        cur = (cur & !(0xff << shift)) | ((value as u32) << shift);
        pci_cfg_write32(address, aligned, cur);
    }

/// 函数说明：执行对应的总线处理步骤。
    fn write_pci_u16(&self, address: PciAddress, offset: u16, value: u16) {
        let aligned = offset & !0x3;
        let mut cur = pci_cfg_read32(address, aligned);
        let shift = (offset & 0x2) * 8;
        cur = (cur & !(0xffff << shift)) | ((value as u32) << shift);
        pci_cfg_write32(address, aligned, cur);
    }

/// 函数说明：执行对应的总线处理步骤。
    fn write_pci_u32(&self, address: PciAddress, offset: u16, value: u32) {
        pci_cfg_write32(address, offset, value)
    }

/// 函数说明：执行对应的总线处理步骤。
    fn create_mutex(&self) -> Handle {
        Handle(NEXT_MUTEX_ID.fetch_add(1, Ordering::Relaxed))
    }

/// 函数说明：执行对应的总线处理步骤。
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
            uptime_nanos().saturating_add((timeout as u64).saturating_mul(1_000_000))
        };

        loop {
            {
                let mut states = AML_MUTEX_STATE.lock();
                if !states[idx] {
                    states[idx] = true;
                    return Ok(());
                }
            }

            if timeout != 0xffff && uptime_nanos() >= deadline {
                return Err(acpi::aml::AmlError::MutexAcquireTimeout);
            }

            core::hint::spin_loop();
        }
    }

/// 函数说明：执行对应的总线处理步骤。
    fn release(&self, mutex: Handle) {
        let idx = (mutex.0 as usize) % AML_MUTEX_MAX;
        let mut states = AML_MUTEX_STATE.lock();
        states[idx] = false;
    }

/// 函数说明：执行对应的总线处理步骤。
    fn nanos_since_boot(&self) -> u64 {
        uptime_nanos()
    }

/// 函数说明：执行对应的总线处理步骤。
    fn stall(&self, microseconds: u64) {
        let deadline = uptime_nanos().saturating_add(microseconds.saturating_mul(1_000));
        while uptime_nanos() < deadline {
            core::hint::spin_loop();
        }
    }

/// 函数说明：执行对应的总线处理步骤。
    fn sleep(&self, milliseconds: u64) {
        self.stall(milliseconds.saturating_mul(1_000));
    }
}
