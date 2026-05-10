# vsched2 使用说明

本文档用于说明 reference/vsched2 的设计、使用顺序，以及它和 AsyncAlien 现有 vDSO / 内核调度链路的对应关系。

## 1. 它是什么

vsched2 是一个基于 vDSO 的内核/用户协作调度器框架。它把“任务选择”和“部分调度状态”放到共享的 vDSO 数据区里，把“真正的上下文切换、地址空间切换、trap 入口处理”留给内核和架构相关实现。

它的核心目标不是替代内核，而是让内核和用户态共享同一套调度状态，减少不必要的陷入，并让调度器可以通过 vDSO 直接访问。

## 2. 核心模块

- `src/interface.rs`：定义 Task、Stack、Context、TrapHandle、SMP、VSpace、UserData 等 trait，是整个系统的接口层。
- `src/current.rs`：保存当前任务、当前进程、当前地址空间、内核调度器等全局状态，以及 vvar 数据访问辅助。
- `src/main_loop.rs`：调度主循环，包含 `trap_entry`、`kschedule`、`uschedule`、`utok_schedule` 等控制流入口。
- `src/schedule/scheduler.rs`：调度器主体，负责事件源注册、优先级计算、任务弹出。
- `src/schedule/ready_queue.rs`：就绪队列，实现一个事件源。
- `src/stack.rs`：栈的分配、回收和 trampoline 约定。
- `src/arch/`：架构相关的上下文切换实现。

## 3. 运行时使用顺序

### 3.1 内核启动

1. 内核先完成 vDSO 映射和 vvar 初始化。
2. 调用 `kernel_init_main(init_stack_base, init_task_ptr)` 初始化主核调度状态。
3. 若有其他 CPU，再调用 `kernel_init_secondary(...)` 完成副核状态初始化。

### 3.2 进程创建

1. 进程地址空间创建完成后，先映射 vDSO / vVAR。
2. 调用 `process_init(vspace_ptr)` 初始化该进程对应的用户态调度器、栈池和进程表项。
3. 进程进入用户态后，再调用 `user_init()` 补齐用户态调度器的 `sources` 字段。

### 3.3 调度与陷入

1. 普通切换入口从 `trap_entry()` / `thread_entry()` 进入调度主循环。
2. `kschedule()` 负责内核侧选择下一个任务并切换地址空间。
3. `uschedule()` 负责用户侧调度器切换。
4. `utok_schedule()` 负责从用户调度器主动陷入内核后的继续调度。

## 4. 语义要点

- Task 是调度器中的最小实体，既可能表示线程，也可能表示协程。
- Scheduler 是“事件源集合 + ready queue”的组合，优先级越小表示越高优先级。
- Context 负责特权级切换和地址空间切换，TrapHandle 负责同步 trap 的任务分发。
- UserData 用来让内核访问用户态 vDSO 私有数据，避免直接假设用户地址空间布局。

## 5. 和 AsyncAlien 的对应关系

当前 AsyncAlien 里，相关能力主要分布在以下位置：

- `vdso/vdso_impl/src/lib.rs`：vDSO 共享数据、时间快照、导出入口。
- `vdso/vdso_impl/src/api.rs`：vDSO 对外导出函数和 ABI。
- `vdso/vdso_impl/src/interface.rs`：vDSO 与内核之间的接口定义。
- `kernel/src/vdso.rs`：内核侧 vDSO 映射和共享数据刷新。
- `domains/common/task/task/src/vdso.rs`：task 域侧的 vDSO 调用封装。
- `user/userlib/src/vdso.rs`：用户态的 vDSO 解析与 syscall 回退。

这意味着，vsched2 进入 AsyncAlien 时，最先要对齐的是“共享数据布局 + 初始化顺序 + 调度入口”，然后再补架构特定的切栈逻辑。

## 6. 这份文档对后续改造的用途

1. 先作为第 1 阶段的使用说明，避免直接把 reference 代码硬搬进内核。
2. 后续改 vDSO 时，以这里的初始化顺序为准，确保用户态能拿到正确的调度状态。
3. 后续改内核时，以这里的职责边界为准，避免把上下文切换和事件源逻辑混在一起。