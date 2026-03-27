# x86_64 设备探测接入 Alien 体系实施计划

## 1. 目标与边界

目标：在不影响现有 riscv64 路径的前提下，让 `x86_64` 的 `bus::init_with_boot_info()` 按与原始 Alien 相同的总线抽象完成设备发现，并在 `domain::load_domains()` 阶段消费这些结果，至少稳定发现并可注册以下设备：

1. 中断控制器（Local APIC / IO APIC）
2. UART
3. VirtIO Block
4. VirtIO Input
5. VirtIO Net

边界约束：

1. 不修改 `reference/` 目录
2. 不删除或破坏 riscv64 代码路径
3. 保持 riscv64 与 x86_64 在“命名、层次、职责”上尽量对齐
4. 设备探测能力全部收敛到 `kernel/bus`，不复用 `platform` 中的 ACPI/AML/PCI 探测逻辑

当前阶段策略：先只实现最小必达设备（Local APIC、IO APIC、UART、virtio blk/input/net），其余扩展探测代码先移除，待最小链路稳定后再逐步恢复。

---

## 1.1 架构原则（新增）

为保证代码组织清晰，x86_64 采用“总线内聚”原则：

1. `platform` 仅负责最小平台启动职责（boot_info、基础映射能力、CPU 启动等）
2. `kernel/bus` 负责 ACPI 静态表解析、AML 解释执行、PCI/ECAM 枚举与分类
3. `domain` 仅消费 `bus` 输出的统一设备描述，不做底层硬件探测

说明：该拆分会带来少量重复初始化/封装成本，但相比设备发现路径清晰度和可维护性，性能损失可忽略。

---

## 2. 对齐原始 Alien 的统一流程

参考原始 Alien（riscv64）的主线：

1. 启动阶段准备硬件描述输入（riscv: DTB；x86: ACPI）
2. `bus::init_with_*` 负责“发现 + 注册到总线容器”
3. `domain::load_domains()` 负责“按设备类型创建并绑定 driver domain”

x86_64 也按同一分层落地：

1. 平台层：仅提供最小启动信息（不承载探测策略）
2. bus 层：按 `静态 ACPI -> AML -> PCI` 三步发现设备并注册
3. domain 层：只消费 `platform_bus/mmio_bus/pci_bus` 的结果，不自己做底层探测

---

## 2.1 x86_64 设备模型与命名（新增）

为避免沿用 riscv64 特有语义（如 `plic`），在 `CommonDeviceType` 中引入 x86_64 专用设备类型，并保持名称与硬件语义一致。

建议把 x86_64 侧设备分为三组：

1. 中断与时钟基础设施：
   - `LocalApic`
   - `IoApic`
   - `Hpet`
   - `AcpiPmTimer`
   - `AcpiSci`
   - `LegacyPic8259`（仅 `InterruptModel::Unknown` 时作为兼容路径）
2. 传统板载设备：
   - `Uart`
   - `Rtc`
   - `Ps2Controller`（由 FADT `iapc_boot_arch` 指示）
3. PCI 体系：
   - `PciHostBridge`（ECAM 段与总线范围）
   - `PciEndpoint`（BDF、vendor/device、class、BAR、中断能力）
   - `VirtioPciTransport`（可选，作为 `PciEndpoint` 的派生分类）

建议新增结构（示意）：

1. `PciHostInfo { segment, bus_start, bus_end, ecam_range }`
2. `PciEndpointInfo { bdf, vendor_id, device_id, class_code, subclass, prog_if, revision, bars, irq }`

这样 domain 层不再把 `pci_ecam` 作为“伪设备”批量绑定，而是按 endpoint 精确绑定驱动域。

---

## 2.2 第三方库优先策略（新增）

尽量避免手写基础解析逻辑，优先使用成熟库：

1. `acpi`：
   - `Rsdp::search_for_on_bios`
   - `AcpiTables::from_rsdp`
   - `platform::interrupt::InterruptModel::new`
   - `mcfg::PciConfigRegions::new`
   - `HpetInfo::new`
   - `find_table::<Fadt/Spcr/Madt>()`
2. `aml`：
   - `AmlContext::new`
   - `parse_table`
   - `initialize_objects`
   - 命名空间遍历与 `_HID/_CID/_STA/_CRS` 读取
3. `pci_types`：
   - `PciAddress/PciHeader/EndpointHeader`
   - BAR 读取
   - Capability 迭代（MSI/MSI-X）
4. `x86` / `x86_64`：
   - APIC/IOAPIC 控制与端口 IO
   - x86_64 结构与中断基础能力

---

## 3. 分阶段实施步骤

## 阶段 A：先消除 page fault 前置条件

目的：保证 ACPI/AML/ECAM 访问前，相关物理区间在当前页表中可访问。

步骤：

1. 梳理 `mem::init_memory_system(true)` 完成后，是否已覆盖：
   - 低端 ACPI/RSDP 可能所在区域
   - MADT/DSDT/SSDT 所在物理页
   - MCFG ECAM 区域（通常 `0xb000_0000`）
   - IOAPIC/HPET/LAPIC MMIO 区域
2. 对 `phys_to_virt()` 前提做一致性检查：
   - `LOW_PHYS_MAP_BASE/LOW_PHYS_MAP_SIZE` 对应映射必须真实存在
   - `PHYS_VIRT_OFFSET` 线性映射区必须覆盖 ACPI 解析会访问到的页面
3. 在 ACPI/AML/Pci 枚举入口加最小诊断日志（地址、长度、阶段），定位 fault 前最后一次成功访问
4. 梳理 `platform::acpi::*` 在 `kernel/bus` 的依赖链，为阶段 B 迁移做清单

建议改动位置：

1. `base/platform/src/common_x86_64/mem.rs`
2. `base/platform/src/common_x86_64/acpi/support.rs`
3. `kernel/src/bus/acpi/mod.rs`
4. `kernel/src/bus/pci/mod.rs`

验收：

1. `bus::init_with_boot_info()` 可完整返回，不因 ACPI/AML/ECAM 访问触发 page fault

---

## 阶段 B：固化“三步探测”在 x86_64 的总线入口

目的：把你要求的三步流程固化为稳定管线，并迁移到 `kernel/bus` 内部。

步骤：

1. 第一步（静态 ACPI）：
   - 在 `kernel/bus/acpi` 侧直接完成 RSDP/ACPI 表初始化，不再复用 `platform::acpi::*`
   - 使用 `InterruptModel::new` 从 MADT 获取 APIC/IOAPIC 拓扑
   - 使用 `HpetInfo::new` 解析 HPET
   - 使用 `find_table::<Fadt>()` 获取 `sci_interrupt`、`pm_timer_block`、`iapc_boot_arch`
   - 使用 `find_table::<Spcr>()` 优先识别串口（SPCR 缺失时再回退 AML）
   - 统一转换成 `CommonDeviceType` 并注册 `platform_bus`
2. 第二步（AML）：
   - 在 ACPI 表可用时加载 `DSDT + SSDT`
   - `initialize_objects()` 后遍历 namespace
   - 聚焦 `_HID/_CID` 为串口/RTC/输入控制器的设备，解析 `_CRS` 得到 IO/IRQ
   - 读取 `_STA`，跳过未启用设备
3. 第三步（PCI）：
   - 优先 MCFG 枚举 ECAM 区域并注册到 `pci_bus`
   - 用 `pci_types` 解析 endpoint（BDF/vendor/device/class/BAR/capability）
   - 按 virtio 规范识别 `vendor=0x1AF4` 的 virtio-pci 设备并分类

建议改动位置：

1. `kernel/src/bus/acpi/mod.rs`
2. `kernel/src/bus/acpi/root.rs`（新增，集中 ACPI 初始化与表访问）
3. `kernel/src/bus/acpi/descriptor_parser.rs`
4. `kernel/src/bus/acpi/pci.rs`
5. `kernel/src/bus/pci/device.rs`
6. `kernel/src/bus/mod.rs`

验收：

1. 日志中可看到三步均执行
2. `platform_bus` 至少含 `local_apic/io_apic/uart`
3. `pci_bus.endpoint_devices()` 非空，且能识别 virtio-pci 端点

---

## 阶段 C：建立 PCI 端点到驱动域的可消费信息

目的：避免 `domain` 直接拿整段 ECAM 当设备，改为“按端点类型绑定驱动域”。

步骤：

1. 在 `pci_bus` 增加 endpoint 过滤接口（按 vendor/device/class）
2. 先支持 virtio-pci 识别（vendor `0x1af4`）
3. 为 block/net/input 产出最小初始化参数：
   - BDF
   - BAR 信息（至少先读 BAR0/BAR1）
   - IRQ 线/中断能力（初期可先 INTx，后续补 MSI/MSI-X）
4. 形成统一“PCI 设备描述”供 `domain::init_device()` 消费

建议改动位置：

1. `kernel/src/bus/pci/device.rs`
2. `kernel/src/bus/pci/mod.rs`
3. （如需）新增 `kernel/src/bus/pci/virtio.rs`

验收：

1. 可列出 virtio-pci block/net/input 对应的 BDF
2. domain 层不再依赖 `"pci_ecam"` 伪设备名批量创建设备域

---

## 阶段 D：按 riscv64 风格重构 x86_64 的 domain 绑定

目的：让 x86_64 `init_device()` 和 riscv64 一样“只绑定已发现设备”，避免硬编码误绑定。

步骤：

1. 平台设备绑定：
   - `local_apic/io_apic/hpet/uart/rtc` 从 `platform_bus` 消费
2. MMIO 设备绑定：
   - 仅在确有 virtio-mmio 设备时消费 `mmio_bus`
3. PCI 设备绑定：
   - 从 `pci_bus.endpoint_devices()` 消费 virtio-pci 端点
   - 分别创建 `block/nic/input` 域，保持与 riscv64 域命名一致
4. IRQ 路由：
   - `apic.register_irq(...)` 基于真实设备 IRQ 绑定
   - 暂不稳定时先保留轮询兜底，但日志要明确标记

建议改动位置：

1. `kernel/src/domain/mod.rs`
2. `kernel/src/domain/init.rs`（如需补充初始化顺序）

验收：

1. 能看到中断控制器、uart、virtio blk/input/net 对应域被创建
2. 无重复错误绑定（如把同一 ECAM 区域同时当作 4 类设备）

---

## 阶段 E：验证与回归

目的：确认 x86_64 新链路稳定，同时不回归 riscv64。

步骤：

1. 增加分阶段日志：
   - `[bus][acpi-static]`
   - `[bus][aml]`
   - `[bus][pci]`
   - `[domain][bind]`
2. 运行 QEMU x86_64 启动测试，输出重定向 `run.txt` 并设置超时
3. 重点检查：
   - `bus::init_with_boot_info()` 无 page fault
   - 三类目标 virtio 设备均被发现并注册
   - `domain::load_domains()` 后设备域数量与类型符合预期
4. 回归 riscv64 启动路径，确认 `init_with_dtb()` 逻辑不受影响

---

## 4. 迁移拆分任务（新增）

围绕“探测全部在 `kernel/bus`”增加以下改造：

1. 在 `kernel/src/bus/acpi` 内引入独立 `AcpiHost` 与 `tables cache`
2. `kernel/bus` 直接完成 RSDP 查找与 `AcpiTables` 构建
3. 移除 `kernel/bus` 对 `platform::acpi::device_info()/tables()` 的依赖
4. `platform` 保留最小接口（启动参数、地址转换、CPU/中断底层使能），不再承载设备探测策略

验收：

1. x86_64 设备探测调用链仅经过 `kernel/bus/**`
2. platform 中不再出现 ACPI 设备分类和探测流程控制

---

## 5. 建议执行顺序（最小可用路径）

1. 先完成阶段 A（先止住 page fault）
2. 再完成阶段 B（三步探测完整跑通）
3. 接着完成阶段 D 的“最小绑定”（先 uart + block + net）
4. 最后补阶段 C 与 D 的 input/IRQ 细化

这样可以最快获得“可启动、可发现关键设备、可注册核心驱动域”的里程碑，再逐步完善中断与能力协商细节。

---

## 6. 网络资料与依据（新增）

1. `acpi` crate 文档（RSDP、ACPI tables、MADT/MCFG/HPET/FADT）
   - https://docs.rs/acpi/latest/acpi/
   - https://docs.rs/acpi/latest/acpi/rsdp/struct.Rsdp.html
   - https://docs.rs/acpi/latest/acpi/struct.AcpiTables.html
   - https://docs.rs/acpi/latest/acpi/platform/interrupt/enum.InterruptModel.html
   - https://docs.rs/acpi/latest/acpi/mcfg/struct.PciConfigRegions.html
   - https://docs.rs/acpi/latest/acpi/sdt/fadt/struct.Fadt.html
2. `aml` crate 文档（AML 上下文、表加载、设备命名空间）
   - https://docs.rs/aml/latest/aml/
   - https://docs.rs/aml/latest/aml/struct.AmlContext.html
   - https://docs.rs/aml/latest/aml/trait.Handler.html
3. `x86` / `x86_64` crate 文档（APIC/IO 端口/体系结构基础）
   - https://docs.rs/x86/latest/x86/
   - https://docs.rs/x86/latest/x86/apic/index.html
   - https://docs.rs/x86/latest/x86/io/index.html
   - https://docs.rs/x86_64/latest/x86_64/
4. `pci_types` crate 文档（标准化 PCI 配置空间解析）
   - https://docs.rs/pci_types/latest/pci_types/
   - https://docs.rs/pci_types/latest/pci_types/all.html
5. Virtio 官方规范（PCI 设备识别与传输）
   - https://docs.oasis-open.org/virtio/virtio/v1.2/cs01/virtio-v1.2-cs01.html
