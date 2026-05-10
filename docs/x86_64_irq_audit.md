# x86_64 设备接入与 IRQ 来源审计（阶段一）

> 日期：2026-05-09  
> 范围：`kernel/src/bus/x86_64/*`、`kernel/src/domain/x86_64.rs`、`domains/common/io_apic/io_apic/src/lib.rs`

## 1. 当前设备接入路径总览

1. 总线枚举入口：`kernel/src/bus/x86_64/mod.rs::init_with_acpi()`
2. 静态设备枚举：`acpi::enumerate_static_devices()`（APIC/HPET/UART/RTC）
3. PCI 主桥枚举：`acpi::enumerate_pci_devices()`（优先 MCFG，失败则固定 ECAM 兜底）
4. 平台设备注册：`platform::register_platform_device(...)`
5. 域初始化与 IRQ 绑定：`kernel/src/domain/x86_64.rs::init_device()`

## 2. 设备与 IRQ 来源（现状）

### 2.1 Local APIC / IO APIC

- 地址来源：优先 ACPI MADT（`local_apic_address` / `MadtEntry::IoApic`）
- 兜底：
  - LAPIC：`0xfee0_0000`
  - IOAPIC：`0xfec0_0000`
- IRQ：这两个控制器本身不走普通设备 IRQ 注册表。

### 2.2 UART

- 主路径：ACPI SPCR
  - IRQ 来源：`spcr.irq()` 或 `spcr.global_system_interrupt()`
- 回退路径：AML `_CRS` 解析（`descriptor_parser::first_irq`）
- 最终兜底：COM1（PIO `0x3f8..0x400`，IRQ=4）
- 注册到 io_apic：**已接入**（`configure_irq + set_irq_enable + register_irq`）

### 2.3 RTC

- 当前实现：CMOS 固定端口与固定 IRQ（`0x70..0x72`，IRQ=8）
- ACPI 作用：仅判断 FADT 是否存在，不改变固定 IRQ 策略
- 注册到 io_apic：当前代码未显式注册（沿用最小实现策略）

### 2.4 PCI Host / ECAM

- 来源：优先 ACPI MCFG；失败时固定 ECAM `platform::config::PCI_ECAM_BASE`
- 本身 IRQ：`None`（主桥设备不直接使用中断线）

### 2.5 Virtio PCI（blk/net/input/gpu）

- 设备发现来源：PCI 扫描（ECAM 或 CF8/CFC 回退）
- IRQ 来源：PCI config `Interrupt Line`（offset `0x3c`）
- 注册到 io_apic：
  - 阶段一前：仅 UART 显式注册，virtio PCI 路径不完整
  - 阶段一后：**blk/net/input/gpu 已补齐 io_apic 注册逻辑**

## 3. 阶段一本次修改

### 3.1 统一 IRQ 绑定函数

- 文件：`kernel/src/domain/x86_64.rs`
- 新增：`bind_irq_to_io_apic(...)`
- 行为：
  1. 计算向量 `vector = 32 + irq`
  2. 调用 `io_apic.configure_irq(...)`
  3. 调用 `io_apic.set_irq_enable(...)`
  4. 调用 `io_apic.register_irq(...)`
  5. 输出统一日志：`[x86_64][io_apic] register irq=<n> for device=<name>`

### 3.2 补齐 virtio PCI IRQ 注册

- 文件：`kernel/src/domain/x86_64.rs`
- 范围：`virtio-blk`、`virtio-net`、`virtio-input`、`virtio-gpu`
- IRQ 获取：`ep.interrupt_line()`
- 绑定时机：每个设备域注册完成后立即绑定到 io_apic

### 3.3 修复共享 IRQ 覆盖问题

- 文件：`domains/common/io_apic/io_apic/src/lib.rs`
- 问题：旧实现 `BTreeMap<irq, DeviceDomain>`，同 IRQ 多设备会被后者覆盖
- 修复：
  - 改为 `BTreeMap<irq, Vec<DeviceDomain>>`
  - `register_irq` 改为追加（并去重 Name）
  - `handle_irq` 对同 IRQ 绑定的多个设备逐个分发

## 4. 已知边界与后续建议

1. 当前仍依赖 PCI `Interrupt Line` 字段；若要支持更复杂真实硬件拓扑，建议补 `_PRT` -> GSI 路由解析。
2. RTC 目前维持固定 IRQ=8 的最小策略，如需统一可观测性可考虑也注册到 io_apic 域。
3. 可在后续 `record_run` 中对 `io_apic.irq_info` 做自动采样，形成“设备->irq->计数”闭环。

## 5. 建议验证方式

```bash
ARCH=x86_64 make record_run
```

然后检查日志中是否出现：

- `[x86_64][io_apic] register irq=... for device=virtio_block-*`
- `[x86_64][io_apic] register irq=... for device=virtio_net-*`
- `[x86_64][io_apic] register irq=... for device=virtio_input-*`
- `[x86_64][io_apic] register irq=... for device=virtio_gpu-*`
- `[x86_64][io_apic] register irq=... for device=buf_uart-*`
