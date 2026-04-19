use core::{
    arch::global_asm,
};

use config::{PERCPU_MIRROR_BASE, TRAMPOLINE};
use x86_64::{
    VirtAddr, registers::{
        control::{Efer, EferFlags},
        model_specific::{LStar, SFMask, Star},
        rflags::RFlags,
    }, structures::tss::TaskStateSegment
};

use super::{
    context::X86TrapFrame,
    user_ctx::{prepare_user_return, UserTrapResult},
};

#[unsafe(no_mangle)]
#[percpu::def_percpu]
pub static USER_RSP: usize = 0;

global_asm!(
    include_str!("syscall.asm"),
    tss_rsp0_offset = const core::mem::offset_of!(TaskStateSegment, privilege_stack_table),
);

unsafe extern "C" {
    fn strampoline();
    fn syscall_entry();
    fn _percpu_start();
}


#[unsafe(no_mangle)]
pub extern "C" fn x86_syscall_handler() -> UserTrapResult {
    let frame = super::user_ctx::current_trap_frame();

    let mut parameters = frame.parameters();
    let orig_syscall_id = parameters[0];
    let syscall_id = orig_syscall_id;
    parameters[0] = syscall_id;

    let result = crate::syscall_domain!().call(
        syscall_id,
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
    frame.update_result(res as usize);
    // SysV ABI 下以 rax/rdx 返回 user_cr3 与 trap_cx_ptr。
    let user_return = prepare_user_return();
    user_return
}

/// 处理当前 x86_64 的系统调用入口。
///
/// 兼容 `int 0x80` 的参数寄存器约定：
/// `rax=num, rbx, rcx, rdx, rsi, rdi, rbp`。
pub fn handle_legacy_syscall(frame: &mut X86TrapFrame) {
    let result = crate::syscall_domain!().call(
        frame.rax,
        [
            frame.rbx, frame.rcx, frame.rdx, frame.rsi, frame.rdi, frame.rbp,
        ],
    );
    let res = result.unwrap_or_else(|err| {
        error!("syscall error: {:?}", err);
        err as isize
    });
    frame.update_result(res as usize);
}

/// 初始化 x86_64 syscall 相关寄存器。
///
/// 当前同时支持 `syscall`(MSR/LSTAR) 与 `int 0x80`(IDT) 两条用户态入口。
pub fn init_syscall() {
    // 将 GS 指向 x86_64 percpu 镜像高地址。
    let gs_value = percpu::read_percpu_reg();
    let gs_mirror = PERCPU_MIRROR_BASE - _percpu_start as *const () as usize + gs_value;
    unsafe {
        percpu::write_percpu_reg(gs_mirror);
    }

    Star::write(
        super::gdt::GdtStruct::UCODE64_SELECTOR,
        super::gdt::GdtStruct::UDATA_SELECTOR,
        super::gdt::GdtStruct::KCODE64_SELECTOR,
        super::gdt::GdtStruct::KDATA_SELECTOR,
    )
    .expect("invalid STAR segment selectors");

    // LSTAR 指向 trampoline 虚拟地址，保证用户页表隔离场景可执行入口代码。
    let lstar_offset = syscall_entry as *const () as usize - strampoline as *const () as usize;
    let lstar = VirtAddr::new((TRAMPOLINE + lstar_offset) as u64);

    // 屏蔽 TF/IF/DF/IOPL/NT/AC，避免带入用户态标志位。
    let sfmask = RFlags::TRAP_FLAG
        | RFlags::INTERRUPT_FLAG
        | RFlags::DIRECTION_FLAG
        | RFlags::IOPL_LOW
        | RFlags::IOPL_HIGH
        | RFlags::NESTED_TASK
        | RFlags::ALIGNMENT_CHECK;

    LStar::write(lstar);
    SFMask::write(sfmask);
    unsafe {
        Efer::write(Efer::read() | EferFlags::SYSTEM_CALL_EXTENSIONS);
    }
}
