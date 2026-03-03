//! x86-64 Trap handling
//!
//! This module implements trap/interrupt handling for x86-64 architecture
//! using native x86-64 naming conventions (IDT, vectors, IOAPIC, etc.).

use basic::sync::OnceGet;
use core::arch::asm;
use platform;

use crate::{plic_domain, task_domain, timer};

// ============================================================================
// x86-64 Interrupt/Exception Vectors
// ============================================================================

/// x86-64 exception vector numbers
pub mod vector {
    pub const DIVIDE_ERROR: u8 = 0;
    pub const DEBUG: u8 = 1;
    pub const NMI: u8 = 2;
    pub const BREAKPOINT: u8 = 3;
    pub const OVERFLOW: u8 = 4;
    pub const BOUND_RANGE: u8 = 5;
    pub const INVALID_OPCODE: u8 = 6;
    pub const DEVICE_NOT_AVAILABLE: u8 = 7;
    pub const DOUBLE_FAULT: u8 = 8;
    pub const INVALID_TSS: u8 = 10;
    pub const SEGMENT_NOT_PRESENT: u8 = 11;
    pub const STACK_SEGMENT_FAULT: u8 = 12;
    pub const GENERAL_PROTECTION: u8 = 13;
    pub const PAGE_FAULT: u8 = 14;
    pub const X87_FLOATING_POINT: u8 = 16;
    pub const ALIGNMENT_CHECK: u8 = 17;
    pub const MACHINE_CHECK: u8 = 18;
    pub const SIMD_FLOATING_POINT: u8 = 19;
    pub const VIRTUALIZATION: u8 = 20;
    pub const SECURITY_EXCEPTION: u8 = 30;
    
    // IRQ vectors (mapped after CPU exceptions)
    pub const IRQ_BASE: u8 = 32;
    pub const TIMER: u8 = IRQ_BASE + 0;
    pub const KEYBOARD: u8 = IRQ_BASE + 1;
    pub const SYSCALL: u8 = 0x80;
    
    // APIC vectors
    pub const APIC_TIMER: u8 = 0xFE;
    pub const APIC_ERROR: u8 = 0xFD;
    pub const APIC_SPURIOUS: u8 = 0xFF;
}

// ============================================================================
// Trap Frame for x86-64
// ============================================================================

/// x86-64 trap frame pushed by CPU and trap handler
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct X86TrapFrame {
    // Callee-saved registers (pushed by handler)
    pub r15: usize,
    pub r14: usize,
    pub r13: usize,
    pub r12: usize,
    pub rbp: usize,
    pub rbx: usize,
    // Scratch registers (pushed by handler)
    pub r11: usize,
    pub r10: usize,
    pub r9: usize,
    pub r8: usize,
    pub rsi: usize,
    pub rdi: usize,
    pub rdx: usize,
    pub rcx: usize,
    pub rax: usize,
    // Vector number (pushed by handler)
    pub vector: usize,
    // Error code (pushed by CPU or handler)
    pub error_code: usize,
    // Pushed by CPU on interrupt/exception
    pub rip: usize,
    pub cs: usize,
    pub rflags: usize,
    pub rsp: usize,
    pub ss: usize,
}

impl X86TrapFrame {
    /// Check if trap came from user mode (ring 3)
    pub fn is_user(&self) -> bool {
        (self.cs & 0x3) == 3
    }
    
    /// Check if trap came from kernel mode (ring 0)
    pub fn is_kernel(&self) -> bool {
        (self.cs & 0x3) == 0
    }
    
    /// Get fault address for page faults (from CR2)
    pub fn fault_address() -> usize {
        let addr: usize;
        unsafe {
            asm!("mov {}, cr2", out(reg) addr, options(nomem, nostack, preserves_flags));
        }
        addr
    }
}

// ============================================================================
// IDT Entry and Table
// ============================================================================

/// IDT gate types
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum GateType {
    Interrupt = 0xE,
    Trap = 0xF,
}

/// IDT entry (Interrupt Descriptor Table entry)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct IdtEntry {
    offset_low: u16,
    selector: u16,
    ist: u8,
    type_attr: u8,
    offset_mid: u16,
    offset_high: u32,
    reserved: u32,
}

impl IdtEntry {
    pub const fn empty() -> Self {
        IdtEntry {
            offset_low: 0,
            selector: 0,
            ist: 0,
            type_attr: 0,
            offset_mid: 0,
            offset_high: 0,
            reserved: 0,
        }
    }
    
    /// Create a new IDT entry
    pub fn new(handler: usize, selector: u16, gate_type: GateType, dpl: u8, ist: u8) -> Self {
        IdtEntry {
            offset_low: handler as u16,
            selector,
            ist,
            type_attr: (1 << 7) | ((dpl & 0x3) << 5) | (gate_type as u8),
            offset_mid: (handler >> 16) as u16,
            offset_high: (handler >> 32) as u32,
            reserved: 0,
        }
    }
}

/// IDT table with 256 entries
#[repr(C, align(16))]
pub struct Idt {
    entries: [IdtEntry; 256],
}

impl Idt {
    pub const fn new() -> Self {
        Idt {
            entries: [IdtEntry::empty(); 256],
        }
    }
    
    pub fn set_handler(&mut self, vector: u8, handler: usize, gate_type: GateType, dpl: u8) {
        self.entries[vector as usize] = IdtEntry::new(handler, 0x08, gate_type, dpl, 0);
    }
    
    pub fn load(&self) {
        #[repr(C, packed)]
        struct IdtPointer {
            limit: u16,
            base: u64,
        }
        
        let ptr = IdtPointer {
            limit: (core::mem::size_of::<Idt>() - 1) as u16,
            base: self as *const _ as u64,
        };
        
        unsafe {
            asm!("lidt [{}]", in(reg) &ptr, options(readonly, nostack, preserves_flags));
        }
    }
}

// ============================================================================
// Trap entry configuration
// ============================================================================

/// Set kernel trap entry point
/// 
/// On x86-64, this configures the IDT for kernel mode exception handling.
#[inline]
pub fn set_kernel_trap_entry() {
    // IDT is already set up, no per-trap-entry switching needed on x86-64
    // Unlike RISC-V's stvec, x86-64 IDT is always active
}

/// Set user trap entry point
/// 
/// On x86-64, user and kernel share the same IDT.
#[inline]
pub fn set_user_trap_entry() {
    // No operation needed - x86-64 uses a single IDT for both modes
}

/// User trap entry vector (address marker)
/// 
/// On x86-64, traps enter through IDT handlers, but this function
/// serves as a marker for the syscall interface to provide a consistent API.
#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn user_trap_vector() {
    // This is a marker function - actual trap handling goes through IDT
    // The address of this function is returned by sys_trap_from_user()
}

// ============================================================================
// Trap handlers
// ============================================================================

/// Kernel trap handler
/// 
/// Called when an interrupt/exception occurs in ring 0.
#[unsafe(no_mangle)]
pub fn kernel_trap_handler(frame: &mut X86TrapFrame) {
    if frame.is_user() {
        panic!("kernel_trap_handler: received user-mode trap");
    }
    
    let enable = arch::is_interrupt_enable();
    assert!(!enable, "Interrupts should be disabled in kernel trap handler");
    
    handle_trap(frame, false);
}

/// User trap handler
/// 
/// Called when an interrupt/exception occurs in ring 3.
#[unsafe(no_mangle)]
pub fn user_trap_handler(frame: &mut X86TrapFrame) {
    if frame.is_kernel() {
        panic!("user_trap_handler: received kernel-mode trap");
    }
    
    set_kernel_trap_entry();
    handle_trap(frame, true);
    trap_return();
}

/// Common trap handler
fn handle_trap(frame: &mut X86TrapFrame, from_user: bool) {
    let vector = frame.vector as u8;
    
    match vector {
        // CPU Exceptions
        vector::DIVIDE_ERROR => {
            panic!("Divide error at RIP={:#x}", frame.rip);
        }
        vector::DEBUG => {
            log::debug!("Debug exception at RIP={:#x}", frame.rip);
        }
        vector::BREAKPOINT => {
            log::debug!("Breakpoint at RIP={:#x}", frame.rip);
        }
        vector::INVALID_OPCODE => {
            panic!("Invalid opcode at RIP={:#x}", frame.rip);
        }
        vector::GENERAL_PROTECTION => {
            panic!(
                "General protection fault at RIP={:#x}, error_code={:#x}",
                frame.rip, frame.error_code
            );
        }
        vector::PAGE_FAULT => {
            let fault_addr = X86TrapFrame::fault_address();
            if from_user {
                task_domain!()
                    .do_load_page_fault(fault_addr)
                    .expect("do_load_page_fault failed");
                log::debug!(
                    "Page fault handled: addr={:#x}, RIP={:#x}",
                    fault_addr, frame.rip
                );
            } else {
                panic!(
                    "Kernel page fault at RIP={:#x}, fault_addr={:#x}, error={:#x}",
                    frame.rip, fault_addr, frame.error_code
                );
            }
        }
        vector::DOUBLE_FAULT => {
            panic!("Double fault! RIP={:#x}", frame.rip);
        }
        
        // Timer interrupt (APIC timer)
        vector::APIC_TIMER => {
            trace!("APIC timer interrupt");
            timer::set_next_trigger();
            if from_user {
                crate::task::yield_now();
            }
            // Send EOI to APIC
            send_apic_eoi();
        }
        
        // System call
        vector::SYSCALL => {
            if from_user {
                super::exception::syscall_exception_handler();
            } else {
                panic!("syscall from kernel mode");
            }
        }
        
        // External interrupts (IRQ 0-15)
        v if v >= vector::IRQ_BASE && v < vector::IRQ_BASE + 16 => {
            trace!("[{}] External interrupt: IRQ {}", arch::cpu_id(), v - vector::IRQ_BASE);
            plic_domain!().handle_irq().expect("handle_irq failed");
            send_apic_eoi();
        }
        
        // APIC spurious interrupt
        vector::APIC_SPURIOUS => {
            log::warn!("Spurious APIC interrupt");
        }
        
        _ => {
            panic!(
                "Unhandled trap: vector={}, RIP={:#x}, error={:#x}",
                vector, frame.rip, frame.error_code
            );
        }
    }
}

/// Send End-Of-Interrupt to APIC
fn send_apic_eoi() {
    platform::apic::eoi();
}

/// Return to user mode
/// 
/// Restores user context and executes IRETQ to return to user space.
#[unsafe(no_mangle)]
pub fn trap_return() -> ! {
    set_user_trap_entry();
    let task_domain = task_domain!();
    let (user_cr3, trap_cx_ptr) = task_domain.satp_with_trap_frame_virt_addr().unwrap();
    
    // Switch to user page table and restore context
    // This would typically jump to a return stub that does IRETQ
    unsafe {
        asm!(
            // Load user CR3
            "mov cr3, {cr3}",
            // Load trap frame pointer
            "mov rsp, {frame}",
            // Restore general purpose registers
            "pop r15",
            "pop r14",
            "pop r13",
            "pop r12",
            "pop rbp",
            "pop rbx",
            "pop r11",
            "pop r10",
            "pop r9",
            "pop r8",
            "pop rsi",
            "pop rdi",
            "pop rdx",
            "pop rcx",
            "pop rax",
            // Skip vector and error_code
            "add rsp, 16",
            // IRETQ: pops RIP, CS, RFLAGS, RSP, SS
            "iretq",
            cr3 = in(reg) user_cr3,
            frame = in(reg) trap_cx_ptr,
            options(noreturn)
        )
    }
}

// ============================================================================
// IDT Initialization
// ============================================================================

// Static IDT - will be initialized at boot
static mut IDT: Idt = Idt::new();

/// Initialize the IDT
/// 
/// Sets up interrupt/exception handlers for all vectors.
pub fn init_idt() {
    // TODO: Set up all exception and interrupt handlers
    // Each handler should be an assembly stub that:
    // 1. Pushes error code (if not pushed by CPU)
    // 2. Pushes vector number
    // 3. Saves all registers
    // 4. Calls kernel_trap_handler or user_trap_handler
    // 5. Restores registers
    // 6. Does IRETQ
    
    unsafe {
        let idt = &raw mut IDT;
        (*idt).load();
    }
}
