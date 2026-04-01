//! _CRS 资源描述解析已迁移到 utils。
//! 内核侧保留薄转发，避免调用点改动。

pub use acpi_resource_parser::{first_io_port_base, first_io_port_length, first_irq};
