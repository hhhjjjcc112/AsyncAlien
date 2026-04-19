//! x86_64 的 Local APIC 与 I/O APIC 管理。

use core::mem::MaybeUninit;

use spin::{Mutex, Once};
use x2apic::ioapic::IoApic;
use x2apic::lapic::{xapic_base, LocalApic, LocalApicBuilder};

use crate::common_x86_64::boot::PHYS_VIRT_OFFSET;

pub mod vectors {
    pub const APIC_TIMER_VECTOR: u8 = 0xf0;
    pub const APIC_SPURIOUS_VECTOR: u8 = 0xf1;
    pub const APIC_ERROR_VECTOR: u8 = 0xf2;
}

static mut LOCAL_APIC: MaybeUninit<LocalApic> = MaybeUninit::uninit();
static mut IS_X2APIC: bool = false;
static IO_APIC: Once<Mutex<IoApic>> = Once::new();

fn cpu_has_x2apic() -> bool {
    raw_cpuid::CpuId::new()
        .get_feature_info()
        .map_or(false, |finfo| finfo.has_x2apic())
}

/// 初始化主核（BSP）的 Local APIC。
pub fn init_primary_apic() {
    println!("[x86_apic] init_primary_apic enter");
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
        *local_apic_slot() = MaybeUninit::new(apic);
    }

    // 初始化 I/O APIC。
    let io_apic_base = crate::qemu_x86_64::config::DEVICE_SPACE[1].1;
    println!("[x86_apic] init ioapic base={:#x}", io_apic_base);
    let io_apic = unsafe { IoApic::new((io_apic_base as u64) + (PHYS_VIRT_OFFSET as u64)) };
    IO_APIC.call_once(|| Mutex::new(io_apic));
    println!("[x86_apic] init_primary_apic ready");
}

/// 初始化从核（AP）的 APIC。
pub fn init_secondary_apic() {
    println!("[x86_apic] init_secondary_apic cpu_id={} x2apic={}", current_cpu_id(), is_x2apic());
    unsafe {
        get_local_apic().enable();
    }
    println!("[x86_apic] init_secondary_apic ready cpu_id={}", current_cpu_id());
}

fn build_local_apic() -> LocalApic {
    let mut builder = LocalApicBuilder::new();
    builder
        .spurious_vector(vectors::APIC_SPURIOUS_VECTOR as _)
        .timer_vector(vectors::APIC_TIMER_VECTOR as _)
        .error_vector(vectors::APIC_ERROR_VECTOR as _);

    if is_x2apic() {
        println!("[x86_apic] x2APIC mode enabled");
    } else {
        builder.set_xapic_base(unsafe { xapic_base() } + PHYS_VIRT_OFFSET);
        println!("[x86_apic] xAPIC mode enabled");
    }

    builder.build().unwrap()
}

unsafe fn local_apic_slot() -> *mut MaybeUninit<LocalApic> {
    core::ptr::addr_of_mut!(LOCAL_APIC)
}

/// 获取 Local APIC 可变引用。
/// 必须在 APIC 初始化后调用。
pub unsafe fn get_local_apic() -> &'static mut LocalApic {
    unsafe { &mut *(*local_apic_slot()).as_mut_ptr() }
}

/// 是否运行在 x2APIC 模式。
pub fn is_x2apic() -> bool {
    unsafe { IS_X2APIC }
}

/// 获取当前 CPU ID。
pub fn current_cpu_id() -> usize {
    crate::current_cpu_id()
}

/// 发送 APIC EOI。
pub fn eoi() {
    unsafe {
        get_local_apic().end_of_interrupt();
    }
}

/// 开关 I/O APIC 的 IRQ 路由。
pub fn set_irq_enable(vector: usize, enabled: bool) {
    // 不影响 Local APIC 自身中断。
    if vector < vectors::APIC_TIMER_VECTOR as usize {
        if let Some(ioapic) = IO_APIC.get() {
            let mut ioapic = ioapic.lock();
            unsafe {
                if enabled {
                    ioapic.enable_irq(vector as u8);
                } else {
                    ioapic.disable_irq(vector as u8);
                }
            }
        }
    }
}

/// 获取用于 IPI 目标的原始 APIC ID。
pub fn raw_apic_id(cpu_id: u8) -> u32 {
    if is_x2apic() {
        cpu_id as u32
    } else {
        (cpu_id as u32) << 24
    }
}

/// 向指定 CPU 发送 IPI。
pub fn send_ipi(target_cpu: usize, vector: u8) {
    let apic_id = raw_apic_id(target_cpu as u8);
    unsafe {
        get_local_apic().send_ipi(vector, apic_id);
    }
}

/// 向自身发送 IPI。
pub fn send_ipi_self(vector: u8) {
    unsafe {
        get_local_apic().send_ipi_self(vector);
    }
}

/// 向除自身外的所有 CPU 发送 IPI。
pub fn send_ipi_all_excluding_self(vector: u8) {
    use x2apic::lapic::IpiAllShorthand;
    unsafe {
        get_local_apic().send_ipi_all(vector, IpiAllShorthand::AllExcludingSelf);
    }
}

/// 获取 I/O APIC 最大重定向项数。
pub fn ioapic_max_entries() -> u8 {
    if let Some(ioapic) = IO_APIC.get() {
        let mut ioapic = ioapic.lock();
        unsafe { ioapic.max_table_entry() + 1 }
    } else {
        0
    }
}

/// 配置 I/O APIC 重定向项。
pub fn configure_irq(irq: u8, vector: u8, dest_cpu: u8) {
    if let Some(ioapic) = IO_APIC.get() {
        let mut ioapic = ioapic.lock();
        unsafe {
            // 配置重定向项。
            let mut entry = ioapic.table_entry(irq);
            entry.set_vector(vector);
            entry.set_dest(dest_cpu);
            // 投递模式为 Fixed，物理目标。
            entry.set_mode(x2apic::ioapic::IrqMode::Fixed);
            entry.set_flags(
                x2apic::ioapic::IrqFlags::LEVEL_TRIGGERED 
                | x2apic::ioapic::IrqFlags::LOW_ACTIVE 
                | x2apic::ioapic::IrqFlags::MASKED
            );
            ioapic.set_table_entry(irq, entry);
        }
    }
}
