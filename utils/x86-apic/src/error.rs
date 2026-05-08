// utils/x86-apic/src/error.rs
// APIC 错误类型定义

#[derive(Debug, Clone, Copy)]
pub enum ApicError {
    /// 初始化失败
    InitFailed,
    /// 无效的 APIC 基地址
    InvalidBase,
    /// 操作未初始化
    Uninitialized,
    /// 不支持的模式
    UnsupportedMode,
    /// 寄存器访问失败
    RegisterAccessFailed,
}

pub type Result<T> = core::result::Result<T, ApicError>;
