# x86_64_vdso分阶段实施计划（AsyncAlien）

## 1. 目标与原则
1. 总目标：基于 `reference/vdso_crate_template` 在 AsyncAlien 引入 vDSO 机制，先最小可运行，再逐步增强。  
2. 约束：
- 不修改 `reference/`。  
- 不删除或破坏现有 `riscv64` 路径。  
- 对外接口尽量保持平台无关，平台差异下沉到内部实现。  
- 任何 vDSO 失败场景都必须可回退到 syscall，保证功能正确优先。  
3. 当前优先级：先完成 Phase 1（最小可运行）。

## 2. 阶段划分

### Phase 1：最小可运行（MVP）
1. 阶段目标：
- 打通 `构建产物 -> 进程地址空间映射 -> auxv 注入 -> 用户态解析调用 -> 回退 syscall` 全链路。  
2. 功能范围：
- 仅实现 `clock_gettime` 的 vDSO 快路径（`CLOCK_REALTIME`、`CLOCK_MONOTONIC`）。  
- `gettimeofday/time/getcpu` 暂不纳入 vDSO，继续 syscall。  
3. 关键设计：
- 映射布局：`[vVAR(RW)][vDSO(RX)]`，4K 对齐。  
- auxv：注入 `AT_SYSINFO_EHDR = vDSO ELF 基址`。  
- 回退：符号解析失败、映射缺失、调用异常时统一 syscall 回退。  
4. 交付物：
- 根目录计划文档（本文件）。  
- Phase 1 代码实现（后续执行阶段完成）。  
- `迁移文档.md` 追加记录。  
5. 验收标准：
- 用户态可读取合法 `AT_SYSINFO_EHDR`。  
- `clock_gettime` 正常返回，单调时钟不倒退。  
- 禁用/失效 vDSO 后应用行为不变（回退成功）。  
- `timeout 240s ARCH=x86_64 make run |& tee run.txt` 无新增卡死或崩溃。

### Phase 2：双架构对齐（x86_64 / riscv64）
1. 阶段目标：
- 建立统一 vDSO 抽象层，使两架构对等接入。  
2. 功能范围：
- 公共接口模块化（如 `vdso/common`）。  
- 架构实现拆分（如 `vdso/x86_64`、`vdso/riscv64`）。  
- auxv 语义保持一致。  
3. 验收标准：
- 双架构可构建运行，无功能回归。  
- 回退路径和错误处理一致。

### Phase 3：接口扩展（增量）
1. 阶段目标：
- 在稳定基础上扩展高频接口。  
2. 候选接口：
- `gettimeofday`、`time`、`getcpu`。  
3. 原则：
- 每新增一个接口，都具备“可禁用、可回退、可验证”。  
4. 验收标准：
- 每个新增接口有独立测试和回退验证。

### Phase 4：工程化与性能评估
1. 阶段目标：
- 提升可维护性、可观测性，并给出性能收益。  
2. 内容：
- 构建流程固化、符号版本策略整理、日志开关与诊断信息。  
- vDSO 与 syscall 微基准对比（时延/吞吐）。  
3. 验收标准：
- 结论可复现，并写入文档。

## 3. Phase 1 实施清单（执行时使用）

1. 构建侧：
- 引入 `build_vdso` 流程。  
- 生成 vDSO `.so` 与符号/接口产物。  
- 保持与现有构建体系兼容。  

2. 任务域映射侧：
- 在进程地址空间创建阶段加入 vVAR/vDSO 映射。  
- 保证地址不与用户栈、mmap、trampoline/trap 冲突。  

3. auxv 侧：
- 在 `AuxVec` 生成与入栈流程增加 `AT_SYSINFO_EHDR`。  
- 仅映射成功时注入。  

4. 用户态调用侧：
- 时间接口先尝试 vDSO，失败回退 syscall。  
- 解析 `AT_SYSINFO_EHDR`，校验 ELF，定位 `__vdso_clock_gettime`。  

5. 观测与调试：
- 增加最小日志点：映射结果、auxv 注入、是否回退。  

## 4. 测试计划（Phase 1）
1. 用例1：`AT_SYSINFO_EHDR` 存在且 ELF 可解析。  
2. 用例2：`clock_gettime(CLOCK_MONOTONIC)` 多次调用不倒退。  
3. 用例3：关闭 vDSO 后 syscall 回退可用。  
4. 回归：`timeout 240s ARCH=x86_64 make run |& tee run.txt`。  
5. 风险专项：地址冲突、权限错误、auxv 漏传、符号解析失败。  
6. 输出：结果摘要写入 `迁移文档.md`。

## 5. Phase 1 简化项
1. 不实现 `gettimeofday/time/getcpu` 的 vDSO 版本。  
2. 不做复杂符号版本协商。  
3. 不做多页复杂共享结构优化，仅保留最小 vVAR。

## 6. 后续完善建议
1. 完善符号版本与 ABI 稳定策略。  
2. 推进 riscv64 同步实现与统一测试基线。  
3. 引入自动化性能基准并长期跟踪。  
4. 逐步将高频时间调用迁移到 vDSO 快路径。
