use basic::task::TrapFrame;
use mem::PhysAddr;
use riscv::register::{
    scause::{Exception, Interrupt, Trap},
    sepc, stval,
};

use crate::{interrupt_controller_domain, syscall_domain, task_domain, timer};

#[inline]
fn handle_user_syscall() {
    let task_domain = task_domain!();
    let trap_frame_phy_addr = task_domain.trap_frame_phy_addr().unwrap();
    let cx = TrapFrame::from_raw_phy_ptr(PhysAddr::from(trap_frame_phy_addr));

    // ecall 返回前前移用户 PC，避免重复陷入。
    cx.update_user_pc(cx.user_pc() + 4);

    let parameters = cx.parameters();
    let result = syscall_domain!().call(
        parameters[0],
        [
            parameters[1],
            parameters[2],
            parameters[3],
            parameters[4],
            parameters[5],
            parameters[6],
        ],
    );
    let res = result.unwrap_or_else(|err| {
        error!("syscall error: {:?}", err);
        err as isize
    });
    cx.update_result(res as usize);
}

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
                handle_user_syscall();
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
                    self,
                    stval,
                    sepc
                );
            }
            Trap::Exception(Exception::InstructionPageFault) => {
                panic!("<do_user_handle> instruction page fault")
            }
            Trap::Interrupt(Interrupt::SupervisorTimer) => {
                trace!("<do_user_handle> timer interrupt");
                timer::set_next_trigger();
                crate::vdso::refresh_time_snapshot();
                crate::task::yield_now();
            }
            Trap::Interrupt(Interrupt::SupervisorExternal) => {
                trace!("[{}] <do_user_handle> external interrupt", platform::percpu_impl::cpu_id());
                interrupt_controller_domain!()
                    .handle_irq()
                    .expect("handle_irq failed");
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
                timer::set_next_trigger();
                crate::vdso::refresh_time_snapshot();
            }
            Trap::Exception(_) => {
                panic!(
                    "[kernel] {:?} in kernel, stval:{:#x?} sepc:{:#x?}",
                    self, stval, sepc
                );
            }
            Trap::Interrupt(Interrupt::SupervisorExternal) => {
                platform::println!("<do_kernel_handle> external interrupt");
                interrupt_controller_domain!()
                    .handle_irq()
                    .expect("handle_irq failed");
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
