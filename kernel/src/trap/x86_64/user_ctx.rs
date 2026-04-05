use mem::PhysAddr;
use x86_64::VirtAddr;
use x86_64::registers::model_specific::{FsBase, KernelGsBase};

use crate::task_domain;

use super::context::X86TrapFrame;

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

    // 统一在用户态返回前同步 TLS 基址，保证 arch_prctl 设置立即生效。
    let fs_base = task_domain.do_get_fs_base().unwrap_or_else(|err| {
        error!("x86_64 trap: do_get_fs_base failed: {:?}", err);
        panic!("x86_64 trap: failed to get fs base");
    });
    let gs_base = task_domain.do_get_gs_base().unwrap_or_else(|err| {
        error!("x86_64 trap: do_get_gs_base failed: {:?}", err);
        panic!("x86_64 trap: failed to get gs base");
    });
    FsBase::write(VirtAddr::new(fs_base as u64));
    KernelGsBase::write(VirtAddr::new(gs_base as u64));

    // 返回用户态前刷新 TSS.rsp0，确保下一次 CPL3->0 入栈落在当前任务 TrapFrame。
    crate::trap::write_tss_rsp0(trap_cx_ptr + X86TrapFrame::USER_CONTEXT_SIZE);

    UserTrapResult {
        user_cr3,
        trap_cx_ptr,
    }
}
