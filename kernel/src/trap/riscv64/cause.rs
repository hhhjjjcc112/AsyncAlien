use riscv::register::{
    scause::{Exception, Interrupt, Trap},
    sepc, stval,
};

use crate::{plic_domain, task_domain, timer};

pub trait TrapHandler {
    fn do_user_handle(&self);
    fn do_kernel_handle(&self, sp: usize);
}

impl TrapHandler for Trap {
    fn do_user_handle(&self) {
        let stval = stval::read();
        let sepc = sepc::read();
        log::debug!("trap: {:?}", self);

        match self {
            Trap::Exception(Exception::UserEnvCall) => {
                super::super::exception::syscall_exception_handler();
            }
            Trap::Exception(Exception::StoreFault)
            | Trap::Exception(Exception::LoadFault)
            | Trap::Exception(Exception::InstructionFault)
            | Trap::Exception(Exception::IllegalInstruction) => {
                panic!(
                    "<do_user_handle> {:?} in application, stval:{:#x?} sepc:{:#x?}",
                    self, stval, sepc
                );
            }
            Trap::Exception(Exception::StorePageFault)
            | Trap::Exception(Exception::LoadPageFault) => {
                task_domain!()
                    .do_load_page_fault(stval)
                    .expect("do_load_page_fault failed");
                log::debug!(
                    "<do_user_handle> {:?}, stval:{:#x?} sepc:{:#x?}",
                    self, stval, sepc
                );
            }
            Trap::Exception(Exception::InstructionPageFault) => {
                panic!("<do_user_handle> instruction page fault")
            }
            Trap::Interrupt(Interrupt::SupervisorTimer) => {
                trace!("<do_user_handle> timer interrupt");
                timer::set_next_trigger();
                crate::task::yield_now();
            }
            Trap::Interrupt(Interrupt::SupervisorExternal) => {
                trace!("[{}] <do_user_handle> external interrupt", arch::cpu_id());
                plic_domain!().handle_irq().expect("handle_irq failed");
            }
            _ => {
                panic!(
                    "unhandled trap: {:?}, stval: {:?}, sepc: {:x}",
                    self, stval, sepc
                );
            }
        }
    }

    fn do_kernel_handle(&self, _sp: usize) {
        let stval = stval::read();
        let sepc = sepc::read();

        match self {
            Trap::Interrupt(Interrupt::SupervisorTimer) => {
                trace!("<do_kernel_handle> timer interrupt");
                timer::set_next_trigger()
            }
            Trap::Exception(_) => {
                panic!(
                    "[kernel] {:?} in kernel, stval:{:#x?} sepc:{:#x?}",
                    self, stval, sepc
                );
            }
            Trap::Interrupt(Interrupt::SupervisorExternal) => {
                platform::println!("<do_kernel_handle> external interrupt");
                plic_domain!().handle_irq().expect("handle_irq failed");
            }
            _ => {
                panic!(
                    "unhandled trap: {:?}, stval: {:?}, sepc: {:x}",
                    self, stval, sepc
                )
            }
        }
    }
}
