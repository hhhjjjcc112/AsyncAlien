//! IRQ 管理接口。

/// IPI 目标描述。
#[derive(Debug, Clone, Copy)]
pub enum IpiTarget {
    /// 单播到指定 CPU。
    Unicast { cpu_id: usize },
    /// 广播，可选择排除当前 CPU。
    Broadcast { exclude_self: bool },
    /// 按掩码发送。
    Multicast { mask: usize, mask_base: usize },
}

/// IRQ 控制器抽象。
pub trait IrqIf {
    /// 支持的最大 IRQ 号。
    const MAX_IRQ_NUM: usize;

    /// 开关指定 IRQ 线。
    fn set_enable(irq: usize, enabled: bool);

    /// 获取当前正在处理的 IRQ 号。
    fn current_irq() -> Option<usize>;

    /// 完成一次 IRQ 处理（EOI）。
    fn ack_irq(irq: usize);

    /// 发送 IPI。
    fn send_ipi(target: IpiTarget);

    /// 初始化主核 IRQ 控制器。
    fn init_primary();

    /// 初始化从核 IRQ 控制器。
    fn init_secondary(cpu_id: usize);

    /// 分发当前中断，返回已处理的 IRQ 号。
    fn dispatch() -> Option<usize> {
        if let Some(irq) = Self::current_irq() {
            // 具体处理流程由上层中断框架负责。
            Self::ack_irq(irq);
            Some(irq)
        } else {
            None
        }
    }
}
