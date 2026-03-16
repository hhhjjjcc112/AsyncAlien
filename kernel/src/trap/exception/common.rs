use basic::{sync::OnceGet, task::TrapFrame};
use mem::PhysAddr;

use crate::{syscall_domain, task_domain};

#[inline]
pub(super) fn with_current_trap_frame(f: impl FnOnce(&mut TrapFrame)) {
    let task_domain = task_domain!();
    let trap_frame_phy_addr = task_domain.trap_frame_phy_addr().unwrap();
    let trap_frame = TrapFrame::from_raw_phy_ptr(PhysAddr::from(trap_frame_phy_addr));
    f(trap_frame);
}

#[inline]
pub(super) fn dispatch_syscall(cx: &mut TrapFrame) {
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
