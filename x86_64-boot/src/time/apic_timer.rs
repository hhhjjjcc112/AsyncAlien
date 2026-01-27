use crate::apic::get_local_apic;

pub fn init_primary_apic_timer() {
    let local_apic = unsafe { get_local_apic() };
    unsafe { 
        local_apic.set_timer_divide(x2apic::lapic::TimerDivide::Div1);
        local_apic.set_timer_mode(x2apic::lapic::TimerMode::OneShot);
        local_apic.set_timer_initial(0xFFFFFFFF);
        local_apic.enable_timer();
    };

}

pub fn init_secondary_apic_timer() {
    unsafe {
        get_local_apic().enable_timer();
    }
}