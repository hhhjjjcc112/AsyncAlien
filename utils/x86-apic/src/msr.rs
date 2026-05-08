// utils/x86-apic/src/msr.rs
// MSR (Model-Specific Register) 操作

use crate::error::Result;

/// 读取 APIC ESR（Error Status Register）via MSR (0x80B)
/// 仅在 x2APIC 模式下使用
pub fn read_apic_esr() -> Result<u32> {
    let esr_value = unsafe {
        let mut low: u32 = 0;
        core::arch::asm!(
            "rdmsr",
            in("ecx") 0x80B_u32,
            out("eax") low,
            options(preserves_flags)
        );
        low
    };
    Ok(esr_value)
}

/// 读取 x2APIC APIC ID via MSR (0x802)
pub fn read_x2apic_id() -> Result<u32> {
    let id_value = unsafe {
        let mut low: u32 = 0;
        core::arch::asm!(
            "rdmsr",
            in("ecx") 0x802_u32,
            out("eax") low,
            options(preserves_flags)
        );
        low
    };
    Ok(id_value)
}

/// 读取 x2APIC Version via MSR (0x803)
pub fn read_x2apic_version() -> Result<u32> {
    let version_value = unsafe {
        let mut low: u32 = 0;
        core::arch::asm!(
            "rdmsr",
            in("ecx") 0x803_u32,
            out("eax") low,
            options(preserves_flags)
        );
        low
    };
    Ok(version_value)
}
