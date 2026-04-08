## 测试 cfg 清单

目标：把测试、诊断和可见输出统一收敛到已有 cfg / feature gate，默认运行路径不打印证明日志。

### 1. 编译期平台 cfg

- [kernel/build.rs](kernel/build.rs#L98-L118)
	- `plat_qemu_riscv`
	- `plat_qemu_x86_64`
	- `plat_vf2`
	- `plat_vf2_sd`
	- 作用：声明并输出平台 cfg，保证 `target_arch` 与平台组合唯一。

### 2. kernel feature 清单

- [kernel/Cargo.toml](kernel/Cargo.toml#L61-L68)
	- `bench`
	- `apic_timer_test`
	- `unwind_test`
	- `time-tick`
	- `memory_test`
	- `trap_test`
	- `domain_test`
	- `domain_syscall_test`
	- `domain_task_test`
	- `domain_apic_test`
	- `domain_uart_test`
	- `domain_block_test`
	- `domain_net_test`
	- 作用：`apic_timer_test` 会下传到 [base/platform/Cargo.toml](base/platform/Cargo.toml#L10-L21) 的 `platform/apic_timer_test`，用于统一控制平台侧 APIC timer 证明输出。

### 3. 子库 feature 清单

- [base/platform/Cargo.toml](base/platform/Cargo.toml#L10-L21)
	- `apic_timer_test`
	- 作用：承接 kernel 的 `apic_timer_test`，控制平台侧 APIC timer 证明输出。

- [base/mem/Cargo.toml](base/mem/Cargo.toml#L27)
	- `memory_test`

### 4. 测试入口

- [kernel/src/main.rs](kernel/src/main.rs#L67-L90)
	- `#[cfg(all(target_arch = "x86_64", feature = "trap_test"))]`
	- `#[cfg(feature = "unwind_test")]`
	- 作用：trap 自测和 panic/unwind 测试入口。

- [kernel/src/timer.rs](kernel/src/timer.rs#L35)
	- `#[cfg(feature = "time-tick")]`
	- 作用：控制时间耗时输出。

- [kernel/src/trap/x86_64/mod.rs](kernel/src/trap/x86_64/mod.rs#L5)
	- `#[cfg(feature = "trap_test")]`
	- 作用：x86_64 trap 自测导出。

- [kernel/src/domain/mod.rs](kernel/src/domain/mod.rs#L1057)
	- `#[cfg(all(target_arch = "x86_64", any(feature = "domain_test", feature = "domain_syscall_test", feature = "domain_task_test", feature = "domain_apic_test", feature = "domain_uart_test", feature = "domain_block_test", feature = "domain_net_test")))]`
	- 作用：x86_64 域自测入口。

- [base/mem/src/lib.rs](base/mem/src/lib.rs#L46)
	- `#[cfg(feature = "memory_test")]`
	- 作用：内存自测入口。

### 5. 测试输出 gate

- [base/platform/src/common_x86_64/time/apic_timer.rs](base/platform/src/common_x86_64/time/apic_timer.rs#L118-L126)
	- `#[cfg(feature = "apic_timer_test")]`
	- 作用：APIC timer 编程证明输出，仅在测试特征下可见。

- [kernel/src/trap/x86_64/handler.rs](kernel/src/trap/x86_64/handler.rs#L108-L189)
	- `#[cfg(feature = "apic_timer_test")]`
	- 作用：APIC timer 用户态 / 内核态中断证明输出，仅在测试特征下可见。

### 6. 运行脚本测试开关

- [tools/run_x86_minimal.sh](tools/run_x86_minimal.sh#L42-L74)
	- `APIC_TIMER_TEST`
	- `UNWIND_TEST`
	- `MEMORY_TEST`
	- `TRAP_TEST`
	- `DOMAIN_TEST`
	- `DOMAIN_SYSCALL_TEST`
	- `DOMAIN_TASK_TEST`
	- `DOMAIN_APIC_TEST`
	- `DOMAIN_UART_TEST`
	- `DOMAIN_BLOCK_TEST`
	- `DOMAIN_NET_TEST`
	- `FEATURES`
	- 作用：把测试 feature 注入内核构建。

### 7. 实施计划

1. 保持默认启动路径不变，只在 `feature = "apic_timer_test"` 下输出 APIC timer 证明日志。
2. 继续沿用现有 feature gate，不新增专用测试输出 cfg。
3. 把当前仓库里所有测试相关 cfg 统一记录到本文件，后续新增 cfg 只追加这里。
4. 如果后面还要做更多诊断输出，优先挂到现有 feature gate 下，不直接进入默认路径。

### 8. 备注

- 当前策略是“复用现有 gate”，不修改 `reference/` 目录。
- `apic_timer_test` 负责 APIC timer 证明输出，`unwind_test` 负责 panic/unwind 入口，`memory_test` / `trap_test` / `domain_test` 及其子项（含 `domain_net_test`）负责模块级测试。
- `time-tick` 保留时间统计输出，不承担 proof 日志职责。
