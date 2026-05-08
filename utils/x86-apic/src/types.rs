// utils/x86-apic/src/types.rs
// APIC Context 类型定义

use x2apic::lapic::{LocalApic, LocalApicBuilder, TimerDivide as X2TimerDivide, TimerMode as X2TimerMode};
use x2apic::ioapic::{IoApic, IrqFlags as X2IrqFlags, IrqMode as X2IrqMode};
use crate::error::{ApicError, Result};
use crate::{TimerDivide, TimerMode};

/// Local APIC Context - 包装 x2apic 库的 LocalApic 结构
pub struct LocalApicContext {
    /// 虚拟地址形式的 xAPIC 基地址（已加 PHYS_VIRT_OFFSET）
    xapic_base: usize,
    /// 是否使用 x2APIC 模式
    is_x2apic: bool,
    /// x2apic 库提供的 LocalApic 包装（包含所有操作的 unsafe 方法）
    apic: LocalApic,
}

// 显式声明 LocalApicContext 可安全跨线程使用
// 因为：
// 1. xapic_base 和 is_x2apic 都是不可变的配置信息
// 2. LocalApic 对硬件寄存器的访问是原子的或通过 CPU 的隐式同步保证
// 3. 只有在持有 Mutex 时才能调用修改 apic 的方法
unsafe impl Send for LocalApicContext {}
unsafe impl Sync for LocalApicContext {}

impl LocalApicContext {
    /// 创建新的 Local APIC Context
    /// 
    /// # 参数
    /// - `xapic_base`: 虚拟地址形式的 xAPIC 基地址
    /// - `is_x2apic`: 是否使用 x2APIC 模式（如果为 false，使用 xAPIC MMIO 模式）
    pub fn new(xapic_base: usize, is_x2apic: bool) -> Result<Self> {
        if xapic_base == 0 && !is_x2apic {
            return Err(ApicError::InvalidBase);
        }

        let mut builder = LocalApicBuilder::new();
        builder
            .spurious_vector(0xf1)
            .timer_vector(0xf0)
            .error_vector(0xf2)
            .timer_mode(X2TimerMode::OneShot)
            .timer_divide(X2TimerDivide::Div1)
            .timer_initial(u32::MAX);
        if !is_x2apic {
            builder.set_xapic_base(xapic_base as u64);
        }

        let apic = builder.build()
            .map_err(|_| ApicError::InitFailed)?;

        Ok(LocalApicContext {
            xapic_base,
            is_x2apic,
            apic,
        })
    }

    /// 获取 xAPIC 基地址
    pub fn xapic_base(&self) -> usize {
        self.xapic_base
    }

    /// 是否使用 x2APIC 模式
    pub fn is_x2apic(&self) -> bool {
        self.is_x2apic
    }

    /// 初始化 Local APIC 硬件
    pub fn enable(&mut self) -> Result<()> {
        unsafe {
            self.apic.enable();
        }
        Ok(())
    }

    /// 设置 Timer 初始值并启动计时
    pub fn set_timer_initial(&mut self, ticks: u32) -> Result<()> {
        unsafe {
            self.apic.set_timer_initial(ticks);
        }
        Ok(())
    }

    /// 启用 Timer
    pub fn enable_timer(&mut self) -> Result<()> {
        unsafe {
            self.apic.enable_timer();
        }
        Ok(())
    }

    /// 设置 Timer 分频值
    pub fn set_timer_divide(&mut self, divide: crate::TimerDivide) -> Result<()> {
        unsafe {
            self.apic.set_timer_divide(match divide {
                TimerDivide::Div2 => X2TimerDivide::Div2,
                TimerDivide::Div4 => X2TimerDivide::Div4,
                TimerDivide::Div8 => X2TimerDivide::Div8,
                TimerDivide::Div16 => X2TimerDivide::Div16,
                TimerDivide::Div32 => X2TimerDivide::Div32,
                TimerDivide::Div64 => X2TimerDivide::Div64,
                TimerDivide::Div128 => X2TimerDivide::Div128,
                TimerDivide::Div1 => X2TimerDivide::Div1,
            });
        }
        Ok(())
    }

    /// 设置 Timer 模式
    pub fn set_timer_mode(&mut self, mode: crate::TimerMode) -> Result<()> {
        unsafe {
            self.apic.set_timer_mode(match mode {
                TimerMode::OneShot => X2TimerMode::OneShot,
                TimerMode::Periodic => X2TimerMode::Periodic,
                TimerMode::TscDeadline => X2TimerMode::TscDeadline,
            });
        }
        Ok(())
    }

    /// 读取 Timer 当前值
    pub fn timer_current(&self) -> Result<u32> {
        let val = unsafe {
            self.apic.timer_current()
        };
        Ok(val)
    }

    /// 发送 EOI（End of Interrupt）
    pub fn end_of_interrupt(&mut self) -> Result<()> {
        unsafe {
            self.apic.end_of_interrupt();
        }
        Ok(())
    }

    /// 发送 IPI 到指定 APIC ID
    pub fn send_ipi(&mut self, dest_apic_id: u32, vector: u8) -> Result<()> {
        unsafe {
            self.apic.send_ipi(vector, dest_apic_id);
        }
        Ok(())
    }

    /// 发送 IPI 给自己
    pub fn send_ipi_self(&mut self, vector: u8) -> Result<()> {
        unsafe {
            self.apic.send_ipi_self(vector);
        }
        Ok(())
    }

    /// 发送 IPI 给所有其他 CPU（不含自己）
    pub fn send_ipi_all_excluding_self(&mut self, vector: u8) -> Result<()> {
        use x2apic::lapic::IpiAllShorthand;
        unsafe {
            self.apic.send_ipi_all(vector, IpiAllShorthand::AllExcludingSelf);
        }
        Ok(())
    }

    /// 发送 INIT IPI 到指定 APIC ID
    pub fn send_init_ipi(&mut self, dest_apic_id: u32) -> Result<()> {
        unsafe {
            self.apic.send_init_ipi(dest_apic_id);
        }
        Ok(())
    }

    /// 发送 SIPI（Startup IPI）到指定 APIC ID
    pub fn send_sipi(&mut self, vector: u8, dest_apic_id: u32) -> Result<()> {
        unsafe {
            self.apic.send_sipi(vector, dest_apic_id);
        }
        Ok(())
    }

    /// 直接配置 one-shot timer（Div1）
    pub fn configure_oneshot_timer(&mut self) -> Result<()> {
        self.set_timer_divide(TimerDivide::Div1)?;
        self.set_timer_mode(TimerMode::OneShot)?;
        self.enable_timer()?;
        Ok(())
    }

    /// 读取 Error Status Register (ESR)
    pub fn read_error_status(&self) -> Result<u32> {
        if self.is_x2apic {
            // x2APIC 模式：通过 MSR 读取
            crate::msr::read_apic_esr()
        } else {
            // xAPIC 模式：通过 MMIO 读取（需要先写 ESR 触发上次错误捕获）
            unsafe {
                // ESR 寄存器在偏移 0x280
                let esr_addr = (self.xapic_base as *const u32).add(0x280 / 4);
                Ok(core::ptr::read_volatile(esr_addr))
            }
        }
    }

    /// 读取 APIC ID
    pub fn read_apic_id(&self) -> Result<u32> {
        let val = unsafe {
            self.apic.id()
        };
        Ok(val)
    }

    /// 获取内部可变引用（高级操作用）
    pub fn as_mut(&mut self) -> &mut LocalApic {
        &mut self.apic
    }

    /// 获取内部不可变引用（查询用）
    pub fn as_ref(&self) -> &LocalApic {
        &self.apic
    }
}

/// IO APIC Context - 包装 x2apic 库的 IoApic 结构
pub struct IoApicContext {
    /// 虚拟地址形式的 IO APIC 基地址
    ioapic_base: usize,
    /// x2apic 库提供的 IoApic 包装
    apic: IoApic,
}

impl IoApicContext {
    /// 创建新的 IO APIC Context
    /// 
    /// # 参数
    /// - `ioapic_base`: 虚拟地址形式的 IO APIC 基地址
    pub fn new(ioapic_base: usize) -> Result<Self> {
        if ioapic_base == 0 {
            return Err(ApicError::InvalidBase);
        }

        let apic = unsafe {
            IoApic::new(ioapic_base as u64)
        };

        Ok(IoApicContext {
            ioapic_base,
            apic,
        })
    }

    /// 获取 IO APIC 基地址
    pub fn ioapic_base(&self) -> usize {
        self.ioapic_base
    }

    /// 获取最大 IRQ 条目数
    pub fn max_table_entry(&mut self) -> Result<u8> {
        let val = unsafe {
            self.apic.max_table_entry()
        };
        Ok(val)
    }

    /// 启用指定 IRQ
    pub fn enable_irq(&mut self, irq: u8) -> Result<()> {
        unsafe {
            self.apic.enable_irq(irq);
        }
        Ok(())
    }

    /// 禁用指定 IRQ
    pub fn disable_irq(&mut self, irq: u8) -> Result<()> {
        unsafe {
            self.apic.disable_irq(irq);
        }
        Ok(())
    }

    /// 直接配置 IRQ 重定向项
    pub fn configure_irq(&mut self, irq: u8, vector: u8, dest_cpu: u8) -> Result<()> {
        let mut entry = unsafe { self.apic.table_entry(irq) };
        entry.set_vector(vector);
        entry.set_dest(dest_cpu);
        entry.set_mode(X2IrqMode::Fixed);
        entry.set_flags(X2IrqFlags::LEVEL_TRIGGERED | X2IrqFlags::LOW_ACTIVE | X2IrqFlags::MASKED);
        unsafe {
            self.apic.set_table_entry(irq, entry);
        }
        Ok(())
    }

    /// 获取内部可变引用（高级操作用）
    pub fn as_mut(&mut self) -> &mut IoApic {
        &mut self.apic
    }

    /// 获取内部不可变引用（查询用）
    pub fn as_ref(&self) -> &IoApic {
        &self.apic
    }
}
