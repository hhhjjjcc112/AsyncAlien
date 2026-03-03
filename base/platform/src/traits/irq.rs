//! Interrupt request (IRQ) handling interface
//!
//! Provides interrupt management abstraction.

/// Target specification for inter-processor interrupts (IPIs)
#[derive(Debug, Clone, Copy)]
pub enum IpiTarget {
    /// Send to a specific CPU
    Unicast { cpu_id: usize },
    /// Send to all CPUs except self
    Broadcast { exclude_self: bool },
    /// Send to CPUs matching mask
    Multicast { mask: usize, mask_base: usize },
}

/// IRQ management trait
///
/// Platform implementations provide interrupt controller abstraction.
pub trait IrqIf {
    /// The maximum IRQ number supported
    const MAX_IRQ_NUM: usize;

    /// Enable or disable a specific IRQ line
    fn set_enable(irq: usize, enabled: bool);

    /// Get the IRQ number currently being handled (if any)
    /// Called during interrupt handling to determine which IRQ triggered
    fn current_irq() -> Option<usize>;

    /// Acknowledge/complete an IRQ (end of interrupt)
    fn ack_irq(irq: usize);

    /// Send an inter-processor interrupt (IPI)
    fn send_ipi(target: IpiTarget);

    /// Initialize the interrupt controller for primary CPU
    fn init_primary();

    /// Initialize the interrupt controller for secondary CPU
    fn init_secondary(cpu_id: usize);

    /// Dispatch and handle the current interrupt
    /// Returns the IRQ number that was handled, if any
    fn dispatch() -> Option<usize> {
        if let Some(irq) = Self::current_irq() {
            // Handle the IRQ here (call registered handler)
            Self::ack_irq(irq);
            Some(irq)
        } else {
            None
        }
    }
}
