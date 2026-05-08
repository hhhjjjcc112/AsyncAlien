//! x86_64 的 Local APIC 早期初始化与底层访问。

use core::sync::atomic::{AtomicBool, Ordering};
use spin::Mutex;
use x86_apic::LocalApicContext;

use crate::common_x86_64::boot::PHYS_VIRT_OFFSET;

pub mod vectors {
    // Vectors are defined in domain-lib/config and exposed via the `config` crate.
    // Keep this small compatibility module to avoid rippling changes elsewhere.
    pub use config::{APIC_TIMER_VECTOR, APIC_SPURIOUS_VECTOR, APIC_ERROR_VECTOR};
}

static APIC_CONTEXT: Mutex<Option<LocalApicContext>> = Mutex::new(None);
static APIC_CONTEXT_READY: AtomicBool = AtomicBool::new(false);
static IS_X2APIC: AtomicBool = AtomicBool::new(false);

fn cpu_has_x2apic() -> bool {
    raw_cpuid::CpuId::new()
        .get_feature_info()
        .map_or(false, |finfo| finfo.has_x2apic())
}

/// 初始化主核（BSP）的 Local APIC。
pub fn init_primary_apic() {
    // 禁用 8259A PIC
    let _ = x86_apic::port_io::disable_8259a();

    let is_x2apic_mode = cpu_has_x2apic();
    IS_X2APIC.store(is_x2apic_mode, Ordering::Release);
    
    let xapic_base = if is_x2apic_mode {
        0  // x2APIC 不需要 MMIO 基地址
    } else {
        (unsafe { x2apic::lapic::xapic_base() as usize }) + (PHYS_VIRT_OFFSET as usize)
    };

    let mut context = LocalApicContext::new(xapic_base, is_x2apic_mode)
        .expect("failed to create LocalApicContext");
    
    // 启用 APIC
    context.enable().expect("failed to enable APIC");
    
    *APIC_CONTEXT.lock() = Some(context);
    APIC_CONTEXT_READY.store(true, Ordering::Release);
}

/// 初始化从核（AP）的 APIC。
pub fn init_secondary_apic() {
    if let Some(ctx) = APIC_CONTEXT.lock().as_mut() {
        let _ = ctx.enable();
    }
}

/// 获取 Local APIC Context 可变引用。
/// 必须在 APIC 初始化后调用。
pub fn get_local_apic() -> Option<spin::MutexGuard<'static, Option<LocalApicContext>>> {
    if APIC_CONTEXT_READY.load(Ordering::Acquire) {
        Some(APIC_CONTEXT.lock())
    } else {
        None
    }
}

/// 是否运行在 x2APIC 模式
pub fn is_x2apic() -> bool {
    IS_X2APIC.load(Ordering::Acquire)
}

/// 是否已初始化
pub fn is_initialized() -> bool {
    APIC_CONTEXT_READY.load(Ordering::Acquire)
}

