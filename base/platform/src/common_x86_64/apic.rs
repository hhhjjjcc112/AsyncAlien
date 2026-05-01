//! x86_64 的 Local APIC 早期初始化与底层访问。

use core::mem::MaybeUninit;

use x2apic::lapic::{xapic_base, LocalApic, LocalApicBuilder};

use crate::common_x86_64::boot::PHYS_VIRT_OFFSET;

pub mod vectors {
    pub const APIC_TIMER_VECTOR: u8 = 0xf0;
    pub const APIC_SPURIOUS_VECTOR: u8 = 0xf1;
    pub const APIC_ERROR_VECTOR: u8 = 0xf2;
}

static mut LOCAL_APIC: MaybeUninit<LocalApic> = MaybeUninit::uninit();
static mut IS_X2APIC: bool = false;

fn cpu_has_x2apic() -> bool {
    raw_cpuid::CpuId::new()
        .get_feature_info()
        .map_or(false, |finfo| finfo.has_x2apic())
}

/// 初始化主核（BSP）的 Local APIC。
pub fn init_primary_apic() {
    let is_x2apic = cpu_has_x2apic();
    unsafe {
        IS_X2APIC = is_x2apic;
        // 关闭 8259A PIC。
        core::arch::asm!(
            "out dx, al",
            in("dx") 0x21_u16,
            in("al") 0xff_u8,
            options(nomem, nostack, preserves_flags)
        );
        core::arch::asm!(
            "out dx, al",
            in("dx") 0xa1_u16,
            in("al") 0xff_u8,
            options(nomem, nostack, preserves_flags)
        );
    }

    let mut apic = build_local_apic();
    unsafe {
        apic.enable();
        #[allow(static_mut_refs)]
        LOCAL_APIC.write(apic);
    }

}

/// 初始化从核（AP）的 APIC。
pub fn init_secondary_apic() {
    unsafe {
        get_local_apic().enable();
    }
}

fn build_local_apic() -> LocalApic {
    let mut builder = LocalApicBuilder::new();
    builder
        .spurious_vector(vectors::APIC_SPURIOUS_VECTOR as _)
        .timer_vector(vectors::APIC_TIMER_VECTOR as _)
        .error_vector(vectors::APIC_ERROR_VECTOR as _);

    if is_x2apic() {
    } else {
        builder.set_xapic_base(unsafe { xapic_base() } + PHYS_VIRT_OFFSET);
    }

    builder.build().unwrap()
}

/// 获取 Local APIC 可变引用。
/// 必须在 APIC 初始化后调用。
pub unsafe fn get_local_apic() -> &'static mut LocalApic {
    #[allow(static_mut_refs)]
    unsafe { LOCAL_APIC.assume_init_mut() }
}

/// 是否运行在 x2APIC 模式。
pub fn is_x2apic() -> bool {
    unsafe { IS_X2APIC }
}


