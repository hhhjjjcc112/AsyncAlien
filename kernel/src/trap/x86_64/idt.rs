use basic::sync::Once;
use config::TRAMPOLINE;
use spin::Lazy;
use x86_64::PrivilegeLevel;
use x86_64::structures::idt::{
    Entry, HandlerFunc, InterruptDescriptorTable,
};

use super::vector;

const NUM_INT: usize = 256;

pub struct IdtStruct {
    table: InterruptDescriptorTable,
}

impl IdtStruct {
    fn new(entries: &'static [unsafe extern "C" fn(); NUM_INT], map_to_trampoline: bool) -> Self {
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
            let handler = if map_to_trampoline {
                let entry = entries[vec] as usize;
                let offset = entry - strampoline as *const () as usize;
                unsafe { core::mem::transmute(TRAMPOLINE + offset) }
            } else {
                entries[vec]
            };
            #[allow(clippy::missing_transmute_annotations)]
            let opt = table_entries[vec].set_handler_fn(unsafe { core::mem::transmute(handler) });
            if vec as u8 == vector::BREAKPOINT || vec as u8 == vector::SYSCALL {
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
    #[link_name = "user_trap_handler_table"]
    static USER_TRAP_HANDLER_TABLE: [unsafe extern "C" fn(); NUM_INT];
}

static KERNEL_IDT: Lazy<IdtStruct> = Lazy::new(|| unsafe { IdtStruct::new(&TRAP_HANDLER_TABLE, false) });
static USER_IDT: Lazy<IdtStruct> = Lazy::new(|| unsafe { IdtStruct::new(&USER_TRAP_HANDLER_TABLE, true) });

pub fn init_idt() {
    IDT_INIT.call_once(|| {
        let _ = &*KERNEL_IDT;
        let _ = &*USER_IDT;
    });

    set_kernel_trap_entry();
}

#[inline]
pub fn set_kernel_trap_entry() {
    KERNEL_IDT.load();
}

#[inline]
pub fn set_user_trap_entry() {
    USER_IDT.load();
}
