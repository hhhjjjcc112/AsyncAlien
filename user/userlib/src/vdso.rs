use core::sync::atomic::{AtomicUsize, Ordering};

use pconst::{aux::AT_SYSINFO_EHDR, time::TimeSpec};

use crate::syscall::sys_clock_gettime;

static VDSO_BASE: AtomicUsize = AtomicUsize::new(0);
static VDSO_READY: AtomicUsize = AtomicUsize::new(0);

fn is_valid_vdso_base(base: usize) -> bool {
	base >= 0x1000 && base % 0x1000 == 0
}

pub fn init_from_stack(argc_ptr: usize) {
	// 只在用户进程启动时从初始栈解析 auxv；内核只负责把 AT_SYSINFO_EHDR 写进去。
	if let Some(base) = parse_vdso_base(argc_ptr) {
		init(base);
	}
}

pub fn init(base: usize) {
	if !is_valid_vdso_base(base) {
		return;
	}

	// 只缓存用户态可见的 vDSO 基址，后续直接走生成的 ABI 入口。
	// 这个地址就是用户进程后续要传给 `vdso_api` 的入口基址。
	VDSO_BASE.store(base, Ordering::Release);
	unsafe { vdso_api::init_vdso_vtable(base as u64) };
	VDSO_READY.store(1, Ordering::Release);
}

pub fn clock_gettime_vdso(clk: usize, ts: &mut TimeSpec) -> bool {
	// 只试 vDSO 快路径；测试会用这个入口直接对照 syscall，业务路径再在上层回退。
	if VDSO_READY.load(Ordering::Acquire) != 0 && VDSO_BASE.load(Ordering::Acquire) != 0 {
		let mut vdso_ts = vdso_api::TimeSpec::default();
		let ret = vdso_api::__vdso_clock_gettime(clk, &mut vdso_ts as *mut _);
		if ret == 0 {
			ts.tv_sec = vdso_ts.tv_sec;
			ts.tv_nsec = vdso_ts.tv_nsec;
			return true;
		}
	}

	false
}

pub fn clock_gettime(clk: usize, ts: &mut TimeSpec) -> bool {
	if clock_gettime_vdso(clk, ts) {
		return true;
	}

	sys_clock_gettime(clk, ts as *mut TimeSpec as *mut u8) == 0
}

fn parse_vdso_base(argc_ptr: usize) -> Option<usize> {
	// 初始栈布局是 argc/argv/envp/auxv；这里顺着指针扫描到 AT_SYSINFO_EHDR。
	let argc = unsafe { (argc_ptr as *const usize).read_volatile() };
	let mut cursor = argc_ptr + core::mem::size_of::<usize>();

	for _ in 0..argc {
		cursor += core::mem::size_of::<usize>();
	}

	cursor += core::mem::size_of::<usize>();

	loop {
		let env = unsafe { (cursor as *const usize).read_volatile() };
		cursor += core::mem::size_of::<usize>();
		if env == 0 {
			break;
		}
	}

	loop {
		let key = unsafe { (cursor as *const usize).read_volatile() };
		let value = unsafe {
			((cursor + core::mem::size_of::<usize>()) as *const usize).read_volatile()
		};
		cursor += core::mem::size_of::<usize>() * 2;

		if key == AT_SYSINFO_EHDR {
			return Some(value);
		}
		if key == 0 {
			return None;
		}
	}
}