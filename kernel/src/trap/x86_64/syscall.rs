use core::{
    arch::global_asm,
};

use config::{PERCPU_MIRROR_BASE, TRAMPOLINE};
use percpu::read_percpu_reg;
use platform::percpu_impl::cpu_id;
use x86_64::{
    registers::{
        control::{Efer, EferFlags},
        model_specific::{GsBase, KernelGsBase, LStar, SFMask, Star},
        rflags::RFlags,
    },
    structures::tss::TaskStateSegment,
    VirtAddr,
};

use super::{
    context::X86TrapFrame,
    user_ctx::{current_trap_frame, current_trap_state, prepare_user_return, UserTrapResult},
};
use crate::task::{
    current_tid,
    should_trace_tid, trace_current_state,
};

#[unsafe(no_mangle)]
#[percpu::def_percpu]
static USER_RSP: usize = 0;
#[unsafe(no_mangle)]
#[percpu::def_percpu]
static SYSCALL_TRACE_ARMED: usize = 0;
#[unsafe(no_mangle)]
#[percpu::def_percpu]
static SYSCALL_TRACE_STAGE: usize = 0;
#[unsafe(no_mangle)]
#[percpu::def_percpu]
static SYSCALL_TRACE_RIP: usize = 0;
#[unsafe(no_mangle)]
#[percpu::def_percpu]
static SYSCALL_TRACE_RSP0: usize = 0;
#[percpu::def_percpu]
static SYSCALL_TRACE_BUDGET: usize = 0;

global_asm!(
    include_str!("syscall.asm"),
    tss_rsp0_offset = const core::mem::offset_of!(TaskStateSegment, privilege_stack_table),
);

unsafe extern "C" {
    fn strampoline();
    fn syscall_entry();
}

#[inline]
fn syscall_trace_armed() -> bool {
    SYSCALL_TRACE_ARMED.read_current() != 0
}

pub fn arm_syscall_entry_trace() {
    SYSCALL_TRACE_ARMED.write_current(1);
    SYSCALL_TRACE_STAGE.write_current(0);
    SYSCALL_TRACE_RIP.write_current(0);
    SYSCALL_TRACE_RSP0.write_current(0);
    SYSCALL_TRACE_BUDGET.write_current(4);
}

#[inline]
fn consume_syscall_trace_budget() -> bool {
    let budget = SYSCALL_TRACE_BUDGET.read_current();
    if budget == 0 {
        return false;
    }
    SYSCALL_TRACE_BUDGET.write_current(budget - 1);
    true
}

pub fn drain_syscall_entry_trace(label: &str) {
    if !syscall_trace_armed() {
        return;
    }

    let stage = SYSCALL_TRACE_STAGE.read_current();
    if stage != 1 {
        return;
    }

    let rip = SYSCALL_TRACE_RIP.read_current();
    let rsp0 = SYSCALL_TRACE_RSP0.read_current();
    println!(
        "[x86 syscall early] label={} stage=asm_only cpu={} tid={:?} rip={:#x} rsp0={:#x} gs={:#x} kgs={:#x} percpu={:#x}",
        label,
        cpu_id(),
        current_tid(),
        rip,
        rsp0,
        GsBase::read().as_u64() as usize,
        KernelGsBase::read().as_u64() as usize,
        read_percpu_reg(),
    );

    SYSCALL_TRACE_ARMED.write_current(0);
    SYSCALL_TRACE_STAGE.write_current(0);
}

#[unsafe(no_mangle)]
pub extern "C" fn x86_syscall_entry_stage2(user_rip: usize, kernel_rsp: usize, kernel_cr3: usize) {
    if !syscall_trace_armed() {
        return;
    }

    SYSCALL_TRACE_STAGE.write_current(2);
    SYSCALL_TRACE_RIP.write_current(user_rip);
    SYSCALL_TRACE_RSP0.write_current(kernel_rsp);

    println!(
        "[x86 syscall early] label=stage2 cpu={} tid={:?} rip={:#x} rsp0={:#x} kernel_rsp={:#x} kernel_cr3={:#x} gs={:#x} kgs={:#x} percpu={:#x}",
        cpu_id(),
        current_tid(),
        user_rip,
        kernel_rsp,
        kernel_rsp,
        kernel_cr3,
        GsBase::read().as_u64() as usize,
        KernelGsBase::read().as_u64() as usize,
        read_percpu_reg(),
    );

    SYSCALL_TRACE_ARMED.write_current(0);
    SYSCALL_TRACE_STAGE.write_current(0);
}

#[unsafe(no_mangle)]
pub extern "C" fn x86_syscall_handler() -> UserTrapResult {
    let frame = current_trap_frame();

    let mut parameters = frame.parameters();
    let orig_syscall_id = parameters[0];
    let syscall_id = orig_syscall_id;
    parameters[0] = syscall_id;

    if should_trace_tid(current_tid()) && consume_syscall_trace_budget() {
        trace_current_state("syscall_entry", current_trap_state(frame));
        println!(
            "[x86 syscall args] cpu={} tid={:?} orig={:#x} id={:#x} args={:#x?}",
            cpu_id(),
            current_tid(),
            orig_syscall_id,
            syscall_id,
            &parameters[1..]
        );
    }
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
    if should_trace_tid(current_tid()) && consume_syscall_trace_budget() {
        trace_current_state("syscall_return", current_trap_state(frame));
        println!(
            "[x86 syscall result] cpu={} tid={:?} id={:#x} result={}",
            cpu_id(),
            current_tid(),
            syscall_id,
            res,
        );
    }
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
    // 每核槽位大小由 percpu 统一计算，避免手写偏移。
    let cpu_id = cpu_id();
    let slot_size = percpu::percpu_area_layout_expected(1).size();
    let gs_mirror = PERCPU_MIRROR_BASE + cpu_id * slot_size;
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
