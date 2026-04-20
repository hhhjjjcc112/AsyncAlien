#![no_std]

//! vDSO 实现库。
//!
//! 这里保存的是“真正会被映射到用户态执行”的代码：
//! - `vvar_data!` 定义共享快照区，内核负责写入，vDSO 只读。
//! - `read_clock_timespec()` 通过 `seq` 序号做无锁快照读取。
//! - `api` 模块导出 C ABI 函数，供用户态通过 `AT_SYSINFO_EHDR` 找到后直接调用。

use core::sync::atomic::{AtomicUsize, Ordering};
use vdso_helper::{get_vvar_data, vvar_data};

pub mod api;
pub mod interface;

pub use api::*;

pub const CLOCK_REALTIME: usize = 0;
pub const CLOCK_MONOTONIC: usize = 1;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TimeSpec {
	pub tv_sec: usize,
	pub tv_nsec: usize,
}

// 这块共享数据会被内核映射到用户进程的 vVAR 区，vDSO 代码和内核都要按同一布局访问。
vvar_data! {
	seq: usize,
	realtime_ns: usize,
	monotonic_ns: usize,
	shared_counter: AtomicUsize,
}

/// 私有数据，放在 vDSO 自身的数据段里。
pub(crate) static PRIVATE_COUNTER: AtomicUsize = AtomicUsize::new(1);

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct LayoutProbe {
	pub data_base: usize,
	pub private_value: usize,
	pub shared_value: usize,
}

pub(crate) fn bump_layout_counters() {
	get_vvar_data!(shared_counter).fetch_add(1, Ordering::Relaxed);
	PRIVATE_COUNTER.fetch_add(1, Ordering::Relaxed);
}

pub(crate) fn read_layout_probe() -> LayoutProbe {
	let shared_counter = get_vvar_data!(shared_counter);
	LayoutProbe {
		data_base: core::ptr::addr_of!(PRIVATE_COUNTER) as usize,
		private_value: PRIVATE_COUNTER.load(Ordering::Relaxed),
		shared_value: shared_counter.load(Ordering::Relaxed),
	}
}

pub(crate) fn read_clock_timespec(clock_id: usize) -> Option<TimeSpec> {
	// 采用 seqcount 风格读取：偶数表示快照稳定，奇数表示内核正在更新。
	// 这样用户态可以在不陷入 syscall 的情况下，直接读到一组一致的时间值。
	macro_rules! read_snapshot {
		($field:ident) => {{
			loop {
				let seq = unsafe { core::ptr::read_volatile(get_vvar_data!(seq)) };
				if seq & 1 != 0 {
					core::hint::spin_loop();
					continue;
				}
				let nanos = unsafe { core::ptr::read_volatile(get_vvar_data!($field)) };
				let seq_after = unsafe { core::ptr::read_volatile(get_vvar_data!(seq)) };
				if seq == seq_after {
					break Some(TimeSpec {
						tv_sec: nanos / 1_000_000_000,
						tv_nsec: nanos % 1_000_000_000,
					});
				}
			}
		}};
	}

	match clock_id {
		CLOCK_REALTIME => read_snapshot!(realtime_ns),
		CLOCK_MONOTONIC => read_snapshot!(monotonic_ns),
		_ => None,
	}
}