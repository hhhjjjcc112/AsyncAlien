// utils/x86-apic/src/port_io.rs
// 端口 I/O 操作（禁用 8259A PIC）

use crate::error::Result;
use x86::io::outb;

/// 禁用 8259A 可编程中断控制器 (PIC)
/// 
/// 关闭所有中断源，防止与 APIC 中断冲突。
/// 
/// # 操作细节
/// - 主 PIC（Master）：端口 0x21，写入 0xFF（禁用所有中断）
/// - 从 PIC（Slave）：端口 0xA1，写入 0xFF（禁用所有中断）
pub fn disable_8259a() -> Result<()> {
    unsafe {
        // 禁用主 PIC 的所有中断
        outb(0x21, 0xFF);
        
        // 禁用从 PIC 的所有中断
        outb(0xA1, 0xFF);
    }
    Ok(())
}
