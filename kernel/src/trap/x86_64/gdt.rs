use core::arch::asm;

const GDT_KERNEL_CODE: u16 = 0x08;
const GDT_KERNEL_DATA: u16 = 0x10;
const GDT_TSS: u16 = 0x28;

#[repr(C, packed)]
struct GdtDescriptor {
    limit: u16,
    base: u64,
}

#[repr(C, packed)]
struct Tss64 {
    _reserved0: u32,
    rsp0: u64,
    rsp1: u64,
    rsp2: u64,
    _reserved1: u64,
    ist: [u64; 7],
    _reserved2: u64,
    _reserved3: u16,
    io_map_base: u16,
}

impl Tss64 {
    const fn new() -> Self {
        Self {
            _reserved0: 0,
            rsp0: 0,
            rsp1: 0,
            rsp2: 0,
            _reserved1: 0,
            ist: [0; 7],
            _reserved2: 0,
            _reserved3: 0,
            io_map_base: core::mem::size_of::<Tss64>() as u16,
        }
    }
}

// 选择子布局需与 TrapFrame 默认的 CS/SS 保持一致：
// user CS=0x23(index=4), user SS=0x1b(index=3)。
static mut GDT: [u64; 8] = [
    0x0000_0000_0000_0000, // null
    0x00AF_9B00_0000_FFFF, // 0x08 kernel code 64
    0x00CF_9300_0000_FFFF, // 0x10 kernel data
    0x00CF_F300_0000_FFFF, // 0x18 user data
    0x00AF_FB00_0000_FFFF, // 0x20 user code 64
    0x0000_0000_0000_0000, // 0x28 tss low
    0x0000_0000_0000_0000, // 0x30 tss high
    0x0000_0000_0000_0000,
];

static mut TSS: Tss64 = Tss64::new();

#[inline]
fn tss_descriptor(tss: *const Tss64) -> (u64, u64) {
    let base = tss as u64;
    let limit = (core::mem::size_of::<Tss64>() - 1) as u64;

    let low = (limit & 0xFFFF)
        | ((base & 0xFF_FFFF) << 16)
        | (0x9_u64 << 40)
        | (1_u64 << 47)
        | (((limit >> 16) & 0xF) << 48)
        | (((base >> 24) & 0xFF) << 56);
    let high = (base >> 32) & 0xFFFF_FFFF;
    (low, high)
}

#[inline]
unsafe fn reload_segment_registers() {
    unsafe {
        asm!(
            "push {kcode}",
            "lea rax, [rip + 2f]",
            "push rax",
            "retfq",
            "2:",
            "mov ax, {kdata}",
            "mov ds, ax",
            "mov es, ax",
            "mov ss, ax",
            "xor eax, eax",
            "mov fs, ax",
            "mov gs, ax",
            kcode = const GDT_KERNEL_CODE as u64,
            kdata = const GDT_KERNEL_DATA,
            out("rax") _,
            options(preserves_flags)
        );
    }
}

/// 初始化本核 GDT/TSS，并装载 TR。
pub fn init_gdt() {
    unsafe {
        let mut rsp: u64;
        asm!("mov {}, rsp", out(reg) rsp, options(nomem, preserves_flags));
        TSS.rsp0 = rsp;

        let (tss_low, tss_high) = tss_descriptor(&raw const TSS);
        GDT[5] = tss_low;
        GDT[6] = tss_high;

        let gdtr = GdtDescriptor {
            limit: (core::mem::size_of::<[u64; 8]>() - 1) as u16,
            base: (&raw const GDT as *const _ as u64),
        };

        // 关键步骤：lgdt 后必须刷新段寄存器，并显式加载 TSS。
        asm!("lgdt [{}]", in(reg) &gdtr, options(readonly, nostack, preserves_flags));
        reload_segment_registers();
        asm!("ltr ax", in("ax") GDT_TSS, options(nomem, nostack, preserves_flags));
    }
}

#[inline]
pub fn write_tss_rsp0(rsp0: usize) {
    unsafe {
        TSS.rsp0 = rsp0 as u64;
    }
}
