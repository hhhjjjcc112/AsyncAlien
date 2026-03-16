use basic::sync::Once;
use core::arch::asm;

use super::vector;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum GateType {
    Interrupt = 0xE,
    Trap = 0xF,
}

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
        Self {
            offset_low: 0,
            selector: 0,
            ist: 0,
            type_attr: 0,
            offset_mid: 0,
            offset_high: 0,
            reserved: 0,
        }
    }

    pub fn new(handler: usize, selector: u16, gate_type: GateType, dpl: u8, ist: u8) -> Self {
        Self {
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

#[repr(C, align(16))]
pub struct Idt {
    entries: [IdtEntry; 256],
}

impl Idt {
    pub const fn new() -> Self {
        Self {
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

static mut IDT: Idt = Idt::new();
static IDT_INIT: Once<()> = Once::new();

unsafe extern "C" {
    #[link_name = "trap_handler_table"]
    static TRAP_HANDLER_TABLE: [unsafe extern "C" fn(); 256];
}

pub fn init_idt() {
    IDT_INIT.call_once(|| unsafe {
        // 关键步骤：将汇编生成的 256 个入口写入 IDT。
        let idt = &raw mut IDT;
        let handlers = &raw const TRAP_HANDLER_TABLE;
        for vec in 0u8..=u8::MAX {
            let idx = vec as usize;
            let handler = (*handlers)[idx] as usize;
            let gate_type = if vec == vector::BREAKPOINT {
                GateType::Trap
            } else {
                GateType::Interrupt
            };
            let dpl = if vec == vector::BREAKPOINT || vec == vector::SYSCALL {
                3
            } else {
                0
            };
            (*idt).set_handler(vec, handler, gate_type, dpl);
        }
    });

    // 关键步骤：每个 CPU 都要执行一次 lidt。
    unsafe {
        let idt = &raw const IDT;
        (*idt).load();
    }
}
