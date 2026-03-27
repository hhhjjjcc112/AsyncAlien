# 原始 Alien（RISC-V64）外设发现与驱动域注册流程调研

本文基于 `reference/Alien` 的原始代码，按启动时序解释：

1. 内核如何一步步“发现外设”  
2. 这些设备信息如何进入内核总线容器  
3. 最终如何绑定到对应 driver domain（以及 IRQ）

---

## 0. 入口与 DTB 来源

原始 Alien 在平台层先完成 DTB 记录，再进入内核主函数：

- `platform_init(hart_id, dtb)` 会调用 `init_dtb(Some(dtb))`，并缓存 machine info。  
  代码：`reference/Alien/base/platform/src/lib.rs:39-47`
- 后续内核通过 `platform_dtb_ptr()` 取回 DTB 指针。  
  代码：`reference/Alien/base/platform/src/lib.rs:65-70`

结论：`bus` 层设备发现的输入是 DTB（FDT）。

---

## 1. 主核启动时序：先初始化内存，再做 DTB 设备发现

主核分支里关键顺序如下：

1. `mem::init_memory_system(...)`
2. `arch::allow_access_user_memory()`
3. `bus::init_with_dtb().unwrap()`
4. `trap::init_trap_subsystem()`
5. `domain::load_domains().unwrap()`

代码：`reference/Alien/kernel/src/main.rs:40-54`

重点：设备发现明确发生在 `domain::load_domains()` 之前，后者只消费发现结果并绑定驱动域。

---

## 2. bus 总入口：`init_with_dtb()`

`bus::init_with_dtb()` 的职责是“收集 + 分发”：

- 从 DTB 依次探测：
  - `rtc`
  - `uart`
  - `plic`
  - `pci`
  - `virtio`（可能多个）
- 平台条件编译下再补充：
  - `ramdisk` / `loopback` / `sdcard`
- 最后统一按 `CommonDeviceType` 分发到不同总线容器：
  - `platform::register_platform_device(...)`
  - `mmio::register_mmio_device(...)`
  - `pci::pci_init(...)`

代码：`reference/Alien/kernel/src/bus/mod.rs:33-105`

---

## 3. DTB 解析细节：`bus/fdt.rs`

### 3.1 通用节点解析：`probe_common`

`probe_common(device_name, has_irq)` 做了三件事：

1. 用 `node.name.starts_with(device_name)` 找节点  
2. 从 `reg` 取物理地址区间 `address_range`  
3. 按 `has_irq` 决定是否读取 `interrupts`  
4. 记录 `compatible`

代码：`reference/Alien/kernel/src/bus/fdt.rs:64-89`

### 3.2 设备特化探测

- UART：优先 `"uart"`，其次 `"serial"`  
  代码：`reference/Alien/kernel/src/bus/fdt.rs:24-31`
- RTC：`probe_common("rtc", true)`  
  代码：`reference/Alien/kernel/src/bus/fdt.rs:33-35`
- PLIC：`probe_common("plic", false)`  
  代码：`reference/Alien/kernel/src/bus/fdt.rs:37-39`
- PCI：`probe_common("pci", false)`  
  代码：`reference/Alien/kernel/src/bus/fdt.rs:91-93`

### 3.3 VirtIO（MMIO）批量探测

`probe_virtio()` 遍历所有节点，凡 `node.name.starts_with("virtio")` 就提取：

- `reg` -> 地址范围
- `interrupts` -> IRQ
- `compatible`

打包为 `CommonDeviceType::VirtIo`（可多个）。

代码：`reference/Alien/kernel/src/bus/fdt.rs:41-62`

---

## 4. 发现后的第一层注册：总线容器

### 4.1 PlatformBus（PLIC/UART/RTC/ramdisk 等）

- 注册入口：`register_platform_device(info, name)`  
  代码：`reference/Alien/kernel/src/bus/platform/mod.rs:13-18`
- 存储结构：`PlatformBus.common_devices: VecDeque<PlatformCommonDevice>`  
  代码：`reference/Alien/kernel/src/bus/platform/device.rs:9-30`

这是后续 `domain::init_device()` 的主要输入源之一。

### 4.2 MmioBus（VirtIO MMIO）

- 注册入口：`register_mmio_device(info)`  
  代码：`reference/Alien/kernel/src/bus/mmio/mod.rs:15-23`
- 关键过滤条件：
  - `magic == 0x74726976`（VirtIO 魔数）
  - `device_id != 0`
- 满足后入 `MmioBus.common_devices`

代码：`reference/Alien/kernel/src/bus/mmio/mod.rs:13-23`  
容器结构：`reference/Alien/kernel/src/bus/mmio/device.rs:7-30`

### 4.3 PCI 路径现状（原始 Alien）

`bus/mod.rs` 虽然会把 `CommonDeviceType::Pci` 分发到 `pci::pci_init`，但原始代码里：

- `pci_init` 为空实现
- `PciBus` 仅有容器定义，未完成探测流程

代码：

- `reference/Alien/kernel/src/bus/pci/mod.rs:8-10`
- `reference/Alien/kernel/src/bus/pci/device.rs:7-29`

结论：原始 Alien 的 RISC-V 设备发现与 driver 绑定主线是 **PlatformBus + MmioBus**，PCI 在该版本仍是占位状态。

---

## 5. Driver 域二进制准备：`init_domains()`

在真正创建设备驱动域之前，会先把 initrd 中的域 ELF 预注册：

1. 解压 `INITRD_DATA`
2. 遍历 cpio，取 `g*` 命名文件
3. 按 `INIT_DOMAIN_LIST` 调 `register_domain_elf(domain_file_name, elf, ty)`

代码：`reference/Alien/kernel/src/domain/init.rs:44-75`

`register_domain_elf` 会把 ELF 放入 `DOMAIN_ELF` 映射，并更新 `DOMAIN_INFO.ty_list`。

代码：`reference/Alien/kernel/src/domain_loader/creator.rs:31-58`

---

## 6. 从设备容器到 driver 域：`load_domains()` -> `init_device()`

`load_domains()` 的关键顺序：

1. 先加载基础域（scheduler/logger/fs/vfs/task 等）
2. 再调用 `init_device()` 创建设备相关驱动域
3. 最后注册 syscall/task/plic 到 trap/task 子系统

代码：`reference/Alien/kernel/src/domain/mod.rs:315-462`

### 6.1 `init_device()` 先处理 PlatformBus

`init_device()` 先从 `platform_bus` 取 `plic` 创建设备中断控制域：

- 创建设备域：`PLICDomainProxy`
- 初始化参数：`PlicInfo{device_info, ty}`
- 注册域名：`"plic"`（unique）

代码：`reference/Alien/kernel/src/domain/mod.rs:29-54`

然后遍历 `platform_bus.common_devices()`，按 `name()` 分派到 driver domain：

- `rtc` -> `goldfish` 域，并 `plic.register_irq`
- `uart` -> `uart16550` / `uart8250`，再创建 `buf_uart`，并绑 IRQ
- `ramdisk` -> `mem_block`
- `loopback` -> `loopback` 网卡域
- `sdcard` -> `vf2_sd`

代码：`reference/Alien/kernel/src/domain/mod.rs:58-153`

### 6.2 再处理 MmioBus（VirtIO MMIO）

遍历 `mmio_bus.common_devices()`，按 `VirtioMmioDeviceType` 分派：

- `Network` -> `virtio_mmio_net`
- `Block` -> `virtio_mmio_block`
- `Input` -> `virtio_mmio_input` + `buf_input`，并绑 IRQ
- `GPU` -> `virtio_mmio_gpu`

代码：`reference/Alien/kernel/src/domain/mod.rs:155-243`

之后还会创建：

- `net_stack`（依赖 `"nic-1"`）
- `shadow_blk` / `cache_blk`
- `null` / `random`

代码：`reference/Alien/kernel/src/domain/mod.rs:244-312`

---

## 7. “注册到驱动域”到底做了什么

`register_domain(identifier, domain_file, domain, unique)` 的行为：

1. 放入 `DOMAIN_CONTAINER.domains`（name -> DomainType）
2. `unique=false` 时自动生成 `name-N`（如 `nic-1`, `block-1`）
3. 记录 `domain_id -> DomainDataInfo` 到 `DOMAIN_INFO.domain_list`

代码：`reference/Alien/kernel/src/domain_helper/mod.rs:95-119`

宏 `register_domain!` 只是该函数的语法糖：

代码：`reference/Alien/kernel/src/domain_helper/mod.rs:121-126`

---

## 8. 时序总览（RISC-V64 主线）

1. 平台记录 DTB 指针：`platform_init -> init_dtb`  
2. 主核进 `main`，调用 `bus::init_with_dtb()`  
3. `bus::init_with_dtb()` 用 `fdt::Probe` 提取 `CommonDeviceType` 列表  
4. 按类型分发入 `PlatformBus` / `MmioBus` / `Pci`（PCI 现阶段未实作）  
5. `load_domains()` 先 `init_domains()` 预注册各域 ELF  
6. `init_device()` 从 `PlatformBus` 与 `MmioBus` 取设备，创建并注册对应驱动域  
7. 为可中断设备调用 `plic.register_irq(...)` 完成 IRQ 到域的绑定

---

## 9. 关键结论（针对“如何一步步发现并注册”）

- 原始 Alien 的 RISC-V64 外设发现核心是 **DTB 驱动**（不是 ACPI）。
- 设备发现与驱动绑定是两段式：
  - **发现阶段**：`bus::init_with_dtb()` 生产并归档设备描述
  - **绑定阶段**：`domain::init_device()` 消费设备描述并创建 driver domain
- VirtIO 设备在原始 RISC-V64 主线主要走 **VirtIO MMIO**，通过 magic + device_id 判定有效设备。
- PCI 在该原始版本中仍为占位实现，尚未形成完整“发现 -> 绑定”的闭环。
