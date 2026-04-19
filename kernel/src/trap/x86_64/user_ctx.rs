use mem::PhysAddr;

use super::context::X86TrapFrame;
use crate::{
    task::{
        current_tid,
        should_trace_tid, trace_current_state, X86StateTrace,
    },
    task_domain,
};

#[repr(C)]
pub struct UserTrapResult {
    pub user_cr3: usize,
    pub trap_cx_ptr: usize,
}

#[percpu::def_percpu]
static USER_RETURN_TRACE_BUDGET: usize = 0;

#[inline]
pub(super) fn consume_user_return_trace_budget() -> bool {
    let budget = USER_RETURN_TRACE_BUDGET.read_current();
    if budget == 0 {
        return false;
    }
    USER_RETURN_TRACE_BUDGET.write_current(budget - 1);
    true
}

#[inline]
pub fn arm_user_return_trace(budget: usize) {
    USER_RETURN_TRACE_BUDGET.write_current(budget);
}

/// 获取当前任务的 TrapFrame（通过物理地址，避免依赖用户页表映射）。
#[inline]
pub fn current_trap_frame() -> &'static mut X86TrapFrame {
    let task_domain = task_domain!();
    let trap_frame_phy_addr = task_domain.trap_frame_phy_addr().unwrap_or_else(|err| {
        error!("x86_64 trap: trap_frame_phy_addr failed: {:?}", err);
        panic!("x86_64 trap: no trap frame for current task");
    });
    X86TrapFrame::from_raw_phy_ptr(PhysAddr::from(trap_frame_phy_addr))
}

#[inline]
pub fn current_trap_state(frame: &X86TrapFrame) -> X86StateTrace {
    let task_domain = task_domain!();
    let trap_frame_phy_addr = task_domain.trap_frame_phy_addr().unwrap_or_else(|err| {
        error!("x86_64 trap: trap_frame_phy_addr failed: {:?}", err);
        panic!("x86_64 trap: no trap frame for current task");
    });
    let (user_cr3, trap_cx_ptr) = task_domain
        .page_table_token_with_trap_frame_virt_addr()
        .unwrap_or_else(|err| {
            error!(
                "x86_64 trap: page_table_token_with_trap_frame_virt_addr failed: {:?}",
                err
            );
            panic!("x86_64 trap: no user return state for current task");
        });
    X86StateTrace::from_frame(frame, trap_frame_phy_addr, trap_cx_ptr, user_cr3)
}

/// 构造“返回用户态”所需参数。
#[inline]
pub fn prepare_user_return() -> UserTrapResult {
    let task_domain = task_domain!();
    let (user_cr3, trap_cx_ptr) = task_domain
        .page_table_token_with_trap_frame_virt_addr()
        .unwrap_or_else(|err| {
            error!(
                "x86_64 trap: page_table_token_with_trap_frame_virt_addr failed: {:?}",
                err
            );
            panic!("x86_64 trap: failed to prepare user return");
        });

    // 返回用户态前刷新 TSS.rsp0，确保下一次 CPL3->0 入栈落在当前任务 TrapFrame。
    let written_rsp0 = trap_cx_ptr + X86TrapFrame::USER_CONTEXT_SIZE;
    crate::trap::write_tss_rsp0(written_rsp0);

    if should_trace_tid(current_tid()) && consume_user_return_trace_budget() {
        let trap_frame_phy_addr = task_domain.trap_frame_phy_addr().unwrap_or_else(|err| {
            error!("x86_64 trap: trap_frame_phy_addr failed: {:?}", err);
            panic!("x86_64 trap: no trap frame for current task");
        });
        let frame = X86TrapFrame::from_raw_phy_ptr(PhysAddr::from(trap_frame_phy_addr));
        trace_current_state(
            "prepare_user_return",
            X86StateTrace::from_frame(frame, trap_frame_phy_addr, trap_cx_ptr, user_cr3),
        );
    }

    UserTrapResult {
        user_cr3,
        trap_cx_ptr,
    }
}
