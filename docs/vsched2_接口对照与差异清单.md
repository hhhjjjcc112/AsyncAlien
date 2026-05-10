# vsched2 接口对照与差异清单

本文档对照 vsched2 的 8 个核心 trait 与 AsyncAlien 现有实现，明确映射关系和需要补齐的能力。

## 1. 核心 Trait 映射

### 1.1 Task 接口

**vsched2 定义** (`reference/vsched2/src/interface.rs:Task`)
- `state() / set_state()` - 任务状态管理
- `priority() / is_coroutine()` - 优先级和任务类型
- `pid() / set_pid()` - 进程ID 管理
- `save_thread_context() / save_trap_context()` - 上下文保存
- `restore_context() / poll()` - 上下文恢复（线程/协程）
- `thread_stack_base() / set_return_value()` - 栈和返回值

**AsyncAlien 对应实现**
- `/home/hjch/AsyncAlien/domain-lib/task_meta/src/x86_64.rs` - TaskContext（上下文保存）
- `/home/hjch/AsyncAlien/domain-lib/task_meta/src/riscv64.rs` - TaskContext（架构相关）
- `/home/hjch/AsyncAlien/domains/common/task/task/src/task.rs` - Task 包装
- `/home/hjch/AsyncAlien/kernel/src/task/processor.rs` - current_task、current_tid 等

**对应字段/方法**
| vsched2 | AsyncAlien | 位置 | 备注 |
|---------|-----------|------|------|
| state() | TaskStatus | task_meta | Ready/Running/Blocked/Zombie/Exited |
| priority() | scheduling_info.priority | task_meta | 优先级存储 |
| pid() | 来自 Task.pid() | task.rs | pid 字段 |
| save_thread_context() | 无直接对应 | 需补充 | 保存线程上下文 |
| restore_context() | switch() | kernel/task/x86_64.rs | 上下文恢复 |
| thread_stack_base() | TaskContext.kstack_top | task_meta | 栈底指针 |

**需要补齐的能力**
- [ ] Task 的状态管理（Ready/Running/Blocked/Zombie/Exited）没有 vsched2 的完整对应
- [ ] Thread context 保存机制需要补齐
- [ ] Coroutine 的 poll 接口没有对应实现
- [ ] Return value 设置没有对应点

---

### 1.2 Stack 接口

**vsched2 定义**
- `alloc() -> *mut ()` - 分配栈
- `dealloc(stack: *mut ())` - 回收栈

**AsyncAlien 对应实现**
- `/home/hjch/AsyncAlien/kernel/src/task/processor.rs` - 栈分配逻辑（基于 TaskMetaExt）
- `/home/hjch/AsyncAlien/domain-lib/basic/src/` - 基础栈操作

**需要补齐的能力**
- [ ] 需要一个统一的 Stack trait，能在内核和用户态独立实现栈的分配/回收
- [ ] 用户态栈和内核栈的生命周期管理需要分离

---

### 1.3 Context 接口

**vsched2 定义** (`reference/vsched2/src/interface.rs:Context`)
- `into_kernel() -> !` - 陷入内核态
- `into_user(ustack: usize)` - 进入用户态（协程）
- `into_user_context(task: *const ())` - 进入用户态（线程）
- `switch_vspace(vspace_pid: *const ())` - 切换地址空间

**AsyncAlien 对应实现**
- `/home/hjch/AsyncAlien/kernel/src/task/x86_64.rs::switch()` - 任务切换（上下文恢复）
- `/home/hjch/AsyncAlien/kernel/src/trap/` - trap 处理，涉及特权级切换
- `/home/hjch/AsyncAlien/domains/common/task/task/` - 地址空间操作（通过 ptable）

**对应字段/方法**
| vsched2 | AsyncAlien | 位置 | 备注 |
|---------|-----------|------|------|
| into_kernel() | trap 处理 | kernel/src/trap/ | 陷入内核，但当前是被动的 |
| into_user(ustack) | switch() 后 ret | kernel/src/task/x86_64.rs | 返回用户态 |
| into_user_context() | switch() | kernel/src/task/x86_64.rs | 线程上下文切换 |
| switch_vspace() | 无直接实现 | 需补充 | 页表切换 |

**需要补齐的能力**
- [ ] Context 切换的统一抽象（当前分散在 switch/trap 各处）
- [ ] into_kernel 作为主动调用的入口（当前只有被动 trap）
- [ ] 地址空间切换（switch_vspace）没有在调度路径中体现
- [ ] 用户态调度器触发内核 reschedule 的机制（utok_schedule）

---

### 1.4 TrapHandle 接口

**vsched2 定义**
- `get_handler(task: *const ()) -> *const ()` - 获取或创建 trap 处理任务

**AsyncAlien 对应实现**
- `/home/hjch/AsyncAlien/kernel/src/trap/` - trap 处理函数
- `/home/hjch/AsyncAlien/domains/common/task/` - task domain 的 syscall 处理

**对应字段/方法**
| vsched2 | AsyncAlien | 位置 | 备注 |
|---------|-----------|------|------|
| get_handler() | 无对应 | 需补充 | trap 处理任务分发机制 |

**需要补齐的能力**
- [ ] TrapHandle 的任务池管理（当前是直接在内核处理，没有任务分发）
- [ ] Trap 处理异步化（当前是同步处理）
- [ ] Trap 上下文传递到处理任务的机制

---

### 1.5 SMP 接口

**vsched2 定义**
- `cpu_id() -> usize` - 获取当前 CPU ID

**AsyncAlien 对应实现**
- `/home/hjch/AsyncAlien/platform/src/percpu_impl.rs` - `cpu_id()` 函数
- `/home/hjch/AsyncAlien/config.rs` - `CPU_NUM` 常量

**对应字段/方法**
| vsched2 | AsyncAlien | 位置 | 备注 |
|---------|-----------|------|------|
| cpu_id() | cpu_id() | platform::percpu_impl | 直接兼容 |
| CPU_NUM | CPU_NUM | config | 直接兼容 |

**需要补齐的能力**
- [ ] 多核状态同步（当前 CPU_NUM 是编译期常量，运行时可能需要同步）

---

### 1.6 VSpace 接口

**vsched2 定义**
- `into_vspace(vspace: *mut ())` - 切换到指定地址空间（页表根）

**AsyncAlien 对应实现**
- `/home/hjch/AsyncAlien/mem/` - 页表操作
- `/home/hjch/AsyncAlien/domain-lib/elf/` - VM space 创建和克隆
- `/home/hjch/AsyncAlien/kernel/src/mem/` - 内核侧页表管理

**对应字段/方法**
| vsched2 | AsyncAlien | 位置 | 备注 |
|---------|-----------|------|------|
| into_vspace() | 无直接接口 | 需补充 | 页表切换 |

**需要补齐的能力**
- [ ] 统一的 VSpace 切换接口（当前没有在调度路径上使用）
- [ ] 进程表关联的页表标记（vsched2 使用 pid 表示，AsyncAlien 需要转换）

---

### 1.7 UserData 接口

**vsched2 定义**
- `get_user_data(pos, len, vspace) -> *mut ()` - 从内核访问用户 vDSO 私有数据

**AsyncAlien 对应实现**
- `/home/hjch/AsyncAlien/vdso/vdso_impl/src/interface.rs` - vDSO 接口定义
- `/home/hjch/AsyncAlien/kernel/src/vdso.rs` - 内核侧 vDSO 初始化

**对应字段/方法**
| vsched2 | AsyncAlien | 位置 | 备注 |
|---------|-----------|------|------|
| get_user_data() | vdso_reserve_user_vaddr / vaddr_to_paddr | kernel/task/vdso.rs | 部分功能存在 |

**需要补齐的能力**
- [ ] UserData 统一接口（当前分散在 vaddr_to_paddr 等多个地方）
- [ ] 用户态 vDSO 数据的一致性访问保证

---

### 1.8 Scheduler 接口

**vsched2 定义** (`reference/vsched2/src/schedule/scheduler.rs`)
- `hightest_priority(cpu_id) -> isize` - 最高优先级任务
- `pop_task(cpu_id) -> (Option<Task>, isize)` - 取出最高优先级任务
- `push_task(task) -> Result<(), Task>` - 放入任务
- `register_event_source(...)` - 注册事件源
- `unregister_event_source(...)` - 注销事件源

**AsyncAlien 对应实现**
- `/home/hjch/AsyncAlien/kernel/src/task/scheduler.rs` - 简单的 FIFO 调度器
- `/home/hjch/AsyncAlien/domains/common/task/task/src/processor.rs` - Task 域侧的任务管理

**对应字段/方法**
| vsched2 | AsyncAlien | 位置 | 备注 |
|---------|-----------|------|------|
| hightest_priority() | 无对应 | 需补充 | 优先级查询 |
| pop_task() | fetch_task() | kernel/scheduler.rs | 基本相同 |
| push_task() | add_task() | kernel/scheduler.rs | 基本相同 |
| register_event_source() | 无对应 | 需补充 | 事件源管理 |

**需要补齐的能力**
- [ ] 优先级队列而非 FIFO（当前是 FIFO）
- [ ] 事件源系统（vsched2 支持多个事件源竞争）
- [ ] 调度器实现在 vDSO 中共享（当前在内核中）

---

## 2. 全局状态映射

### vsched2 vvar_data

**vsched2 的 vvar_data** (`reference/vsched2/src/current.rs`)
| 字段 | 类型 | 作用 | 对应 |
|------|------|------|------|
| CURRENT_TASK | [AtomicPtr; CPU_NUM] | 当前任务 | kernel/task/processor.rs::CPUS[cpu_id()].task |
| KERNEL_SCHEDULER | LazyInit<AtomicPtr> | 内核调度器 | 无对应 |
| IN_KERNEL | [AtomicBool; CPU_NUM] | 是否在内核态 | 无对应 |
| CURRENT_VSPACE | [AtomicUsize; CPU_NUM] | 当前进程/页表 | 无对应 |
| PROCESS_INFO_TABLE | ProcessInfoTable | 全局进程表 | 无对应 |
| KERNEL_STACKS | SpinMutex<StackHandler> | 内核栈池 | kernel/src/task/ 中分散 |

**需要补齐的能力**
- [ ] 把当前任务从内核独占状态改为 vDSO 共享（这是关键）
- [ ] 添加"在内核态"标志位（影响 trap 路由）
- [ ] 添加进程表在 vDSO 中（用户态调度器能读）
- [ ] 统一内核栈池管理

---

## 3. 初始化接口映射

**vsched2 初始化流程** (`reference/vsched2/src/api.rs`)
| 函数 | 调用时机 | 功能 | AsyncAlien 对应 |
|------|---------|------|----------------|
| kernel_init_main() | 内核启动主核 | 初始化主核调度状态 | kernel/src/main.rs 启动代码 |
| kernel_init_secondary() | 内核启动副核 | 初始化副核调度状态 | platform/percpu::init_ap() |
| process_init(vspace_ptr) | 进程创建后 | 初始化进程调度器和栈池 | domain/task::fork() / execve() |
| process_drop(pid) | 进程销毁前 | 清理进程表项 | domain/task::exit() |
| user_init() | 用户态启动 | 初始化用户态调度器 | domains/task domain 启动 |

**需要补齐的能力**
- [ ] 在内核启动时调用 kernel_init_main（当前没有这个入口）
- [ ] 在进程创建时调用 process_init，补齐用户态调度器初始化
- [ ] 在用户态启动时调用 user_init（当前直接用 domain 初始化）

---

## 4. 关键差异点

### 4.1 任务状态机

**vsched2**
```
Ready <-> Running <-> Blocked -> Exited
```

**AsyncAlien 当前**
```
创建 -> Ready -> Running -> Zombie -> Terminated
```

**差异**：AsyncAlien 缺少 Blocked 状态作为过渡。

### 4.2 调度器位置

**vsched2**：Scheduler 在 vDSO 中共享，内核和用户态都能访问。

**AsyncAlien 当前**：Scheduler 完全在内核中，用户态无法访问（除了通过 syscall）。

**差异**：这是最大的架构差异，会影响后续的用户态调度器实现。

### 4.3 当前任务访问

**vsched2**：CURRENT_TASK 是 vDSO 共享数据，通过原子操作访问。

**AsyncAlien 当前**：CURRENT_TIDS 是 percpu 私有数据，只能在内核中访问。

**差异**：需要把 CURRENT_TASK 改到 vDSO 中，这会影响 task domain 和 kernel 的交互。

### 4.4 多核同步

**vsched2**：通过 vDSO 共享的原子变量和 IN_KERNEL 标志做多核同步。

**AsyncAlien 当前**：使用 percpu 变量和自旋锁。

**差异**：vsched2 的做法更轻量，但需要 vDSO 支持。

---

## 5. 实现建议

### 第 2 阶段的下一步行动

1. **优先级：高** - 改造 vDSO 数据结构，添加 CURRENT_TASK / KERNEL_SCHEDULER / PROCESS_INFO_TABLE 等全局状态。
2. **优先级：高** - 改造 Task trait，补齐状态管理和 context 保存接口。
3. **优先级：中** - 补齐 Context / VSpace / TrapHandle 的统一接口定义。
4. **优先级：中** - 实现进程表的 vDSO 共享版本。
5. **优先级：低** - 协程支持（当前 AsyncAlien 不涉及协程）。

### 分阶段风险评估

| 阶段 | 风险 | 建议 |
|------|------|------|
| vDSO 数据改造 | 可能影响现有 vDSO 构建和布局 | 先在 Phase 3 做单独的编译验证 |
| 调度器共享 | 可能破坏内核/用户态的隔离 | 严格限制 vDSO 中暴露的调度器接口 |
| 初始化顺序 | 可能因为初始化漏掉某些状态导致卡死 | 在 Phase 4 做逐项编译验证 |
| 双架构 | x86_64/riscv64 的上下文切换差异大 | 在 Phase 6 做架构相关的 vtable 定义 |

---

## 6. 后续文档编排

本文档标记为第 2 阶段的"接口映射表与差异清单"，后续：

- **第 3 阶段** 会补充"vDSO 改造设计文档"，明确数据布局和初始化顺序。
- **第 4 阶段** 会补充"内核协作调度设计文档"，明确初始化入口和调度流程。
- **第 5 阶段** 会补充"用户态联调设计文档"，明确用户态如何触发协作调度。
- **第 6 阶段** 会补充"架构相关设计文档"（若需要 x86_64/riscv64 差异化实现）。
