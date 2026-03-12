//! QEMU x86_64 平台支持。

pub mod config;

use core::ops::Range;
use multiboot::information::MultibootInfo;
use spin::Once;

use crate::common_x86_64::basic::MachineInfo as X86MachineInfo;
use crate::common_x86_64::{apic, time};
use crate::traits::{
    ConsoleIf, IpiTarget, IrqIf, MachineInfo, MemIf, MiscIf, PowerIf, RawRange,
    TimeIf,
};

pub static BOOT_INFO: Once<usize> = Once::new();

/// QEMU x86_64 平台类型。
pub struct QemuX86Platform;

impl ConsoleIf for QemuX86Platform {
    fn putchar(ch: u8) {
        crate::common_x86_64::services::console_putchar(ch);
    }

    fn getchar() -> Option<u8> {
        crate::common_x86_64::services::console_getchar()
    }
}

impl IrqIf for QemuX86Platform {
    const MAX_IRQ_NUM: usize = 256;

    fn set_enable(irq: usize, enabled: bool) {
        apic::set_irq_enable(irq, enabled);
    }

    fn current_irq() -> Option<usize> {
        None
    }

    fn ack_irq(_irq: usize) {
        apic::eoi();
    }

    fn send_ipi(target: IpiTarget) {
        match target {
            IpiTarget::Unicast { cpu_id } => apic::send_ipi(cpu_id, crate::qemu_x86_64::config::IPI_IRQ),
            IpiTarget::Broadcast { exclude_self } => {
                let self_id = <Self as PowerIf>::current_cpu_id();
                for cpu_id in 0..<Self as PowerIf>::cpu_count() {
                    if exclude_self && cpu_id == self_id {
                        continue;
                    }
                    apic::send_ipi(cpu_id, crate::qemu_x86_64::config::IPI_IRQ);
                }
            }
            IpiTarget::Multicast { mask, mask_base } => {
                for bit in 0..usize::BITS as usize {
                    if (mask >> bit) & 1 == 0 {
                        continue;
                    }
                    apic::send_ipi(mask_base + bit, crate::qemu_x86_64::config::IPI_IRQ);
                }
            }
        }
    }

    fn init_primary() {
        apic::init_primary_apic();
    }

    fn init_secondary(_cpu_id: usize) {
        apic::init_secondary_apic();
    }
}

impl MemIf for QemuX86Platform {
    const PHYS_VIRT_OFFSET: usize = crate::common_x86_64::boot::PHYS_VIRT_OFFSET as usize;

    fn phys_ram_ranges() -> &'static [RawRange] {
        crate::common_x86_64::mem::phys_ram_ranges()
    }

    fn reserved_ranges() -> &'static [RawRange] {
        crate::common_x86_64::mem::RESERVED_REGIONS
    }

    fn mmio_ranges() -> &'static [RawRange] {
        crate::common_x86_64::mem::mmio_ranges()
    }
}

impl PowerIf for QemuX86Platform {
    fn shutdown() -> ! {
        crate::common_x86_64::services::system_shutdown()
    }

    fn start_secondary_cpu(cpu_id: usize, start_addr: usize, opaque: usize) {
        crate::common_x86_64::services::start_secondary_cpu(cpu_id, start_addr, opaque)
    }

    fn cpu_count() -> usize {
        ::config::CPU_NUM
    }

    fn current_cpu_id() -> usize {
        arch::cpu_id()
    }

    fn halt() {
        x86_64::instructions::hlt();
    }

    fn flush_cache(cpu_mask: usize, cpu_mask_base: usize) {
        crate::common_x86_64::services::flush_cache(cpu_mask, cpu_mask_base)
    }
}

impl TimeIf for QemuX86Platform {
    fn current_ticks() -> u64 {
        time::current_ticks()
    }

    fn tick_freq() -> u64 {
        let freq = time::tsc_frequency();
        if freq == 0 {
            crate::qemu_x86_64::config::CLOCK_FREQ as u64
        } else {
            freq
        }
    }

    fn epochoffset_nanos() -> u64 {
        time::get_rtc_epoch_seconds().saturating_mul(1_000_000_000)
    }

    fn set_timer(deadline: u64) {
        crate::common_x86_64::services::set_timer(deadline as usize);
    }
}


impl MachineInfo for X86MachineInfo {
    fn memory_start(&self) -> usize {
        self.memory.start
    }

    fn memory_size(&self) -> usize {
        self.memory.end - self.memory.start
    }

    fn cpu_count(&self) -> usize {
        self.smp
    }

    fn initrd(&self) -> Option<Range<usize>> {
        self.initrd.clone()
    }

    fn bootargs(&self) -> Option<&str> {
        self.bootargs.as_ref().and_then(|args| {
            core::str::from_utf8(&args[..self.bootargs_len]).ok()
        })
    }
}


impl MiscIf for QemuX86Platform {
    type MachineInfo = X86MachineInfo;

    fn init_boot_info(ptr: usize) {
        BOOT_INFO.call_once(|| ptr);

        // 通过 ACPI 表发现外设（MADT/HPET）。
        crate::common_x86_64::acpi::init();
        for dev in crate::common_x86_64::acpi::device_list().entries.iter() {
            // 早期阶段优先直接输出，避免 logger 尚未初始化导致日志丢失。
            println!("ACPI device: {} @ {:#x} size={:#x}", dev.name, dev.base, dev.size);
        }
        
        // 初始化 APIC。
        apic::init_primary_apic();
        
        // 初始化时间子系统（TSC、RTC）。
        time::init_time();
        
        // 初始化 APIC 定时器（依赖 TSC 校准）。
        time::init_primary_apic_timer();
    }

    fn boot_info_ptr() -> usize {
        *BOOT_INFO.get().unwrap_or(&0)
    }

    fn machine_info() -> Self::MachineInfo {
        crate::common_x86_64::basic::machine_info_from_boot_info(*BOOT_INFO.get().unwrap_or(&0))
    }
}

#[allow(dead_code)]
pub fn init_boot_info(boot_info: usize) {
    <QemuX86Platform as MiscIf>::init_boot_info(boot_info);
}

#[deprecated(note = "use init_boot_info")]
#[allow(dead_code)]
pub fn init_dtb(boot_info: Option<usize>) {
    init_boot_info(boot_info.unwrap_or(0));
}

#[allow(dead_code)]
pub fn boot_info_ptr() -> usize {
    <QemuX86Platform as MiscIf>::boot_info_ptr()
}

#[allow(dead_code)]
pub fn basic_machine_info() -> X86MachineInfo {
    <QemuX86Platform as MiscIf>::machine_info()
}

#[allow(dead_code)]
pub fn set_timer(time: usize) {
    crate::common_x86_64::services::set_timer(time);
}

#[allow(dead_code)]
pub fn system_shutdown() -> ! {
    <QemuX86Platform as PowerIf>::shutdown()
}

#[allow(dead_code)]
pub fn console_putchar(ch: u8) {
    <QemuX86Platform as ConsoleIf>::putchar(ch);
}

#[allow(dead_code)]
pub fn start_secondary_cpu(cpu_id: usize, start_addr: usize, opaque: usize) {
    <QemuX86Platform as PowerIf>::start_secondary_cpu(cpu_id, start_addr, opaque)
}

#[allow(dead_code)]
pub fn flush_cache(cpu_mask: usize, cpu_mask_base: usize) {
    crate::common_x86_64::services::flush_cache(cpu_mask, cpu_mask_base)
}

