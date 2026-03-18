use core::{arch::asm, mem::MaybeUninit};

use arch::cpu_id;
use config::CPU_NUM;
use x86_64::PrivilegeLevel;
use x86_64::instructions::segmentation::CS;
use x86_64::instructions::tables::load_tss;
use x86_64::registers::segmentation::{DS, ES, FS, GS, SS, Segment, SegmentSelector};
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

struct Selectors {
    kcode: SegmentSelector,
    kdata: SegmentSelector,
    tss: SegmentSelector,
}

static mut GDT_PER_CPU: [MaybeUninit<GlobalDescriptorTable>; CPU_NUM] =
    [const { MaybeUninit::uninit() }; CPU_NUM];
static mut TSS_PER_CPU: [MaybeUninit<TaskStateSegment>; CPU_NUM] =
    [const { MaybeUninit::uninit() }; CPU_NUM];
static mut SELECTORS_PER_CPU: [MaybeUninit<Selectors>; CPU_NUM] =
    [const { MaybeUninit::uninit() }; CPU_NUM];
static mut INIT_DONE: [bool; CPU_NUM] = [false; CPU_NUM];

#[inline]
fn current_rsp() -> usize {
    let rsp: usize;
    unsafe {
        asm!("mov {}, rsp", out(reg) rsp, options(nomem, preserves_flags));
    }
    rsp
}

#[inline]
fn init_current_cpu_gdt_tss(cpu: usize) {
    unsafe {
        let mut tss = TaskStateSegment::new();
        tss.privilege_stack_table[0] = VirtAddr::new_truncate(current_rsp() as u64);
        TSS_PER_CPU[cpu].write(tss);

        let mut gdt = GlobalDescriptorTable::new();
        // 顺序保持与现有 TrapFrame 段选择子一致：
        // user CS=0x23(index=4), user SS=0x1b(index=3), tss=0x28(index=5)。
        let kcode = gdt.add_entry(Descriptor::kernel_code_segment());
        let kdata = gdt.add_entry(Descriptor::kernel_data_segment());
        let _udata = gdt.add_entry(Descriptor::user_data_segment());
        let _ucode = gdt.add_entry(Descriptor::user_code_segment());
        let tss_sel = gdt.add_entry(Descriptor::tss_segment(
            TSS_PER_CPU[cpu].assume_init_ref(),
        ));

        GDT_PER_CPU[cpu].write(gdt);
        SELECTORS_PER_CPU[cpu].write(Selectors {
            kcode,
            kdata,
            tss: tss_sel,
        });
        INIT_DONE[cpu] = true;
    }
}

/// 初始化本核 GDT/TSS，并装载 TR。
pub fn init_gdt() {
    unsafe {
        let cpu = cpu_id();
        if !INIT_DONE[cpu] {
            init_current_cpu_gdt_tss(cpu);
        }

        let gdt = &*(&raw const GDT_PER_CPU[cpu]).cast::<GlobalDescriptorTable>();
        let selectors = &*(&raw const SELECTORS_PER_CPU[cpu]).cast::<Selectors>();

        gdt.load();
        CS::set_reg(selectors.kcode);
        SS::set_reg(selectors.kdata);
        DS::set_reg(selectors.kdata);
        ES::set_reg(selectors.kdata);
        let null_sel = SegmentSelector::new(0, PrivilegeLevel::Ring0);
        FS::set_reg(null_sel);
        GS::set_reg(null_sel);
        load_tss(selectors.tss);
    }
}

#[inline]
pub fn write_tss_rsp0(rsp0: usize) {
    unsafe {
        let cpu = cpu_id();
        assert!(INIT_DONE[cpu], "write_tss_rsp0 before init_gdt on cpu {}", cpu);
        let tss = &mut *(&raw mut TSS_PER_CPU[cpu]).cast::<TaskStateSegment>();
        tss.privilege_stack_table[0] = VirtAddr::new_truncate(rsp0 as u64);
    }
}
