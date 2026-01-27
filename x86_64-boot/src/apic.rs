use core::mem::MaybeUninit;

use x2apic::lapic::{LocalApic, LocalApicBuilder, xapic_base};
use x86_64::instructions::port::Port;

use crate::{apic::vectors::{APIC_ERROR_VECTOR, APIC_SPURIOUS_VECTOR, APIC_TIMER_VECTOR}, boot::PHYS_VIRT_OFFSET, println};

pub(super) mod vectors {
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

pub fn init_primary_apic() {
    println!("Initializing Primary APIC...");
    let is_x2apic = cpu_has_x2apic();
    unsafe {
        IS_X2APIC = is_x2apic;
        // 禁用8259A中断控制器
        Port::<u8>::new(0x21).write(0xff);
        Port::<u8>::new(0xa1).write(0xff);
    }

    let mut builder = LocalApicBuilder::new();
    builder
        .spurious_vector(APIC_SPURIOUS_VECTOR as _)
        .timer_vector(APIC_TIMER_VECTOR as _)
        .error_vector(APIC_ERROR_VECTOR as _);

    if is_x2apic {
        println!("x2APIC mode enabled.");
    } else {
        builder.set_xapic_base(unsafe { xapic_base() } + PHYS_VIRT_OFFSET);
        println!("xAPIC mode enabled.");
    }
    let mut apic = builder.build().unwrap();
    unsafe {
        apic.enable();
        #[allow(static_mut_refs)]
        LOCAL_APIC.write(apic);
    }
}

pub fn init_secondary_apic() {
    unsafe {
        get_local_apic().enable();
    }
}

pub unsafe fn get_local_apic() -> &'static mut LocalApic {
    #[allow(static_mut_refs)]
    unsafe { LOCAL_APIC.assume_init_mut() }
}

pub fn is_x2apic() -> bool {
    unsafe { IS_X2APIC }
}



