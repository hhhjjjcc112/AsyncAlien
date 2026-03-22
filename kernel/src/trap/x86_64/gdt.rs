use x86_64::instructions::segmentation::CS;
use x86_64::instructions::tables::{lgdt, load_tss};
use x86_64::registers::segmentation::{SS, Segment, SegmentSelector};
use x86_64::structures::gdt::{Descriptor, DescriptorFlags};
use x86_64::structures::{tss::TaskStateSegment, DescriptorTablePointer};
use x86_64::{PrivilegeLevel, VirtAddr};

#[unsafe(no_mangle)]
#[percpu::def_percpu]
static TSS: TaskStateSegment = TaskStateSegment::new();

#[percpu::def_percpu]
static GDT: GdtStruct = GdtStruct::empty();

#[repr(align(16))]
pub struct GdtStruct {
    table: [u64; 16],
}

impl GdtStruct {
    pub const KCODE64_SELECTOR: SegmentSelector = SegmentSelector::new(2, PrivilegeLevel::Ring0);
    pub const KDATA_SELECTOR: SegmentSelector = SegmentSelector::new(3, PrivilegeLevel::Ring0);
    pub const UDATA_SELECTOR: SegmentSelector = SegmentSelector::new(5, PrivilegeLevel::Ring3);
    pub const UCODE64_SELECTOR: SegmentSelector = SegmentSelector::new(6, PrivilegeLevel::Ring3);
    pub const TSS_SELECTOR: SegmentSelector = SegmentSelector::new(7, PrivilegeLevel::Ring0);

    const fn empty() -> Self {
        Self { table: [0; 16] }
    }

    fn new(tss: &'static TaskStateSegment) -> Self {
        let mut table = [0; 16];
        // 布局与引导和用户态段选择子保持一致。
        table[1] = DescriptorFlags::KERNEL_CODE32.bits();
        table[2] = DescriptorFlags::KERNEL_CODE64.bits();
        table[3] = DescriptorFlags::KERNEL_DATA.bits();
        table[4] = DescriptorFlags::USER_CODE32.bits();
        table[5] = DescriptorFlags::USER_DATA.bits();
        table[6] = DescriptorFlags::USER_CODE64.bits();
        if let Descriptor::SystemSegment(low, high) = Descriptor::tss_segment(tss) {
            table[7] = low;
            table[8] = high;
        }
        Self { table }
    }

    fn pointer(&self) -> DescriptorTablePointer {
        DescriptorTablePointer {
            base: VirtAddr::new(self.table.as_ptr() as u64),
            limit: (core::mem::size_of_val(&self.table) - 1) as u16,
        }
    }

    unsafe fn load(&'static self) {
        unsafe {
            lgdt(&self.pointer());
            // 长模式下分段基本无效，保留最小必要装载：CS + SS。
            // GS 基址由 percpu 初始化，不在此处改动。 
            CS::set_reg(Self::KCODE64_SELECTOR);
            SS::set_reg(Self::KDATA_SELECTOR);
        }
    }

    unsafe fn load_tss(&'static self) {
        unsafe {
            load_tss(Self::TSS_SELECTOR);
        }
    }
}

/// 初始化本核 GDT/TSS，并装载 TR。
pub fn init_gdt() {
    unsafe {
        // 初始化顺序由外部保证，此处不再维护额外初始化标志。
        let gdt = GDT.current_ref_mut_raw();
        *gdt = GdtStruct::new(TSS.current_ref_raw());

        let gdt = GDT.current_ref_raw();
        gdt.load();
        gdt.load_tss();
    }
}

#[inline]
pub fn write_tss_rsp0(rsp0: usize) {
    unsafe {
        let tss = TSS.current_ref_mut_raw();
        tss.privilege_stack_table[0] = VirtAddr::new_truncate(rsp0 as u64);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn x86_read_tss_rsp0() -> usize {
    unsafe {
        let tss = TSS.current_ref_raw();
        tss.privilege_stack_table[0].as_u64() as usize
    }
}
