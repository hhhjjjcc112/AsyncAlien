
use mem::PhysAddr;

use super::context::X86TrapFrame;
use crate::task_domain;

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

    UserTrapResult {
        user_cr3,
        trap_cx_ptr,
    }
}
