use core::sync::atomic::{AtomicUsize, Ordering};

use mem::PhysAddr;

use super::context::X86TrapFrame;
use crate::{task::current_tid, task_domain};
use platform::percpu_impl::cpu_id;

static PREPARE_USER_RETURN_TRACE_COUNT: AtomicUsize = AtomicUsize::new(0);

#[repr(C)]
pub struct UserTrapResult {
    pub user_cr3: usize,
    pub trap_cx_ptr: usize,
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
    let trace_idx = PREPARE_USER_RETURN_TRACE_COUNT.fetch_add(1, Ordering::Relaxed);
    if trace_idx < 16 {
        log::warn!(
            "[x86 prepare_user_return] cpu={} tid={:?} trap_cx={:#x} rsp0={:#x}",
            cpu_id(),
            current_tid(),
            trap_cx_ptr,
            written_rsp0,
        );
    }

    UserTrapResult {
        user_cr3,
        trap_cx_ptr,
    }
}
