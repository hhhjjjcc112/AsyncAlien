use basic::sync::Once;
use config::TRAMPOLINE;
use spin::Lazy;
use x86_64::PrivilegeLevel;
use x86_64::structures::idt::{
    Entry, HandlerFunc, InterruptDescriptorTable,
};

use super::vectors;

const NUM_INT: usize = 256;

pub struct IdtStruct {
    table: InterruptDescriptorTable,
}

impl IdtStruct {
    fn new(entries: &'static [unsafe extern "C" fn(); NUM_INT]) -> Self {
        let mut idt = Self {
            table: InterruptDescriptorTable::new(),
        };

        let table_entries = unsafe {
            core::slice::from_raw_parts_mut(
                &mut idt.table as *mut _ as *mut Entry<HandlerFunc>,
                NUM_INT,
            )
        };

        for vec in 0..NUM_INT {
            let entry = entries[vec] as usize;
            let offset = entry - strampoline as *const () as usize;
            let handler: unsafe extern "C" fn() =
                unsafe { core::mem::transmute(TRAMPOLINE + offset) };
            #[allow(clippy::missing_transmute_annotations)]
            let opt = table_entries[vec].set_handler_fn(unsafe { core::mem::transmute(handler) });
            if vec as u8 == vectors::BREAKPOINT || vec as u8 == vectors::SYSCALL {
                opt.set_privilege_level(PrivilegeLevel::Ring3);
            }
        }

        idt
    }

    #[inline]
    fn load(&'static self) {
        self.table.load();
    }
}

static IDT_INIT: Once<()> = Once::new();

unsafe extern "C" {
    fn strampoline();
    #[link_name = "trap_handler_table"]
    static TRAP_HANDLER_TABLE: [unsafe extern "C" fn(); NUM_INT];
}

static IDT: Lazy<IdtStruct> = Lazy::new(|| unsafe { IdtStruct::new(&TRAP_HANDLER_TABLE) });

pub fn init_idt() {
    IDT_INIT.call_once(|| {
        let _ = &*IDT;
    });

    set_trap_entry();
}

#[inline]
pub fn set_trap_entry() {
    IDT.load();
}
