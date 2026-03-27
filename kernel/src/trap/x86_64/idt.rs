use basic::sync::Once;
use config::TRAMPOLINE;
use x86_64::VirtAddr;
use x86_64::structures::idt::{Entry, HandlerFunc, InterruptDescriptorTable};


const NUM_INT: usize = 256;
static IDT: Once<IdtStruct> = Once::new();

unsafe extern "C" {
    fn strampoline();
    #[link_name = "trap_handler_table"]
    static TRAP_HANDLER_TABLE: [unsafe extern "C" fn(); NUM_INT];
}

pub struct IdtStruct {
    table: InterruptDescriptorTable,
}

impl IdtStruct {
    fn new(handlers: &'static [unsafe extern "C" fn(); NUM_INT]) -> Self {
        let mut idt = Self {
            table: InterruptDescriptorTable::new(),
        };
        let entries = unsafe {
            core::slice::from_raw_parts_mut(
                &mut idt.table as *mut _ as *mut Entry<HandlerFunc>,
                NUM_INT,
            )
        };
        for i in 0..NUM_INT {
            let offset = handlers[i] as usize - strampoline as *const () as usize;
            let handler_va = VirtAddr::new((TRAMPOLINE + offset) as u64);
            let opt = unsafe { 
                entries[i].set_handler_addr(handler_va)
            };
            if i == 0x3 || i == 0x80 {
                // 允许用户态使用 int 0x3 (breakpoint) 和 int 0x80 (legacy syscall)
                opt.set_privilege_level(x86_64::PrivilegeLevel::Ring3);
            }
        }
        idt
    }

    #[inline]
    fn load(&'static self) {
        self.table.load();
    }
}


pub fn init_idt() {
    let entries = unsafe { &TRAP_HANDLER_TABLE };
    let idt_struct = IdtStruct::new(entries);
    IDT.call_once(|| idt_struct);

    set_trap_entry();
}

#[inline]
pub fn set_trap_entry() {
    IDT.get().unwrap().load();
}
