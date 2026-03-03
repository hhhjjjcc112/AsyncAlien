//! Local APIC and I/O APIC initialization and management for x86-64
//!
//! Supports both xAPIC and x2APIC modes.

use core::mem::MaybeUninit;

use spin::{Mutex, Once};
use x2apic::ioapic::IoApic;
use x2apic::lapic::{xapic_base, LocalApic, LocalApicBuilder};

use crate::common_x86_64::boot::PHYS_VIRT_OFFSET;

pub mod vectors {
    pub const APIC_TIMER_VECTOR: u8 = 0xf0;
    pub const APIC_SPURIOUS_VECTOR: u8 = 0xf1;
    pub const APIC_ERROR_VECTOR: u8 = 0xf2;
}

const IO_APIC_BASE: usize = 0xFEC00000;

static mut LOCAL_APIC: MaybeUninit<LocalApic> = MaybeUninit::uninit();
static mut IS_X2APIC: bool = false;
static IO_APIC: Once<Mutex<IoApic>> = Once::new();

fn cpu_has_x2apic() -> bool {
    raw_cpuid::CpuId::new()
        .get_feature_info()
        .map_or(false, |finfo| finfo.has_x2apic())
}

/// Initialize the Local APIC for the primary (BSP) CPU
pub fn init_primary_apic() {
    log::info!("Initializing Primary APIC...");
    let is_x2apic = cpu_has_x2apic();
    unsafe {
        IS_X2APIC = is_x2apic;
        // Disable 8259A PIC
        core::arch::asm!(
            "out dx, al",
            in("dx") 0x21_u16,
            in("al") 0xff_u8,
            options(nomem, nostack, preserves_flags)
        );
        core::arch::asm!(
            "out dx, al",
            in("dx") 0xa1_u16,
            in("al") 0xff_u8,
            options(nomem, nostack, preserves_flags)
        );
    }

    let mut builder = LocalApicBuilder::new();
    builder
        .spurious_vector(vectors::APIC_SPURIOUS_VECTOR as _)
        .timer_vector(vectors::APIC_TIMER_VECTOR as _)
        .error_vector(vectors::APIC_ERROR_VECTOR as _);

    if is_x2apic {
        log::info!("x2APIC mode enabled.");
    } else {
        builder.set_xapic_base(unsafe { xapic_base() } + PHYS_VIRT_OFFSET);
        log::info!("xAPIC mode enabled.");
    }

    let mut apic = builder.build().unwrap();
    unsafe {
        apic.enable();
        #[allow(static_mut_refs)]
        LOCAL_APIC.write(apic);
    }

    // Initialize I/O APIC
    log::info!("Initializing I/O APIC at {:#x}...", IO_APIC_BASE);
    let io_apic = unsafe { IoApic::new((IO_APIC_BASE as u64) + (PHYS_VIRT_OFFSET as u64)) };
    IO_APIC.call_once(|| Mutex::new(io_apic));
}

/// Initialize APIC for secondary (AP) CPUs
pub fn init_secondary_apic() {
    unsafe {
        get_local_apic().enable();
    }
}

/// Get mutable reference to local APIC
///
/// # Safety
/// Must be called after init_primary_apic or init_secondary_apic
pub unsafe fn get_local_apic() -> &'static mut LocalApic {
    #[allow(static_mut_refs)]
    unsafe { LOCAL_APIC.assume_init_mut() }
}

/// Check if running in x2APIC mode
pub fn is_x2apic() -> bool {
    unsafe { IS_X2APIC }
}

/// Get current CPU ID from APIC
pub fn current_cpu_id() -> usize {
    raw_cpuid::CpuId::new()
        .get_feature_info()
        .map_or(0, |finfo| finfo.initial_local_apic_id() as usize)
}

/// Send End-Of-Interrupt to APIC
pub fn eoi() {
    unsafe {
        get_local_apic().end_of_interrupt();
    }
}

// ============================================================================
// I/O APIC IRQ Management
// ============================================================================

/// Enable or disable an IRQ in the I/O APIC
/// 
/// For vectors below APIC_TIMER_VECTOR, this controls I/O APIC routing.
pub fn set_irq_enable(vector: usize, enabled: bool) {
    // Don't affect Local APIC interrupts
    if vector < vectors::APIC_TIMER_VECTOR as usize {
        if let Some(ioapic) = IO_APIC.get() {
            let mut ioapic = ioapic.lock();
            unsafe {
                if enabled {
                    ioapic.enable_irq(vector as u8);
                } else {
                    ioapic.disable_irq(vector as u8);
                }
            }
        }
    }
}

/// Get raw APIC ID for IPI targeting
pub fn raw_apic_id(cpu_id: u8) -> u32 {
    if is_x2apic() {
        cpu_id as u32
    } else {
        (cpu_id as u32) << 24
    }
}

/// Send IPI to another CPU
pub fn send_ipi(target_cpu: usize, vector: u8) {
    let apic_id = raw_apic_id(target_cpu as u8);
    unsafe {
        get_local_apic().send_ipi(vector, apic_id);
    }
}

/// Send IPI to self
pub fn send_ipi_self(vector: u8) {
    unsafe {
        get_local_apic().send_ipi_self(vector);
    }
}

/// Send IPI to all other CPUs
pub fn send_ipi_all_excluding_self(vector: u8) {
    use x2apic::lapic::IpiAllShorthand;
    unsafe {
        get_local_apic().send_ipi_all(vector, IpiAllShorthand::AllExcludingSelf);
    }
}

/// Get the I/O APIC maximum redirection entry count
pub fn ioapic_max_entries() -> u8 {
    if let Some(ioapic) = IO_APIC.get() {
        let mut ioapic = ioapic.lock();
        unsafe { ioapic.max_table_entry() + 1 }
    } else {
        0
    }
}

/// Configure I/O APIC redirection entry for an IRQ
/// 
/// Maps a hardware IRQ to a specific interrupt vector and target CPU.
pub fn configure_irq(irq: u8, vector: u8, dest_cpu: u8) {
    if let Some(ioapic) = IO_APIC.get() {
        let mut ioapic = ioapic.lock();
        unsafe {
            // Set up the redirection entry
            let mut entry = ioapic.table_entry(irq);
            entry.set_vector(vector);
            entry.set_dest(dest_cpu);
            // Set delivery mode to Fixed (0), physical destination
            entry.set_mode(x2apic::ioapic::IrqMode::Fixed);
            entry.set_flags(
                x2apic::ioapic::IrqFlags::LEVEL_TRIGGERED 
                | x2apic::ioapic::IrqFlags::LOW_ACTIVE 
                | x2apic::ioapic::IrqFlags::MASKED
            );
            ioapic.set_table_entry(irq, entry);
        }
    }
}
