# x86_64 syscall 实现汇总

范围：以下内容基于当前仓库的 `domains/common/syscall/syscall`、`user/userlib` 和相关 x86_64 迁移路径整理，侧重当前可用性、语义差异和未实现项。

## 已实现

### 文件系统与 I/O
- `openat`, `close`, `read`, `write`, `readv`, `writev`
- `pread64`, `pwrite64`, `fstatat`, `ftruncate`, `faccessat`, `lseek`, `fstat`, `fsync`
- `utimensat`, `sendfile`
- `pselect6`, `select`, `ppoll`, `getdents64`, `chdir`, `fchdir`, `getcwd`
- `mkdirat`, `unlinkat`, `renameat2`, `truncate`, `statfs`, `fstatfs`
- `linkat`, `symlinkat`, `readlinkat`
- `pipe2`, `epoll_create1`, `epoll_ctl`, `eventfd2`
- `dup`, `dup2`, `fcntl`, `ioctl`

### 任务与进程
- `clone`, `execve`, `exit`, `exit_group`
- `wait4`, `waitid`, `set_tid_address`, `arch_prctl`
- `getpid`, `getppid`, `gettid`
- `getuid`, `geteuid`, `getgid`, `getegid`
- `setpgid`, `getpgid`, `setsid`
- `sigaltstack`, `sigaction`, `sigprocmask`, `futex`
- `setpriority`, `getpriority`, `getrlimit`, `setrlimit`, `getrusage`, `prlimit64`, `umask`, `madvise`

### 时间与系统信息
- `clock_gettime`, `gettimeofday`, `nanosleep`
- `uname`, `getrandom`

### 内存管理
- `brk`, `mmap`, `munmap`, `mprotect`

### 套接字
- `socket`, `socketpair`, `bind`, `listen`, `accept`, `connect`
- `getsockname`, `getpeername`, `sendto`, `recvfrom`
- `setsockopt`, `getsockopt`, `shutdown`

### GPU 与输入
- `framebuffer`, `framebuffer_flush`, `event_get`

### 域管理
- `load_domain`, `replace_domain`

## 语义有差异或最小实现

| syscall | 现状 |
| --- | --- |
| `uname` | `machine` 字段仍沿用固定值，未按目标架构动态区分。 |
| `wait4` | 返回的是 Linux 风格状态字，需要按 `status >> 8` 读退出码。 |
| `waitid` | 仅支持 `P_ALL` / `P_PID`，`siginfo` 只回填最小字段。 |
| `faccessat` | 目前更接近“能否打开”的最小检查，不是完整的逐位权限判定。 |
| `select` / `pselect6` / `ppoll` | 采用简单轮询实现，`fd` 位图只覆盖 64 个位置；`ppoll` 的无效 fd 记为 `EPOLLERR`，`select` 只是 x86_64 兼容别名。 |
| `nanosleep` | 忙等 + `yield`，`rem` 只回写零值，不处理信号打断。 |
| `clock_gettime` | 只支持 `CLOCK_MONOTONIC`、`CLOCK_REALTIME`、`CLOCK_PROCESS_CPUTIME_ID`，其他 clock id 目前会触发 panic。 |
| `getrusage` | 当前返回零值统计，先满足常见调用路径。 |
| `umask` | 当前作为 task 级状态保存并返回旧值，尚未接入 VFS 创建掩码。 |
| `getrandom` | 使用 `oorandom` 生成伪随机字节流；只接受 `GRND_NONBLOCK` / `GRND_RANDOM`，`GRND_RANDOM` 单次最多返回 512 字节。 |
| `setpgid` / `getpgid` / `setsid` | 当前是最小 stub，直接返回成功，不维护真实会话/进程组。 |
| `getuid` / `geteuid` / `getgid` / `getegid` | 当前固定返回 0。 |
| `madvise` | 当前是最小 stub，直接返回成功。 |
| `linkat` | 已恢复硬链接语义，但 `AT_EMPTY_PATH` 仍不支持。 |
| `statfs` / `fstatfs` | 仅映射最小文件系统统计字段，不保证完整 Linux `statfs` 结构细节。 |
| `fcntl` | 只覆盖常用命令子集，`GETLK` / `SETLK` / `SETLKW` 当前忽略。 |
| `ioctl` | 仅做命令枚举和 VFS 转发，不覆盖完整 Linux ioctl 矩阵。 |
| `socket` 系列 | 以 AF_INET 为主，部分选项和错误路径仍是最小实现。 |
| `accept` / `connect` / `recvfrom` / `sendto` | 采用阻塞轮询 + `yield`，未做完整信号/超时/非阻塞细节。 |
| `setsockopt` / `getsockopt` | 仅支持少量常用选项。 |

## 未实现

| syscall | 现状 |
| --- | --- |
| `mount` | 当前直接返回 `ENOSYS`。 |
| xattr 全家桶（`setxattr`、`lsetxattr`、`fsetxattr`、`getxattr`、`lgetxattr`、`fgetxattr`、`listxattr`、`llistxattr`、`flistxattr`、`removexattr`、`lremovexattr`、`fremovexattr`） | 当前直接返回 `ENOSYS`。 |
| `arch_prctl`（非 `x86_64` 路径） | 非 `x86_64` 构建下返回 `ENOSYS`。 |

## Alien 专有扩展

这些接口是项目内扩展，不属于 Linux 标准 ABI：
- `load_domain`
- `replace_domain`
- `framebuffer`
- `framebuffer_flush`
- `event_get`

## 备注

- `poll(7)` 在 x86_64 路径会先归一化到 `ppoll`，用 `PPOLL_FROM_POLL_SIGMASK` 作为内部哨兵。
- 当前 summary 以 x86_64 迁移闭环为准；如果后续新增 syscall 或补全 Linux 语义，这份文件应同步更新。
