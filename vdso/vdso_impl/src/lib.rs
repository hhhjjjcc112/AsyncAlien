#![no_std]

//! vDSO 实现库，保留 gettimeofday，并附带布局探针。

use core::sync::atomic::{AtomicUsize, Ordering};
use vdso_helper::{get_vvar_data, vvar_data};
use vdso_helper::vvar_data::get_code_base;

pub mod api;
pub mod interface;

pub use api::*;

vvar_data! {
	shared_counter: AtomicUsize,
}

/// 私有数据，放在 vDSO 自身的数据段里。
pub(crate) static PRIVATE_COUNTER: AtomicUsize = AtomicUsize::new(1);

/// 仅用于验证打包布局的影子 vVAR 片段。
#[used]
#[unsafe(link_section = ".vvar_data")]
static VVAR_LAYOUT_PROBE: [u8; 16] = *b"vdso-vvar-probe!";

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct LayoutProbe {
	pub code_base: usize,
	pub data_base: usize,
	pub vvar_shadow_base: usize,
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
		code_base: get_code_base(0x1000),
		data_base: core::ptr::addr_of!(PRIVATE_COUNTER) as usize,
		vvar_shadow_base: core::ptr::addr_of!(VVAR_LAYOUT_PROBE) as usize,
		private_value: PRIVATE_COUNTER.load(Ordering::Relaxed),
		shared_value: shared_counter.load(Ordering::Relaxed),
	}
}