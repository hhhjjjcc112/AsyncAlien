# x86_64 Linux syscall ABI

## ABI 总览
- 触发指令：`syscall`
- syscall 号寄存器：`rax/eax`
- 返回值寄存器：`rax`
- 参数寄存器顺序：`rdi`, `rsi`, `rdx`, `r10`, `r8`, `r9`
- 注意：第 4 个参数走 `r10`，不是 `rcx`。

## 说明
- 本文按 Linux 主线 x86_64 raw ABI 收录当前可见 syscall，并按 syscall 号升序排列。
- `AsyncAlien现状` 只标注当前仓库中的 Linux syscall 处理情况；不包含 `reference/` 中的实现。
- 下列 AsyncAlien 私有扩展不是 Linux 标准 ABI，因此不写入本表：`load_domain`、`replace_domain`、`framebuffer`、`framebuffer_flush`、`event_get`。

## syscall 列表
| syscall号 | 名称 | 作用 | 参数个数 | 参数1 | 参数2 | 参数3 | 参数4 | 参数5 | 参数6 | AsyncAlien现状 | 备注 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 0 | read | 从文件描述符读取数据。 | 3 | int: 文件描述符 | void __user *: 输出缓冲区 | size_t: 请求读取字节数 | - | - | - | 已实现 | - |
| 1 | write | 向文件描述符写入数据。 | 3 | int: 文件描述符 | const void __user *: 输入缓冲区 | size_t: 请求写入字节数 | - | - | - | 已实现 | - |
| 2 | open | 按旧接口打开文件。 | 3 | const char __user *: 路径字符串 | int: 打开标志 | mode_t: 创建时权限位 | - | - | - | x86_64 兼容别名 | 内部按 `openat(AT_FDCWD, pathname, flags, mode)` 兼容处理。 |
| 3 | close | 关闭文件描述符。 | 1 | int: 文件描述符 | - | - | - | - | - | 已实现 | - |
| 4 | stat | 按路径查询文件状态。 | 2 | const char __user *: 路径字符串 | struct stat __user *: 输出文件状态 | - | - | - | - | x86_64 兼容别名 | 内部按 `newfstatat(AT_FDCWD, pathname, statbuf, 0)` 兼容处理。 |
| 5 | fstat | 按文件描述符查询文件状态。 | 2 | int: 文件描述符 | struct stat __user *: 输出文件状态 | - | - | - | - | 已实现 | - |
| 6 | lstat | 按路径查询链接自身状态。 | 2 | const char __user *: 路径字符串 | struct stat __user *: 输出文件状态 | - | - | - | - | x86_64 兼容别名 | 内部按 `newfstatat(AT_FDCWD, pathname, statbuf, 0)` 兼容处理。 当前按 `stat` 语义处理，未保留“不跟随符号链接”的完整差异。 |
| 7 | poll | 等待一组文件描述符事件。 | 3 | struct pollfd __user *: 待轮询数组 | unsigned int: 数组长度 | int: 超时毫秒 | - | - | - | x86_64 兼容别名 | 内部按 `ppoll` 兼容处理，超时参数会先转换为 `timespec`。 当前走简化轮询路径，主要覆盖常用场景。 |
| 8 | lseek | 调整文件偏移。 | 3 | int: 文件描述符 | off_t: 目标偏移 | int: 基准位置 | - | - | - | 已实现 | - |
| 9 | mmap | 建立内存映射。 | 6 | void __user *: 期望起始地址 | size_t: 映射长度 | int: 保护标志 | int: 映射标志 | int: 文件描述符 | off_t: 文件偏移 | 已实现 | - |
| 10 | mprotect | 修改一段映射的访问权限。 | 3 | void __user *: 映射起始地址 | size_t: 区域长度 | int: 新的保护标志 | - | - | - | 已实现 | - |
| 11 | munmap | 撤销一段内存映射。 | 2 | void __user *: 映射起始地址 | size_t: 区域长度 | - | - | - | - | 已实现 | - |
| 12 | brk | 调整进程数据段末端。 | 1 | unsigned long: 新的 brk 地址 | - | - | - | - | - | 已实现 | - |
| 13 | rt_sigaction | 安装或读取信号处理动作。 | 4 | int: 信号编号 | const struct sigaction __user *: 新的处理动作 | struct sigaction __user *: 旧的处理动作 | size_t: 信号集字节数 | - | - | 已实现 | - |
| 14 | rt_sigprocmask | 修改或读取线程信号掩码。 | 4 | int: 操作方式 | const sigset_t __user *: 新的信号集 | sigset_t __user *: 旧的信号集 | size_t: 信号集字节数 | - | - | 已实现 | - |
| 15 | rt_sigreturn | 从信号处理上下文返回。 | 0 | - | - | - | - | - | - | 未实现 | - |
| 16 | ioctl | 对文件描述符执行设备控制命令。 | 3 | int: 文件描述符 | unsigned int: 命令号 | unsigned long: 附加参数 | - | - | - | 已实现 | 详细语义依赖 `cmd`；当前只覆盖最小命令集合。 |
| 17 | pread64 | 从给定偏移读取数据而不改变文件偏移。 | 4 | int: 文件描述符 | void __user *: 输出缓冲区 | size_t: 请求读取字节数 | off_t: 读取偏移 | - | - | 未实现 | - |
| 18 | pwrite64 | 向给定偏移写入数据而不改变文件偏移。 | 4 | int: 文件描述符 | const void __user *: 输入缓冲区 | size_t: 请求写入字节数 | off_t: 写入偏移 | - | - | 未实现 | - |
| 19 | readv | 分散读取到多个缓冲区。 | 3 | int: 文件描述符 | const struct iovec __user *: iovec 数组 | int: 数组长度 | - | - | - | 已实现 | - |
| 20 | writev | 聚集写入多个缓冲区。 | 3 | int: 文件描述符 | const struct iovec __user *: iovec 数组 | int: 数组长度 | - | - | - | 已实现 | - |
| 21 | access | 按旧接口检查路径可访问性。 | 2 | const char __user *: 路径字符串 | int: 访问模式 | - | - | - | - | x86_64 兼容别名 | 内部按 `faccessat(AT_FDCWD, pathname, mode, 0)` 兼容处理。 |
| 22 | pipe | 按旧接口创建管道。 | 1 | int __user *: 长度为 2 的文件描述符数组 | - | - | - | - | - | x86_64 兼容别名 | 内部按 `pipe2(pipefd, 0)` 兼容处理。 |
| 23 | select | 等待多组文件描述符事件。 | 5 | int: 最大 fd 加 1 | fd_set __user *: 读集合 | fd_set __user *: 写集合 | fd_set __user *: 异常集合 | struct timeval __user *: 超时 | - | 未实现 | - |
| 24 | sched_yield | 主动让出处理器。 | 0 | - | - | - | - | - | - | 已实现 | - |
| 25 | mremap | 执行 `mremap` 相关内核操作。 | 5 | unsigned long: 地址 | unsigned long: 大小或数量 | unsigned long: 大小或数量 | unsigned long: 控制标志 | unsigned long: 参数5 | - | 未实现 | - |
| 26 | msync | 执行 `msync` 相关内核操作。 | 3 | unsigned long: 起始地址 | size_t: 长度 | int: 控制标志 | - | - | - | 未实现 | - |
| 27 | mincore | 执行 `mincore` 相关内核操作。 | 3 | unsigned long: 起始地址 | size_t: 长度 | unsigned char __user *: 向量数组 | - | - | - | 未实现 | - |
| 28 | madvise | 向内核提供内存访问建议。 | 3 | void __user *: 区域起始地址 | size_t: 区域长度 | int: 建议类型 | - | - | - | 已实现 | 当前为最小 stub。 |
| 29 | shmget | 操作 System V 共享内存并完成 `shmget` 语义。 | 3 | key_t: 参数1 | size_t: 大小 | int: 控制标志 | - | - | - | 未实现 | - |
| 30 | shmat | 操作 System V 共享内存并完成 `shmat` 语义。 | 3 | int: 参数1 | char __user *: 参数2 | int: 参数3 | - | - | - | 未实现 | - |
| 31 | shmctl | 操作 System V 共享内存并完成 `shmctl` 语义。 | 3 | int: 参数1 | int: 命令号 | struct shmid_ds __user *: 输出缓冲区 | - | - | - | 未实现 | - |
| 32 | dup | 复制文件描述符。 | 1 | int: 旧文件描述符 | - | - | - | - | - | 已实现 | - |
| 33 | dup2 | 把文件描述符复制到指定编号。 | 2 | int: 旧文件描述符 | int: 新文件描述符 | - | - | - | - | 未实现 | - |
| 34 | pause | 执行 `pause` 相关内核操作。 | 0 | - | - | - | - | - | - | 未实现 | - |
| 35 | nanosleep | 按相对时间休眠。 | 2 | const struct timespec __user *: 请求休眠时间 | struct timespec __user *: 剩余时间输出 | - | - | - | - | 已实现 | 当前采用忙等加 `yield` 的最小实现。 |
| 36 | getitimer | 读取完成 `getitimer` 语义。 | 2 | int: 对象类型 | struct itimerval __user *: 数值 | - | - | - | - | 未实现 | - |
| 37 | alarm | 执行 `alarm` 相关内核操作。 | 1 | unsigned int: 参数1 | - | - | - | - | - | 未实现 | - |
| 38 | setitimer | 设置完成 `setitimer` 语义。 | 3 | int: 对象类型 | struct itimerval __user *: 数值 | struct itimerval __user *: 参数3 | - | - | - | 未实现 | - |
| 39 | getpid | 获取当前进程 ID。 | 0 | - | - | - | - | - | - | 已实现 | - |
| 40 | sendfile | 在两个文件描述符之间直接搬运数据。 | 4 | int: 输出文件描述符 | int: 输入文件描述符 | off_t __user *: 输入偏移，返回时可更新 | size_t: 搬运字节数 | - | - | 已实现 | - |
| 41 | socket | 创建套接字。 | 3 | int: 协议族 | int: 套接字类型 | int: 协议号 | - | - | - | 已实现 | 当前以 AF_INET 常用路径为主。 |
| 42 | connect | 主动连接到远端地址。 | 3 | int: 套接字描述符 | const struct sockaddr __user *: 远端地址 | int: 地址长度 | - | - | - | 已实现 | 当前采用阻塞轮询的最小实现。 |
| 43 | accept | 接受一个入站连接。 | 3 | int: 监听套接字 | struct sockaddr __user *: 输出对端地址 | int __user *: 输入输出地址长度 | - | - | - | 已实现 | 当前采用阻塞轮询的最小实现。 |
| 44 | sendto | 向套接字发送数据。 | 6 | int: 套接字描述符 | const void __user *: 输入缓冲区 | size_t: 发送字节数 | unsigned int: 发送标志 | const struct sockaddr __user *: 目标地址 | int: 地址长度 | 已实现 | 当前采用阻塞轮询的最小实现。 |
| 45 | recvfrom | 从套接字接收数据。 | 6 | int: 套接字描述符 | void __user *: 输出缓冲区 | size_t: 接收缓冲区大小 | unsigned int: 接收标志 | struct sockaddr __user *: 输出源地址 | int __user *: 输入输出地址长度 | 已实现 | 当前采用阻塞轮询的最小实现。 |
| 46 | sendmsg | 按消息头描述发送套接字数据。 | 3 | int: 套接字描述符 | const struct msghdr __user *: 消息头 | unsigned int: 发送标志 | - | - | - | 未实现 | - |
| 47 | recvmsg | 按消息头描述接收套接字数据。 | 3 | int: 套接字描述符 | struct msghdr __user *: 消息头 | unsigned int: 接收标志 | - | - | - | 未实现 | - |
| 48 | shutdown | 关闭套接字的发送或接收方向。 | 2 | int: 套接字描述符 | int: 关闭方式 | - | - | - | - | 已实现 | - |
| 49 | bind | 给套接字绑定本地地址。 | 3 | int: 套接字描述符 | const struct sockaddr __user *: 本地地址 | int: 地址长度 | - | - | - | 已实现 | - |
| 50 | listen | 把套接字切换到监听状态。 | 2 | int: 套接字描述符 | int: 监听队列长度 | - | - | - | - | 已实现 | - |
| 51 | getsockname | 读取套接字本地地址。 | 3 | int: 套接字描述符 | struct sockaddr __user *: 输出本地地址 | int __user *: 输入输出地址长度 | - | - | - | 已实现 | - |
| 52 | getpeername | 读取套接字对端地址。 | 3 | int: 套接字描述符 | struct sockaddr __user *: 输出对端地址 | int __user *: 输入输出地址长度 | - | - | - | 已实现 | - |
| 53 | socketpair | 创建一对已连接套接字。 | 4 | int: 协议族 | int: 套接字类型 | int: 协议号 | int __user *: 长度为 2 的结果数组 | - | - | 已实现 | - |
| 54 | setsockopt | 设置套接字选项。 | 5 | int: 套接字描述符 | int: 选项层级 | int: 选项名 | const void __user *: 选项值缓冲区 | int: 选项值字节数 | - | 已实现 | 详细语义依赖 `level/optname`；当前只覆盖少量选项。 |
| 55 | getsockopt | 读取套接字选项。 | 5 | int: 套接字描述符 | int: 选项层级 | int: 选项名 | void __user *: 输出选项值缓冲区 | int __user *: 输入输出缓冲区字节数 | - | 已实现 | 详细语义依赖 `level/optname`；当前只覆盖少量选项。 |
| 56 | clone | 按给定标志创建新任务。 | 5 | unsigned long: clone 标志 | void __user *: 子任务栈顶 | int __user *: 父线程 TID 输出地址 | int __user *: 子线程 TID 输出地址 | unsigned long: 线程本地存储 TLS 值 | - | 已实现 | - |
| 57 | fork | 按旧接口复制当前进程。 | 0 | - | - | - | - | - | - | x86_64 兼容别名 | 内部按 `clone(SIGCHLD, 0, 0, 0, 0)` 兼容处理。 |
| 58 | vfork | 按旧接口创建共享地址空间的子进程。 | 0 | - | - | - | - | - | - | x86_64 兼容别名 | 内部按 `clone(CLONE_VFORK + CLONE_VM + SIGCHLD, 0, 0, 0, 0)` 兼容处理。 |
| 59 | execve | 执行新的用户程序映像。 | 3 | const char __user *: 程序路径 | const char __user *const __user *: 参数向量 | const char __user *const __user *: 环境向量 | - | - | - | 已实现 | - |
| 60 | exit | 结束当前线程。 | 1 | int: 退出码 | - | - | - | - | - | 已实现 | - |
| 61 | wait4 | 等待子进程状态变化。 | 4 | pid_t: 目标 pid 或选择器 | int __user *: 输出状态字 | int: 等待选项 | struct rusage __user *: 资源使用统计 | - | - | 已实现 | 返回 Linux 风格状态字，退出码通常需要按 `status >> 8` 解读。 |
| 62 | kill | 向进程发送信号。 | 2 | pid_t: 目标进程 ID | int: 信号编号 | - | - | - | - | 未实现 | - |
| 63 | uname | 读取系统标识信息。 | 1 | struct utsname __user *: 输出系统信息 | - | - | - | - | - | 已实现 | - |
| 64 | semget | 操作 System V 信号量并完成 `semget` 语义。 | 3 | key_t: 参数1 | int: 参数2 | int: 参数3 | - | - | - | 未实现 | - |
| 65 | semop | 操作 System V 信号量并完成 `semop` 语义。 | 3 | int: 参数1 | struct sembuf __user *: 参数2 | unsigned: 参数3 | - | - | - | 未实现 | - |
| 66 | semctl | 操作 System V 信号量并完成 `semctl` 语义。 | 4 | int: 参数1 | int: 参数2 | int: 命令号 | unsigned long: 附加参数 | - | - | 未实现 | - |
| 67 | shmdt | 操作 System V 共享内存并完成 `shmdt` 语义。 | 1 | char __user *: 参数1 | - | - | - | - | - | 未实现 | - |
| 68 | msgget | 操作 System V 消息队列并完成 `msgget` 语义。 | 2 | key_t: 参数1 | int: 参数2 | - | - | - | - | 未实现 | - |
| 69 | msgsnd | 操作 System V 消息队列并完成 `msgsnd` 语义。 | 4 | int: 参数1 | struct msgbuf __user *: 消息缓冲区 | size_t: 参数3 | int: 参数4 | - | - | 未实现 | - |
| 70 | msgrcv | 操作 System V 消息队列并完成 `msgrcv` 语义。 | 5 | int: 参数1 | struct msgbuf __user *: 消息缓冲区 | size_t: 参数3 | long: 参数4 | int: 参数5 | - | 未实现 | - |
| 71 | msgctl | 操作 System V 消息队列并完成 `msgctl` 语义。 | 3 | int: 参数1 | int: 命令号 | struct msqid_ds __user *: 输出缓冲区 | - | - | - | 未实现 | - |
| 72 | fcntl | 对文件描述符执行控制命令。 | 3 | int: 文件描述符 | int: 控制命令 | unsigned long: 附加参数 | - | - | - | 已实现 | 详细语义依赖 `cmd`；当前仅覆盖常用子集。 |
| 73 | flock | 执行 `flock` 相关内核操作。 | 2 | unsigned int: 文件描述符 | unsigned int: 命令号 | - | - | - | - | 未实现 | - |
| 74 | fsync | 把文件数据同步到稳定存储。 | 1 | int: 文件描述符 | - | - | - | - | - | 已实现 | - |
| 75 | fdatasync | 把文件数据同步到稳定存储但尽量少同步元数据。 | 1 | int: 文件描述符 | - | - | - | - | - | 未实现 | - |
| 76 | truncate | 按路径调整文件长度。 | 2 | const char __user *: 路径字符串 | off_t: 新长度 | - | - | - | - | 已实现 | - |
| 77 | ftruncate | 按文件描述符调整文件长度。 | 2 | int: 文件描述符 | off_t: 新长度 | - | - | - | - | 已实现 | - |
| 78 | getdents | 读取完成 `getdents` 语义。 | 3 | unsigned int: 文件描述符 | struct linux_dirent __user *: 参数2 | unsigned int: 字节数 | - | - | - | 未实现 | - |
| 79 | getcwd | 读取当前工作目录。 | 2 | char __user *: 输出缓冲区 | unsigned long: 缓冲区大小 | - | - | - | - | 已实现 | - |
| 80 | chdir | 切换当前工作目录。 | 1 | const char __user *: 路径字符串 | - | - | - | - | - | 已实现 | - |
| 81 | fchdir | 切换到文件描述符指向的目录。 | 1 | int: 目录文件描述符 | - | - | - | - | - | 未实现 | - |
| 82 | rename | 按旧接口重命名路径。 | 2 | const char __user *: 旧路径 | const char __user *: 新路径 | - | - | - | - | 未实现 | - |
| 83 | mkdir | 按旧接口创建目录。 | 2 | const char __user *: 路径字符串 | mode_t: 目录权限位 | - | - | - | - | x86_64 兼容别名 | 内部按 `mkdirat(AT_FDCWD, pathname, mode)` 兼容处理。 |
| 84 | rmdir | 删除空目录。 | 1 | const char __user *: 路径字符串 | - | - | - | - | - | 未实现 | - |
| 85 | creat | 执行 `creat` 相关内核操作。 | 2 | const char __user *: 路径字符串 | mode_t: 权限或模式标志 | - | - | - | - | 未实现 | - |
| 86 | link | 按旧接口创建硬链接。 | 2 | const char __user *: 旧路径 | const char __user *: 新路径 | - | - | - | - | 未实现 | - |
| 87 | unlink | 按旧接口删除目录项。 | 1 | const char __user *: 路径字符串 | - | - | - | - | - | 未实现 | - |
| 88 | symlink | 按旧接口创建符号链接。 | 2 | const char __user *: 目标路径 | const char __user *: 链接路径 | - | - | - | - | 未实现 | - |
| 89 | readlink | 按旧接口读取符号链接内容。 | 3 | const char __user *: 路径字符串 | char __user *: 输出缓冲区 | int: 缓冲区大小 | - | - | - | 未实现 | - |
| 90 | chmod | 按路径修改权限位。 | 2 | const char __user *: 路径字符串 | mode_t: 新的权限位 | - | - | - | - | 未实现 | - |
| 91 | fchmod | 按文件描述符修改权限位。 | 2 | unsigned int: 文件描述符 | mode_t: 新的权限位 | - | - | - | - | 未实现 | - |
| 92 | chown | 执行 `chown` 相关内核操作。 | 3 | const char __user *: 路径字符串 | uid_t: 参数2 | gid_t: 参数3 | - | - | - | 未实现 | - |
| 93 | fchown | 执行 `fchown` 相关内核操作。 | 3 | unsigned int: 文件描述符 | uid_t: 参数2 | gid_t: 参数3 | - | - | - | 未实现 | - |
| 94 | lchown | 执行 `lchown` 相关内核操作。 | 3 | const char __user *: 路径字符串 | uid_t: 参数2 | gid_t: 参数3 | - | - | - | 未实现 | - |
| 95 | umask | 设置并返回进程文件创建掩码。 | 1 | int: 新的掩码值 | - | - | - | - | - | 未实现 | - |
| 96 | gettimeofday | 读取墙上时钟时间。 | 2 | struct timeval __user *: 输出时间值 | struct timezone __user *: 历史时区参数 | - | - | - | - | 已实现 | - |
| 97 | getrlimit | 读取资源限制。 | 2 | unsigned int: 资源类型 | struct rlimit __user *: 输出限制 | - | - | - | - | 未实现 | - |
| 98 | getrusage | 读取资源使用统计。 | 2 | int: 统计对象 | struct rusage __user *: 输出统计 | - | - | - | - | 未实现 | - |
| 99 | sysinfo | 读取系统资源概况。 | 1 | struct sysinfo __user *: 输出系统信息 | - | - | - | - | - | 未实现 | - |
| 100 | times | 读取进程与子进程 CPU 时间统计。 | 1 | struct tms __user *: 输出时间统计 | - | - | - | - | - | 未实现 | - |
| 101 | ptrace | 执行 `ptrace` 相关内核操作。 | 4 | long: 参数1 | long: 进程 ID | unsigned long: 地址 | unsigned long: 数据指针 | - | - | 未实现 | - |
| 102 | getuid | 获取真实用户 ID。 | 0 | - | - | - | - | - | - | 已实现 | 当前固定返回 0。 |
| 103 | syslog | 执行 `syslog` 相关内核操作。 | 3 | int: 类型编号 | char __user *: 输出缓冲区 | int: 长度 | - | - | - | 未实现 | - |
| 104 | getgid | 获取真实组 ID。 | 0 | - | - | - | - | - | - | 已实现 | 当前固定返回 0。 |
| 105 | setuid | 设置完成 `setuid` 语义。 | 1 | uid_t: 用户 ID | - | - | - | - | - | 未实现 | - |
| 106 | setgid | 设置完成 `setgid` 语义。 | 1 | gid_t: 组 ID | - | - | - | - | - | 未实现 | - |
| 107 | geteuid | 获取有效用户 ID。 | 0 | - | - | - | - | - | - | 已实现 | 当前固定返回 0。 |
| 108 | getegid | 获取有效组 ID。 | 0 | - | - | - | - | - | - | 已实现 | 当前固定返回 0。 |
| 109 | setpgid | 设置进程组 ID。 | 2 | pid_t: 目标进程 ID | pid_t: 目标进程组 ID | - | - | - | - | 已实现 | 当前为最小 stub，未维护真实进程组。 |
| 110 | getppid | 获取父进程 ID。 | 0 | - | - | - | - | - | - | 已实现 | - |
| 111 | getpgrp | 读取完成 `getpgrp` 语义。 | 0 | - | - | - | - | - | - | 未实现 | - |
| 112 | setsid | 创建新会话并成为会话首进程。 | 0 | - | - | - | - | - | - | 已实现 | 当前为最小 stub，未维护真实会话。 |
| 113 | setreuid | 设置完成 `setreuid` 语义。 | 2 | uid_t: 用户 ID | uid_t: 用户 ID | - | - | - | - | 未实现 | - |
| 114 | setregid | 设置完成 `setregid` 语义。 | 2 | gid_t: 组 ID | gid_t: 组 ID | - | - | - | - | 未实现 | - |
| 115 | getgroups | 读取完成 `getgroups` 语义。 | 2 | int: 大小或数量 | gid_t __user *: 参数2 | - | - | - | - | 未实现 | - |
| 116 | setgroups | 设置完成 `setgroups` 语义。 | 2 | int: 大小或数量 | gid_t __user *: 参数2 | - | - | - | - | 未实现 | - |
| 117 | setresuid | 设置完成 `setresuid` 语义。 | 3 | uid_t: 用户 ID | uid_t: 用户 ID | uid_t: 用户 ID | - | - | - | 未实现 | - |
| 118 | getresuid | 读取完成 `getresuid` 语义。 | 3 | uid_t __user *: 用户 ID | uid_t __user *: 用户 ID | uid_t __user *: 用户 ID | - | - | - | 未实现 | - |
| 119 | setresgid | 设置完成 `setresgid` 语义。 | 3 | gid_t: 组 ID | gid_t: 组 ID | gid_t: 组 ID | - | - | - | 未实现 | - |
| 120 | getresgid | 读取完成 `getresgid` 语义。 | 3 | gid_t __user *: 组 ID | gid_t __user *: 组 ID | gid_t __user *: 组 ID | - | - | - | 未实现 | - |
| 121 | getpgid | 读取进程组 ID。 | 1 | pid_t: 目标进程 ID | - | - | - | - | - | 已实现 | 当前为最小 stub，未维护真实进程组。 |
| 122 | setfsuid | 设置完成 `setfsuid` 语义。 | 1 | uid_t: 用户 ID | - | - | - | - | - | 未实现 | - |
| 123 | setfsgid | 设置完成 `setfsgid` 语义。 | 1 | gid_t: 组 ID | - | - | - | - | - | 未实现 | - |
| 124 | getsid | 读取完成 `getsid` 语义。 | 1 | pid_t: 进程 ID | - | - | - | - | - | 未实现 | - |
| 125 | capget | 执行 `capget` 相关内核操作。 | 2 | cap_user_header_t: 参数1 | cap_user_data_t: 参数2 | - | - | - | - | 未实现 | - |
| 126 | capset | 执行 `capset` 相关内核操作。 | 2 | cap_user_header_t: 参数1 | const cap_user_data_t: 数据指针 | - | - | - | - | 未实现 | - |
| 127 | rt_sigpending | 执行 `rt_sigpending` 相关内核操作。 | 2 | sigset_t __user *: 输入集合 | size_t: 信号集字节数 | - | - | - | - | 未实现 | - |
| 128 | rt_sigtimedwait | 执行 `rt_sigtimedwait` 相关内核操作。 | 4 | const sigset_t __user *: 参数1 | siginfo_t __user *: 参数2 | const struct timespec __user *: 参数3 | size_t: 信号集字节数 | - | - | 未实现 | - |
| 129 | rt_sigqueueinfo | 执行 `rt_sigqueueinfo` 相关内核操作。 | 3 | pid_t: 进程 ID | int: 信号编号 | siginfo_t __user *: 参数3 | - | - | - | 未实现 | - |
| 130 | rt_sigsuspend | 执行 `rt_sigsuspend` 相关内核操作。 | 2 | sigset_t __user *: 参数1 | size_t: 信号集字节数 | - | - | - | - | 未实现 | - |
| 131 | sigaltstack | 设置或读取备用信号栈。 | 2 | const stack_t __user *: 新的备用栈 | stack_t __user *: 旧的备用栈 | - | - | - | - | 已实现 | - |
| 132 | utime | 执行 `utime` 相关内核操作。 | 2 | char __user *: 路径字符串 | struct utimbuf __user *: 参数2 | - | - | - | - | 未实现 | - |
| 133 | mknod | 执行 `mknod` 相关内核操作。 | 3 | const char __user *: 路径字符串 | mode_t: 权限或模式标志 | unsigned: 参数3 | - | - | - | 未实现 | - |
| 135 | personality | 执行 `personality` 相关内核操作。 | 1 | unsigned int: 参数1 | - | - | - | - | - | 未实现 | - |
| 136 | ustat | 执行 `ustat` 相关内核操作。 | 2 | unsigned: 参数1 | struct ustat __user *: 输出缓冲区 | - | - | - | - | 未实现 | - |
| 137 | statfs | 按路径查询文件系统状态。 | 2 | const char __user *: 路径字符串 | struct statfs __user *: 输出文件系统状态 | - | - | - | - | 已实现 | 当前只回填最小统计字段。 |
| 138 | fstatfs | 按文件描述符查询文件系统状态。 | 2 | unsigned int: 文件描述符 | struct statfs __user *: 输出文件系统状态 | - | - | - | - | 已实现 | 当前只回填最小统计字段。 |
| 139 | sysfs | 执行 `sysfs` 相关内核操作。 | 3 | int: 选项值 | unsigned long: 附加参数 | unsigned long: 附加参数 | - | - | - | 未实现 | - |
| 140 | getpriority | 读取进程、进程组或用户的 nice 值。 | 2 | int: 对象类型 | int: 对象 ID | - | - | - | - | 已实现 | - |
| 141 | setpriority | 设置进程、进程组或用户的 nice 值。 | 3 | int: 对象类型 | int: 对象 ID | int: 新的优先级 | - | - | - | 已实现 | - |
| 142 | sched_setparam | 操作调度器并完成 `sched_setparam` 语义。 | 2 | pid_t: 进程 ID | struct sched_param __user *: 参数2 | - | - | - | - | 未实现 | - |
| 143 | sched_getparam | 操作调度器并完成 `sched_getparam` 语义。 | 2 | pid_t: 进程 ID | struct sched_param __user *: 参数2 | - | - | - | - | 未实现 | - |
| 144 | sched_setscheduler | 操作调度器并完成 `sched_setscheduler` 语义。 | 3 | pid_t: 进程 ID | int: 参数2 | struct sched_param __user *: 参数3 | - | - | - | 未实现 | - |
| 145 | sched_getscheduler | 操作调度器并完成 `sched_getscheduler` 语义。 | 1 | pid_t: 进程 ID | - | - | - | - | - | 未实现 | - |
| 146 | sched_get_priority_max | 操作调度器并完成 `sched_get_priority_max` 语义。 | 1 | int: 参数1 | - | - | - | - | - | 未实现 | - |
| 147 | sched_get_priority_min | 操作调度器并完成 `sched_get_priority_min` 语义。 | 1 | int: 参数1 | - | - | - | - | - | 未实现 | - |
| 148 | sched_rr_get_interval | 操作调度器并完成 `sched_rr_get_interval` 语义。 | 2 | pid_t: 进程 ID | struct timespec __user *: 参数2 | - | - | - | - | 未实现 | - |
| 149 | mlock | 执行 `mlock` 相关内核操作。 | 2 | unsigned long: 起始地址 | size_t: 长度 | - | - | - | - | 未实现 | - |
| 150 | munlock | 执行 `munlock` 相关内核操作。 | 2 | unsigned long: 起始地址 | size_t: 长度 | - | - | - | - | 未实现 | - |
| 151 | mlockall | 执行 `mlockall` 相关内核操作。 | 1 | int: 控制标志 | - | - | - | - | - | 未实现 | - |
| 152 | munlockall | 执行 `munlockall` 相关内核操作。 | 0 | - | - | - | - | - | - | 未实现 | - |
| 153 | vhangup | 执行 `vhangup` 相关内核操作。 | 0 | - | - | - | - | - | - | 未实现 | - |
| 154 | modify_ldt | 执行 `modify_ldt` 相关内核操作。 | 3 | int: 参数1 | void __user *: 指针参数 | unsigned long: 大小或数量 | - | - | - | 未实现 | - |
| 155 | pivot_root | 执行 `pivot_root` 相关内核操作。 | 2 | const char __user *: 参数1 | const char __user *: 参数2 | - | - | - | - | 未实现 | - |
| 156 | _sysctl | 读取或设置旧式内核参数树。 | 1 | struct __sysctl_args __user *: sysctl 参数结构 | - | - | - | - | - | 未实现 | Linux 主线当前为保留或废弃入口。 |
| 157 | prctl | 执行 `prctl` 相关内核操作。 | 5 | int: 选项值 | unsigned long: 附加参数 | unsigned long: 参数3 | unsigned long: 参数4 | unsigned long: 参数5 | - | 未实现 | - |
| 158 | arch_prctl | 读取或设置 x86_64 线程架构状态。 | 2 | int: 控制命令 | unsigned long: 地址或输出缓冲区 | - | - | - | - | 已实现 | 仅 x86_64 提供此 syscall。 |
| 159 | adjtimex | 执行 `adjtimex` 相关内核操作。 | 1 | struct __kernel_timex __user *: 参数1 | - | - | - | - | - | 未实现 | - |
| 160 | setrlimit | 设置完成 `setrlimit` 语义。 | 2 | unsigned int: 资源类型 | struct rlimit __user *: 资源限制结构 | - | - | - | - | 未实现 | - |
| 161 | chroot | 执行 `chroot` 相关内核操作。 | 1 | const char __user *: 路径字符串 | - | - | - | - | - | 未实现 | - |
| 162 | sync | 执行 `sync` 相关内核操作。 | 0 | - | - | - | - | - | - | 未实现 | - |
| 163 | acct | 执行 `acct` 相关内核操作。 | 1 | const char __user *: 名称字符串 | - | - | - | - | - | 未实现 | - |
| 164 | settimeofday | 设置完成 `settimeofday` 语义。 | 2 | struct timeval __user *: 时间值结构 | struct timezone __user *: 时区结构 | - | - | - | - | 未实现 | - |
| 165 | mount | 挂载文件系统。 | 5 | const char __user *: 源设备或伪设备名 | const char __user *: 挂载点路径 | const char __user *: 文件系统类型 | unsigned long: 挂载标志 | const void __user *: 文件系统私有数据 | - | 未实现 | 当前直接返回 `ENOSYS`。 |
| 166 | umount2 | 卸载文件系统。 | 2 | char __user *: 名称字符串 | int: 控制标志 | - | - | - | - | 未实现 | - |
| 167 | swapon | 执行 `swapon` 相关内核操作。 | 2 | const char __user *: 设备节点路径 | int: 控制标志 | - | - | - | - | 未实现 | - |
| 168 | swapoff | 执行 `swapoff` 相关内核操作。 | 1 | const char __user *: 设备节点路径 | - | - | - | - | - | 未实现 | - |
| 169 | reboot | 执行 `reboot` 相关内核操作。 | 4 | int: 参数1 | int: 参数2 | unsigned int: 命令号 | void __user *: 附加参数 | - | - | 未实现 | - |
| 170 | sethostname | 设置完成 `sethostname` 语义。 | 2 | char __user *: 名称字符串 | int: 长度 | - | - | - | - | 未实现 | - |
| 171 | setdomainname | 设置完成 `setdomainname` 语义。 | 2 | char __user *: 名称字符串 | int: 长度 | - | - | - | - | 未实现 | - |
| 172 | iopl | 执行 `iopl` 相关内核操作。 | 1 | int: 选项层级 | - | - | - | - | - | 未实现 | - |
| 173 | ioperm | 执行 `ioperm` 相关内核操作。 | 3 | unsigned long: 源路径 | unsigned long: 参数2 | int: 参数3 | - | - | - | 未实现 | - |
| 175 | init_module | 执行 `init_module` 相关内核操作。 | 3 | void __user *: 参数1 | unsigned long: 长度 | const char __user *: 参数3 | - | - | - | 未实现 | - |
| 176 | delete_module | 执行 `delete_module` 相关内核操作。 | 2 | const char __user *: 路径或名称字符串 | unsigned int: 控制标志 | - | - | - | - | 未实现 | - |
| 179 | quotactl | 执行 `quotactl` 相关内核操作。 | 4 | unsigned int: 命令号 | const char __user *: 参数2 | qid_t: 参数3 | void __user *: 地址 | - | - | 未实现 | - |
| 186 | gettid | 获取当前线程 ID。 | 0 | - | - | - | - | - | - | 已实现 | - |
| 187 | readahead | 执行 `readahead` 相关内核操作。 | 3 | int: 文件描述符 | loff_t: 偏移量 | size_t: 字节数 | - | - | - | 未实现 | - |
| 188 | setxattr | 设置路径扩展属性。 | 5 | const char __user *: 路径字符串 | const char __user *: 属性名 | const void __user *: 属性值缓冲区 | size_t: 属性值字节数 | int: 行为标志 | - | 未实现 | 当前直接返回 `ENOSYS`。 |
| 189 | lsetxattr | 设置符号链接自身的扩展属性。 | 5 | const char __user *: 路径字符串 | const char __user *: 属性名 | const void __user *: 属性值缓冲区 | size_t: 属性值字节数 | int: 行为标志 | - | 未实现 | 当前直接返回 `ENOSYS`。 |
| 190 | fsetxattr | 设置文件描述符对应对象的扩展属性。 | 5 | int: 文件描述符 | const char __user *: 属性名 | const void __user *: 属性值缓冲区 | size_t: 属性值字节数 | int: 行为标志 | - | 未实现 | 当前直接返回 `ENOSYS`。 |
| 191 | getxattr | 读取路径扩展属性。 | 4 | const char __user *: 路径字符串 | const char __user *: 属性名 | void __user *: 输出属性值缓冲区 | size_t: 缓冲区字节数 | - | - | 未实现 | 当前直接返回 `ENOSYS`。 |
| 192 | lgetxattr | 读取符号链接自身的扩展属性。 | 4 | const char __user *: 路径字符串 | const char __user *: 属性名 | void __user *: 输出属性值缓冲区 | size_t: 缓冲区字节数 | - | - | 未实现 | 当前直接返回 `ENOSYS`。 |
| 193 | fgetxattr | 读取文件描述符对应对象的扩展属性。 | 4 | int: 文件描述符 | const char __user *: 属性名 | void __user *: 输出属性值缓冲区 | size_t: 缓冲区字节数 | - | - | 未实现 | 当前直接返回 `ENOSYS`。 |
| 194 | listxattr | 列出路径上的扩展属性名。 | 3 | const char __user *: 路径字符串 | char __user *: 输出名称缓冲区 | size_t: 缓冲区字节数 | - | - | - | 未实现 | 当前直接返回 `ENOSYS`。 |
| 195 | llistxattr | 列出符号链接自身的扩展属性名。 | 3 | const char __user *: 路径字符串 | char __user *: 输出名称缓冲区 | size_t: 缓冲区字节数 | - | - | - | 未实现 | 当前直接返回 `ENOSYS`。 |
| 196 | flistxattr | 列出文件描述符对应对象的扩展属性名。 | 3 | int: 文件描述符 | char __user *: 输出名称缓冲区 | size_t: 缓冲区字节数 | - | - | - | 未实现 | 当前直接返回 `ENOSYS`。 |
| 197 | removexattr | 删除路径上的扩展属性。 | 2 | const char __user *: 路径字符串 | const char __user *: 属性名 | - | - | - | - | 未实现 | 当前直接返回 `ENOSYS`。 |
| 198 | lremovexattr | 删除符号链接自身的扩展属性。 | 2 | const char __user *: 路径字符串 | const char __user *: 属性名 | - | - | - | - | 未实现 | 当前直接返回 `ENOSYS`。 |
| 199 | fremovexattr | 删除文件描述符对应对象的扩展属性。 | 2 | int: 文件描述符 | const char __user *: 属性名 | - | - | - | - | 未实现 | 当前直接返回 `ENOSYS`。 |
| 200 | tkill | 向指定线程发送信号。 | 2 | pid_t: 进程 ID | int: 信号编号 | - | - | - | - | 未实现 | - |
| 201 | time | 执行 `time` 相关内核操作。 | 1 | __kernel_old_time_t __user *: 时间输出地址 | - | - | - | - | - | 未实现 | - |
| 202 | futex | 执行 futex 等待、唤醒等原子同步操作。 | 6 | uint32_t __user *: futex 字地址 | int: futex 操作 | uint32_t: 附加整数参数 | const struct timespec __user *: 超时或附加参数 | uint32_t __user *: 第二个 futex 地址 | uint32_t: 附加整数参数 | 已实现 | - |
| 203 | sched_setaffinity | 操作调度器并完成 `sched_setaffinity` 语义。 | 3 | pid_t: 进程 ID | unsigned int: 长度 | unsigned long __user *: 参数3 | - | - | - | 未实现 | - |
| 204 | sched_getaffinity | 操作调度器并完成 `sched_getaffinity` 语义。 | 3 | pid_t: 进程 ID | unsigned int: 长度 | unsigned long __user *: 参数3 | - | - | - | 未实现 | - |
| 206 | io_setup | 操作异步 I/O 并完成 `io_setup` 语义。 | 2 | unsigned: 参数1 | aio_context_t __user *: 参数2 | - | - | - | - | 未实现 | - |
| 207 | io_destroy | 操作异步 I/O 并完成 `io_destroy` 语义。 | 1 | aio_context_t: 参数1 | - | - | - | - | - | 未实现 | - |
| 208 | io_getevents | 操作异步 I/O 并完成 `io_getevents` 语义。 | 5 | aio_context_t: 参数1 | long: 参数2 | long: 编号 | struct io_event __user *: 参数4 | struct timespec __user *: 超时 | - | 未实现 | - |
| 209 | io_submit | 操作异步 I/O 并完成 `io_submit` 语义。 | 3 | a: 参数1 | l: 参数2 | struct iocb __user *__user *: 参数3 | - | - | - | 未实现 | - |
| 210 | io_cancel | 操作异步 I/O 并完成 `io_cancel` 语义。 | 3 | aio_context_t: 参数1 | struct iocb __user *: 参数2 | struct io_event __user *: 参数3 | - | - | - | 未实现 | - |
| 213 | epoll_create | 操作 epoll 并完成 `epoll_create` 语义。 | 1 | int: 大小 | - | - | - | - | - | 未实现 | - |
| 216 | remap_file_pages | 执行 `remap_file_pages` 相关内核操作。 | 5 | unsigned long: 起始地址 | unsigned long: 大小 | unsigned long: 保护标志 | unsigned long: 参数4 | unsigned long: 控制标志 | - | 未实现 | - |
| 217 | getdents64 | 读取目录项数组。 | 3 | unsigned int: 目录文件描述符 | struct linux_dirent64 __user *: 输出目录项缓冲区 | unsigned int: 缓冲区大小 | - | - | - | 已实现 | - |
| 218 | set_tid_address | 登记线程退出时写回的 TID 地址。 | 1 | int __user *: 用户态 TID 地址 | - | - | - | - | - | 已实现 | - |
| 219 | restart_syscall | 执行 `restart_syscall` 相关内核操作。 | 0 | - | - | - | - | - | - | 未实现 | - |
| 220 | semtimedop | 操作 System V 信号量并完成 `semtimedop` 语义。 | 4 | int: 参数1 | struct sembuf __user *: 参数2 | unsigned: 参数3 | const struct timespec __user *: 超时 | - | - | 未实现 | - |
| 221 | fadvise64 | 执行 `fadvise64` 相关内核操作。 | 4 | int: 文件描述符 | loff_t: 偏移量 | size_t: 长度 | int: 参数4 | - | - | 未实现 | - |
| 222 | timer_create | 操作 POSIX 定时器并完成 `timer_create` 语义。 | 3 | clockid_t: 参数1 | struct sigevent __user *: 参数2 | timer_t __user *: 参数3 | - | - | - | 未实现 | - |
| 223 | timer_settime | 操作 POSIX 定时器并完成 `timer_settime` 语义。 | 4 | timer_t: 参数1 | int: 控制标志 | const struct __kernel_itimerspec __user *: 参数3 | struct __kernel_itimerspec __user *: 参数4 | - | - | 未实现 | - |
| 224 | timer_gettime | 操作 POSIX 定时器并完成 `timer_gettime` 语义。 | 2 | timer_t: 参数1 | struct __kernel_itimerspec __user *: 参数2 | - | - | - | - | 未实现 | - |
| 225 | timer_getoverrun | 操作 POSIX 定时器并完成 `timer_getoverrun` 语义。 | 1 | timer_t: 参数1 | - | - | - | - | - | 未实现 | - |
| 226 | timer_delete | 操作 POSIX 定时器并完成 `timer_delete` 语义。 | 1 | timer_t: 参数1 | - | - | - | - | - | 未实现 | - |
| 227 | clock_settime | 操作时钟并完成 `clock_settime` 语义。 | 2 | clockid_t: 参数1 | const struct timespec __user *: 参数2 | - | - | - | - | 未实现 | - |
| 228 | clock_gettime | 读取指定时钟的当前时间。 | 2 | clockid_t: 时钟编号 | struct timespec __user *: 输出时间值 | - | - | - | - | 已实现 | 当前只覆盖部分常用 clock id，其余分支未完整实现。 |
| 229 | clock_getres | 操作时钟并完成 `clock_getres` 语义。 | 2 | clockid_t: 参数1 | struct timespec __user *: 参数2 | - | - | - | - | 未实现 | - |
| 230 | clock_nanosleep | 按指定时钟休眠。 | 4 | clockid_t: 时钟编号 | int: 控制标志 | const struct timespec __user *: 请求时间 | struct timespec __user *: 剩余时间输出 | - | - | 未实现 | - |
| 231 | exit_group | 结束整个线程组。 | 1 | int: 退出码 | - | - | - | - | - | 已实现 | - |
| 232 | epoll_wait | 操作 epoll 并完成 `epoll_wait` 语义。 | 4 | int: epoll 文件描述符 | struct epoll_event __user *: 参数2 | int: 参数3 | int: 超时 | - | - | 未实现 | - |
| 233 | epoll_ctl | 管理 epoll 关注的文件描述符。 | 4 | int: epoll 文件描述符 | int: 控制操作 | int: 目标文件描述符 | struct epoll_event __user *: 事件配置 | - | - | 已实现 | - |
| 234 | tgkill | 向指定线程组中的线程发送信号。 | 3 | pid_t: 线程组 ID | pid_t: 进程 ID | int: 信号编号 | - | - | - | 未实现 | - |
| 235 | utimes | 执行 `utimes` 相关内核操作。 | 2 | char __user *: 路径字符串 | struct timeval __user *: 时间戳数组 | - | - | - | - | 未实现 | - |
| 237 | mbind | 执行 `mbind` 相关内核操作。 | 6 | unsigned long: 起始地址 | unsigned long: 长度 | unsigned long: 权限或模式标志 | const unsigned long __user *: 参数4 | unsigned long: 参数5 | unsigned: 控制标志 | 未实现 | - |
| 238 | set_mempolicy | 设置完成 `set_mempolicy` 语义。 | 3 | int: 权限或模式标志 | const unsigned long __user *: 参数2 | unsigned long: 参数3 | - | - | - | 未实现 | - |
| 239 | get_mempolicy | 读取完成 `get_mempolicy` 语义。 | 5 | int __user *: 参数1 | unsigned long __user *: 参数2 | unsigned long: 参数3 | unsigned long: 地址 | unsigned long: 控制标志 | - | 未实现 | - |
| 240 | mq_open | 操作 POSIX 消息队列并完成 `mq_open` 语义。 | 4 | const char __user *: 名称字符串 | int: 控制标志 | mode_t: 权限或模式标志 | struct mq_attr __user *: 参数4 | - | - | 未实现 | - |
| 241 | mq_unlink | 操作 POSIX 消息队列并完成 `mq_unlink` 语义。 | 1 | const char __user *: 名称字符串 | - | - | - | - | - | 未实现 | - |
| 242 | mq_timedsend | 操作 POSIX 消息队列并完成 `mq_timedsend` 语义。 | 5 | mqd_t: 参数1 | const char __user *: 参数2 | size_t: 大小或数量 | unsigned int: 参数4 | const struct timespec __user *: 参数5 | - | 未实现 | - |
| 243 | mq_timedreceive | 操作 POSIX 消息队列并完成 `mq_timedreceive` 语义。 | 5 | mqd_t: 参数1 | char __user *: 参数2 | size_t: 大小或数量 | unsigned int __user *: 参数4 | const struct timespec __user *: 参数5 | - | 未实现 | - |
| 244 | mq_notify | 操作 POSIX 消息队列并完成 `mq_notify` 语义。 | 2 | mqd_t: 参数1 | const struct sigevent __user *: 参数2 | - | - | - | - | 未实现 | - |
| 245 | mq_getsetattr | 操作 POSIX 消息队列并完成 `mq_getsetattr` 语义。 | 3 | mqd_t: 参数1 | const struct mq_attr __user *: 参数2 | struct mq_attr __user *: 参数3 | - | - | - | 未实现 | - |
| 246 | kexec_load | 执行 `kexec_load` 相关内核操作。 | 4 | unsigned long: 参数1 | unsigned long: 参数2 | struct kexec_segment __user *: 参数3 | unsigned long: 控制标志 | - | - | 未实现 | - |
| 247 | waitid | 等待子进程状态变化并回填 `siginfo`。 | 5 | int: 等待对象类型 | pid_t: 目标 ID | struct siginfo __user *: 输出信号信息 | int: 等待选项 | struct rusage __user *: 资源使用统计 | - | 已实现 | 当前仅覆盖最常用 idtype，`siginfo` 只回填最小字段。 |
| 248 | add_key | 添加内核密钥并完成 `add_key` 语义。 | 5 | const char __user *: 参数1 | const char __user *: 参数2 | const void __user *: 参数3 | size_t: 大小或数量 | key_serial_t: 组 ID | - | 未实现 | - |
| 249 | request_key | 请求内核密钥并完成 `request_key` 语义。 | 4 | const char __user *: 参数1 | const char __user *: 参数2 | const char __user *: 参数3 | key_serial_t: 组 ID | - | - | 未实现 | - |
| 250 | keyctl | 管理内核密钥并完成 `keyctl` 语义。 | 5 | int: 命令号 | unsigned long: 附加参数 | unsigned long: 参数3 | unsigned long: 参数4 | unsigned long: 参数5 | - | 未实现 | - |
| 251 | ioprio_set | 执行 `ioprio_set` 相关内核操作。 | 3 | int: 对象类型 | int: 对象 ID | int: 参数3 | - | - | - | 未实现 | - |
| 252 | ioprio_get | 执行 `ioprio_get` 相关内核操作。 | 2 | int: 对象类型 | int: 对象 ID | - | - | - | - | 未实现 | - |
| 253 | inotify_init | 操作 inotify 并完成 `inotify_init` 语义。 | 0 | - | - | - | - | - | - | 未实现 | - |
| 254 | inotify_add_watch | 操作 inotify 并完成 `inotify_add_watch` 语义。 | 3 | int: 文件描述符 | const char __user *: 路径字符串 | u32: 掩码 | - | - | - | 未实现 | - |
| 255 | inotify_rm_watch | 操作 inotify 并完成 `inotify_rm_watch` 语义。 | 2 | int: 文件描述符 | __s32: 参数2 | - | - | - | - | 未实现 | - |
| 256 | migrate_pages | 执行 `migrate_pages` 相关内核操作。 | 4 | pid_t: 进程 ID | unsigned long: 参数2 | const unsigned long __user *: 源路径 | const unsigned long __user *: 目标路径 | - | - | 未实现 | - |
| 257 | openat | 相对目录文件描述符打开文件。 | 4 | int: 目录文件描述符 | const char __user *: 路径字符串 | int: 打开标志 | mode_t: 创建时权限位 | - | - | 已实现 | - |
| 258 | mkdirat | 相对目录文件描述符创建目录。 | 3 | int: 目录文件描述符 | const char __user *: 路径字符串 | mode_t: 目录权限位 | - | - | - | 已实现 | - |
| 259 | mknodat | 执行 `mknodat` 相关内核操作。 | 4 | int: 目录文件描述符 | const char __user *: 路径字符串 | mode_t: 权限或模式标志 | unsigned: 参数4 | - | - | 未实现 | - |
| 260 | fchownat | 相对目录文件描述符修改属主和属组。 | 5 | int: 目录文件描述符 | const char __user *: 路径字符串 | uid_t: 新的用户 ID | gid_t: 新的组 ID | int: 控制标志 | - | 未实现 | - |
| 261 | futimesat | 执行 `futimesat` 相关内核操作。 | 3 | int: 目录文件描述符 | const char __user *: 路径字符串 | struct timeval __user *: 时间戳数组 | - | - | - | 未实现 | - |
| 262 | newfstatat | 相对目录文件描述符查询文件状态。 | 4 | int: 目录文件描述符 | const char __user *: 路径字符串 | struct stat __user *: 输出文件状态 | int: 查找标志 | - | - | 已实现 | - |
| 263 | unlinkat | 相对目录文件描述符删除目录项。 | 3 | int: 目录文件描述符 | const char __user *: 路径字符串 | int: 删除标志 | - | - | - | 已实现 | - |
| 264 | renameat | 相对目录文件描述符重命名路径。 | 4 | int: 旧目录文件描述符 | const char __user *: 旧路径 | int: 新目录文件描述符 | const char __user *: 新路径 | - | - | 已实现 | 内部复用 `renameat2(..., flags=0)`。 |
| 265 | linkat | 相对目录文件描述符创建硬链接。 | 5 | int: 旧目录文件描述符 | const char __user *: 旧路径 | int: 新目录文件描述符 | const char __user *: 新路径 | int: 链接标志 | - | 已实现 | - |
| 266 | symlinkat | 相对目录文件描述符创建符号链接。 | 3 | const char __user *: 目标路径 | int: 目录文件描述符 | const char __user *: 链接路径 | - | - | - | 已实现 | - |
| 267 | readlinkat | 相对目录文件描述符读取符号链接内容。 | 4 | int: 目录文件描述符 | const char __user *: 路径字符串 | char __user *: 输出缓冲区 | int: 缓冲区大小 | - | - | 已实现 | - |
| 268 | fchmodat | 相对目录文件描述符修改权限位。 | 3 | int: 目录文件描述符 | const char __user *: 路径字符串 | mode_t: 新的权限位 | - | - | - | 未实现 | - |
| 269 | faccessat | 相对目录文件描述符检查路径可访问性。 | 4 | int: 目录文件描述符 | const char __user *: 路径字符串 | int: 访问模式 | int: 控制标志 | - | - | 已实现 | 当前更接近“能否打开”的最小检查，不是完整权限判定。 |
| 270 | pselect6 | 等待多组文件描述符事件并临时切换信号掩码。 | 6 | int: 最大 fd 加 1 | fd_set __user *: 读集合 | fd_set __user *: 写集合 | fd_set __user *: 异常集合 | struct timespec __user *: 超时 | void __user *: 指向信号掩码与长度的结构 | 已实现 | 当前走简化轮询路径，信号掩码参数按最小需求处理。 |
| 271 | ppoll | 等待一组文件描述符事件并临时切换信号掩码。 | 5 | struct pollfd __user *: 待轮询数组 | nfds_t: 数组长度 | const struct timespec __user *: 超时 | const sigset_t __user *: 临时信号掩码 | size_t: 信号集字节数 | - | 已实现 | 当前走简化轮询路径，fd 位图仅覆盖有限范围。 |
| 272 | unshare | 执行 `unshare` 相关内核操作。 | 1 | unsigned long: 控制标志 | - | - | - | - | - | 未实现 | - |
| 273 | set_robust_list | 设置完成 `set_robust_list` 语义。 | 2 | struct robust_list_head __user *: 参数1 | size_t: 长度 | - | - | - | - | 未实现 | - |
| 274 | get_robust_list | 读取完成 `get_robust_list` 语义。 | 3 | int: 进程 ID | struct robust_list_head __user *__user *: 参数2 | size_t __user *: 大小或数量 | - | - | - | 未实现 | - |
| 275 | splice | 执行 `splice` 相关内核操作。 | 6 | int: 参数1 | loff_t __user *: 参数2 | int: 参数3 | loff_t __user *: 参数4 | size_t: 长度 | unsigned int: 控制标志 | 未实现 | - |
| 276 | tee | 执行 `tee` 相关内核操作。 | 4 | int: 参数1 | int: 参数2 | size_t: 长度 | unsigned int: 控制标志 | - | - | 未实现 | - |
| 277 | sync_file_range | 执行 `sync_file_range` 相关内核操作。 | 4 | int: 文件描述符 | loff_t: 偏移量 | loff_t: 字节数 | unsigned int: 控制标志 | - | - | 未实现 | - |
| 278 | vmsplice | 执行 `vmsplice` 相关内核操作。 | 4 | int: 文件描述符 | const struct iovec __user *: iovec 数组 | unsigned long: 参数3 | unsigned int: 控制标志 | - | - | 未实现 | - |
| 279 | move_pages | 执行 `move_pages` 相关内核操作。 | 6 | pid_t: 进程 ID | unsigned long: 参数2 | const void __user *__user *: 参数3 | const int __user *: 参数4 | int __user *: 参数5 | int: 控制标志 | 未实现 | - |
| 280 | utimensat | 按纳秒精度更新时间戳。 | 4 | int: 目录文件描述符 | const char __user *: 路径字符串 | const struct timespec __user *: 新的时间戳数组 | int: 控制标志 | - | - | 已实现 | - |
| 281 | epoll_pwait | 操作 epoll 并完成 `epoll_pwait` 语义。 | 6 | int: epoll 文件描述符 | struct epoll_event __user *: 参数2 | int: 参数3 | int: 超时 | const sigset_t __user *: 参数5 | size_t: 信号集字节数 | 未实现 | - |
| 282 | signalfd | 执行 `signalfd` 相关内核操作。 | 3 | int: 文件描述符 | sigset_t __user *: 参数2 | size_t: 大小或数量 | - | - | - | 未实现 | - |
| 283 | timerfd_create | 操作 timerfd 并完成 `timerfd_create` 语义。 | 2 | int: 时钟编号 | int: 控制标志 | - | - | - | - | 未实现 | - |
| 284 | eventfd | 执行 `eventfd` 相关内核操作。 | 1 | unsigned int: 字节数 | - | - | - | - | - | 未实现 | - |
| 285 | fallocate | 执行 `fallocate` 相关内核操作。 | 4 | int: 文件描述符 | int: 权限或模式标志 | loff_t: 偏移量 | loff_t: 长度 | - | - | 未实现 | - |
| 286 | timerfd_settime | 操作 timerfd 并完成 `timerfd_settime` 语义。 | 4 | int: 文件描述符 | int: 控制标志 | const struct __kernel_itimerspec __user *: 参数3 | struct __kernel_itimerspec __user *: 参数4 | - | - | 未实现 | - |
| 287 | timerfd_gettime | 操作 timerfd 并完成 `timerfd_gettime` 语义。 | 2 | int: 文件描述符 | struct __kernel_itimerspec __user *: 参数2 | - | - | - | - | 未实现 | - |
| 288 | accept4 | 接受一个入站连接并附带标志。 | 4 | int: 监听套接字 | struct sockaddr __user *: 输出对端地址 | int __user *: 输入输出地址长度 | int: accept 标志 | - | - | 未实现 | - |
| 289 | signalfd4 | 执行 `signalfd4` 相关内核操作。 | 4 | int: 文件描述符 | sigset_t __user *: 参数2 | size_t: 大小或数量 | int: 控制标志 | - | - | 未实现 | - |
| 290 | eventfd2 | 创建事件计数文件描述符。 | 2 | unsigned int: 初始计数值 | int: eventfd 标志 | - | - | - | - | 已实现 | - |
| 291 | epoll_create1 | 创建 epoll 实例。 | 1 | int: epoll 标志 | - | - | - | - | - | 已实现 | - |
| 292 | dup3 | 把文件描述符复制到指定编号并附带标志。 | 3 | int: 旧文件描述符 | int: 新文件描述符 | int: 复制标志 | - | - | - | 已实现 | 当前实际只复用 `dup2` 语义，`flags` 语义未完整展开。 |
| 293 | pipe2 | 创建管道并设置标志。 | 2 | int __user *: 长度为 2 的文件描述符数组 | int: 管道标志 | - | - | - | - | 已实现 | - |
| 294 | inotify_init1 | 操作 inotify 并完成 `inotify_init1` 语义。 | 1 | int: 控制标志 | - | - | - | - | - | 未实现 | - |
| 295 | preadv | 执行 `preadv` 相关内核操作。 | 5 | unsigned long: 文件描述符 | const struct iovec __user *: 向量数组 | unsigned long: 数组长度 | unsigned long: 参数4 | unsigned long: 参数5 | - | 未实现 | - |
| 296 | pwritev | 执行 `pwritev` 相关内核操作。 | 5 | unsigned long: 文件描述符 | const struct iovec __user *: 向量数组 | unsigned long: 数组长度 | unsigned long: 参数4 | unsigned long: 参数5 | - | 未实现 | - |
| 297 | rt_tgsigqueueinfo | 执行 `rt_tgsigqueueinfo` 相关内核操作。 | 4 | pid_t: 线程组 ID | pid_t: 进程 ID | int: 信号编号 | siginfo_t __user *: 参数4 | - | - | 未实现 | - |
| 298 | perf_event_open | 操作性能事件并完成 `perf_event_open` 语义。 | 5 | struct perf_event_attr __user *: 参数1 | pid_t: 进程 ID | int: 参数3 | int: 文件描述符 | unsigned long: 控制标志 | - | 未实现 | - |
| 299 | recvmmsg | 执行 `recvmmsg` 相关内核操作。 | 5 | int: 文件描述符 | struct mmsghdr __user *: 消息缓冲区 | unsigned int: 数组长度 | unsigned: 控制标志 | struct timespec __user *: 超时 | - | 未实现 | - |
| 300 | fanotify_init | 操作 fanotify 并完成 `fanotify_init` 语义。 | 2 | unsigned int: 控制标志 | unsigned int: 控制标志 | - | - | - | - | 未实现 | - |
| 301 | fanotify_mark | 操作 fanotify 并完成 `fanotify_mark` 语义。 | 5 | int: 文件描述符 | unsigned int: 控制标志 | u64: 掩码 | int: 文件描述符 | const char __user *: 路径字符串 | - | 未实现 | - |
| 302 | prlimit64 | 读取或设置资源限制。 | 4 | pid_t: 目标进程 ID | unsigned int: 资源类型 | const struct rlimit64 __user *: 新的限制 | struct rlimit64 __user *: 旧的限制 | - | - | 已实现 | - |
| 303 | name_to_handle_at | 执行 `name_to_handle_at` 相关内核操作。 | 5 | int: 目录文件描述符 | const char __user *: 名称字符串 | struct file_handle __user *: 参数3 | void __user *: 参数4 | int: 控制标志 | - | 未实现 | - |
| 304 | open_by_handle_at | 执行 `open_by_handle_at` 相关内核操作。 | 3 | int: 文件描述符 | struct file_handle __user *: 参数2 | int: 控制标志 | - | - | - | 未实现 | - |
| 305 | clock_adjtime | 操作时钟并完成 `clock_adjtime` 语义。 | 2 | clockid_t: 参数1 | struct __kernel_timex __user *: 参数2 | - | - | - | - | 未实现 | - |
| 306 | syncfs | 执行 `syncfs` 相关内核操作。 | 1 | int: 文件描述符 | - | - | - | - | - | 未实现 | - |
| 307 | sendmmsg | 执行 `sendmmsg` 相关内核操作。 | 4 | int: 文件描述符 | struct mmsghdr __user *: 消息缓冲区 | unsigned int: 数组长度 | unsigned: 控制标志 | - | - | 未实现 | - |
| 308 | setns | 设置完成 `setns` 语义。 | 2 | int: 文件描述符 | int: 参数2 | - | - | - | - | 未实现 | - |
| 309 | getcpu | 读取完成 `getcpu` 语义。 | 3 | unsigned __user *: 参数1 | unsigned __user *: 参数2 | void __user *: 参数3 | - | - | - | 未实现 | - |
| 310 | process_vm_readv | 操作目标进程并完成 `process_vm_readv` 语义。 | 6 | pid_t: 进程 ID | const struct iovec __user *: 参数2 | unsigned long: 参数3 | const struct iovec __user *: 参数4 | unsigned long: 参数5 | unsigned long: 控制标志 | 未实现 | - |
| 311 | process_vm_writev | 操作目标进程并完成 `process_vm_writev` 语义。 | 6 | pid_t: 进程 ID | const struct iovec __user *: 参数2 | unsigned long: 参数3 | const struct iovec __user *: 参数4 | unsigned long: 参数5 | unsigned long: 控制标志 | 未实现 | - |
| 312 | kcmp | 执行 `kcmp` 相关内核操作。 | 5 | pid_t: 参数1 | pid_t: 参数2 | int: 类型编号 | unsigned long: 参数4 | unsigned long: 参数5 | - | 未实现 | - |
| 313 | finit_module | 执行 `finit_module` 相关内核操作。 | 3 | int: 文件描述符 | const char __user *: 参数2 | int: 控制标志 | - | - | - | 未实现 | - |
| 314 | sched_setattr | 操作调度器并完成 `sched_setattr` 语义。 | 3 | pid_t: 进程 ID | struct sched_attr __user *: 参数2 | unsigned int: 控制标志 | - | - | - | 未实现 | - |
| 315 | sched_getattr | 操作调度器并完成 `sched_getattr` 语义。 | 4 | pid_t: 进程 ID | struct sched_attr __user *: 参数2 | unsigned int: 大小 | unsigned int: 控制标志 | - | - | 未实现 | - |
| 316 | renameat2 | 相对目录文件描述符重命名路径并附带标志。 | 5 | int: 旧目录文件描述符 | const char __user *: 旧路径 | int: 新目录文件描述符 | const char __user *: 新路径 | unsigned int: 重命名标志 | - | 已实现 | - |
| 317 | seccomp | 执行 `seccomp` 相关内核操作。 | 3 | unsigned int: 操作码 | unsigned int: 控制标志 | void __user *: 参数3 | - | - | - | 未实现 | - |
| 318 | getrandom | 从内核随机源获取字节流。 | 3 | char __user *: 输出缓冲区 | size_t: 请求字节数 | unsigned int: 随机标志 | - | - | - | 已实现 | 当前使用伪随机后端，语义为最小兼容实现。 |
| 319 | memfd_create | 操作匿名内存文件并完成 `memfd_create` 语义。 | 2 | const char __user *: 路径或名称字符串 | unsigned int: 控制标志 | - | - | - | - | 未实现 | - |
| 320 | kexec_file_load | 执行 `kexec_file_load` 相关内核操作。 | 5 | int: 文件描述符 | int: 文件描述符 | unsigned long: 大小或数量 | const char __user *: 参数4 | unsigned long: 控制标志 | - | 未实现 | - |
| 321 | bpf | 操作 eBPF 并完成 `bpf` 语义。 | 3 | int: 命令号 | union bpf_attr __user *: 参数2 | unsigned int: 大小 | - | - | - | 未实现 | - |
| 322 | execveat | 相对目录文件描述符执行新的用户程序映像。 | 5 | int: 目录文件描述符 | const char __user *: 路径字符串 | const char __user *const __user *: 参数向量 | const char __user *const __user *: 环境向量 | int: 控制标志 | - | 未实现 | - |
| 323 | userfaultfd | 执行 `userfaultfd` 相关内核操作。 | 1 | int: 控制标志 | - | - | - | - | - | 未实现 | - |
| 324 | membarrier | 执行 `membarrier` 相关内核操作。 | 3 | int: 命令号 | unsigned int: 控制标志 | int: 参数3 | - | - | - | 未实现 | - |
| 325 | mlock2 | 执行 `mlock2` 相关内核操作。 | 3 | unsigned long: 起始地址 | size_t: 长度 | int: 控制标志 | - | - | - | 未实现 | - |
| 326 | copy_file_range | 执行 `copy_file_range` 相关内核操作。 | 6 | int: 参数1 | loff_t __user *: 参数2 | int: 参数3 | loff_t __user *: 参数4 | size_t: 长度 | unsigned int: 控制标志 | 未实现 | - |
| 327 | preadv2 | 执行 `preadv2` 相关内核操作。 | 6 | unsigned long: 文件描述符 | const struct iovec __user *: 向量数组 | unsigned long: 数组长度 | unsigned long: 参数4 | unsigned long: 参数5 | rwf_t: 控制标志 | 未实现 | - |
| 328 | pwritev2 | 执行 `pwritev2` 相关内核操作。 | 6 | unsigned long: 文件描述符 | const struct iovec __user *: 向量数组 | unsigned long: 数组长度 | unsigned long: 参数4 | unsigned long: 参数5 | rwf_t: 控制标志 | 未实现 | - |
| 329 | pkey_mprotect | 执行 `pkey_mprotect` 相关内核操作。 | 4 | unsigned long: 起始地址 | size_t: 长度 | unsigned long: 保护标志 | int: 参数4 | - | - | 未实现 | - |
| 330 | pkey_alloc | 执行 `pkey_alloc` 相关内核操作。 | 2 | unsigned long: 控制标志 | unsigned long: 参数2 | - | - | - | - | 未实现 | - |
| 331 | pkey_free | 执行 `pkey_free` 相关内核操作。 | 1 | int: 参数1 | - | - | - | - | - | 未实现 | - |
| 332 | statx | 以扩展格式读取文件状态。 | 5 | int: 目录文件描述符 | const char __user *: 路径字符串 | unsigned int: 查找标志 | unsigned int: 请求字段掩码 | struct statx __user *: 输出扩展状态 | - | 未实现 | - |
| 333 | io_pgetevents | 操作异步 I/O 并完成 `io_pgetevents` 语义。 | 6 | aio_context_t: 参数1 | long: 参数2 | long: 编号 | struct io_event __user *: 参数4 | struct timespec __user *: 超时 | const struct __aio_sigset __user *: 信号编号 | 未实现 | - |
| 334 | rseq | 执行 `rseq` 相关内核操作。 | 4 | struct rseq __user *: 参数1 | uint32_t: 大小或数量 | int: 控制标志 | uint32_t: 信号编号 | - | - | 未实现 | - |
| 335 | uretprobe | 注册用户态返回探针。 | 0 | - | - | - | - | - | - | 未实现 | - |
| 336 | uprobe | 注册用户态探针。 | 0 | - | - | - | - | - | - | 未实现 | - |
| 424 | pidfd_send_signal | 通过 pidfd 向目标进程发送信号。 | 4 | int: 进程文件描述符 | int: 信号编号 | siginfo_t __user *: 参数3 | unsigned int: 控制标志 | - | - | 未实现 | - |
| 425 | io_uring_setup | 创建 io_uring 实例。 | 2 | u32: 参数1 | struct io_uring_params __user *: 参数2 | - | - | - | - | 未实现 | - |
| 426 | io_uring_enter | 向 io_uring 提交或等待请求。 | 6 | unsigned int: 文件描述符 | u32: 参数2 | u32: 参数3 | u32: 控制标志 | const void __user *: 参数5 | size_t: 参数6 | 未实现 | - |
| 427 | io_uring_register | 为 io_uring 注册辅助资源。 | 4 | unsigned int: 文件描述符 | unsigned int: 操作码 | void __user *: 附加参数 | unsigned int: 参数4 | - | - | 未实现 | - |
| 428 | open_tree | 执行 `open_tree` 相关内核操作。 | 3 | int: 目录文件描述符 | const char __user *: 路径字符串 | unsigned: 控制标志 | - | - | - | 未实现 | - |
| 429 | move_mount | 执行 `move_mount` 相关内核操作。 | 5 | int: 文件描述符 | const char __user *: 路径或名称字符串 | int: 文件描述符 | const char __user *: 路径或名称字符串 | unsigned int: 控制标志 | - | 未实现 | - |
| 430 | fsopen | 执行 `fsopen` 相关内核操作。 | 2 | const char __user *: 路径或名称字符串 | unsigned int: 控制标志 | - | - | - | - | 未实现 | - |
| 431 | fsconfig | 执行 `fsconfig` 相关内核操作。 | 5 | int: 文件描述符 | unsigned int: 命令号 | const char __user *: 参数3 | const void __user *: 数值 | int: 参数5 | - | 未实现 | - |
| 432 | fsmount | 执行 `fsmount` 相关内核操作。 | 3 | int: 文件描述符 | unsigned int: 控制标志 | unsigned int: 控制标志 | - | - | - | 未实现 | - |
| 433 | fspick | 执行 `fspick` 相关内核操作。 | 3 | int: 目录文件描述符 | const char __user *: 路径字符串 | unsigned int: 控制标志 | - | - | - | 未实现 | - |
| 434 | pidfd_open | 为进程创建 pidfd。 | 2 | pid_t: 进程 ID | unsigned int: 控制标志 | - | - | - | - | 未实现 | - |
| 435 | clone3 | 按 `struct clone_args` 创建新任务。 | 2 | struct clone_args __user *: clone 参数结构 | size_t: 结构大小 | - | - | - | - | 未实现 | 当前直接返回 `ENOSYS`。 |
| 436 | close_range | 批量关闭一段文件描述符范围。 | 3 | unsigned int: 文件描述符 | unsigned int: 文件描述符 | unsigned int: 控制标志 | - | - | - | 未实现 | - |
| 437 | openat2 | 以 `struct open_how` 描述打开文件。 | 4 | int: 目录文件描述符 | const char __user *: 路径字符串 | const struct open_how __user *: 打开参数结构 | size_t: 结构大小 | - | - | 未实现 | - |
| 438 | pidfd_getfd | 通过 pidfd 复制目标进程的文件描述符。 | 3 | int: 进程文件描述符 | int: 文件描述符 | unsigned int: 控制标志 | - | - | - | 未实现 | - |
| 439 | faccessat2 | 按扩展标志相对目录文件描述符检查路径可访问性。 | 4 | int: 目录文件描述符 | const char __user *: 路径字符串 | int: 权限或模式标志 | int: 控制标志 | - | - | 未实现 | - |
| 440 | process_madvise | 操作目标进程并完成 `process_madvise` 语义。 | 5 | int: 进程文件描述符 | const struct iovec __user *: 向量数组 | size_t: 数组长度 | int: 参数4 | unsigned int: 控制标志 | - | 未实现 | - |
| 441 | epoll_pwait2 | 操作 epoll 并完成 `epoll_pwait2` 语义。 | 6 | int: epoll 文件描述符 | struct epoll_event __user *: 参数2 | int: 参数3 | const struct timespec __user *: 超时 | const sigset_t __user *: 参数5 | size_t: 信号集字节数 | 未实现 | - |
| 442 | mount_setattr | 批量修改挂载点属性。 | 5 | int: 目录文件描述符 | const char __user *: 路径字符串 | unsigned int: 控制标志 | struct mount_attr __user *: 参数4 | size_t: 大小或数量 | - | 未实现 | - |
| 443 | quotactl_fd | 通过文件描述符控制磁盘配额。 | 4 | unsigned int: 文件描述符 | unsigned int: 命令号 | qid_t: 参数3 | void __user *: 地址 | - | - | 未实现 | - |
| 444 | landlock_create_ruleset | 创建 Landlock 规则集。 | 3 | const struct landlock_ruleset_attr __user *: 参数1 | size_t: 大小 | __u32: 控制标志 | - | - | - | 未实现 | - |
| 445 | landlock_add_rule | 向 Landlock 规则集添加规则。 | 4 | int: 文件描述符 | enum landlock_rule_type: 参数2 | const void __user *: 参数3 | __u32: 控制标志 | - | - | 未实现 | - |
| 446 | landlock_restrict_self | 对当前任务启用 Landlock 限制。 | 2 | int: 文件描述符 | __u32: 控制标志 | - | - | - | - | 未实现 | - |
| 447 | memfd_secret | 创建 secretmem 保护的匿名内存文件。 | 1 | unsigned int: 控制标志 | - | - | - | - | - | 未实现 | - |
| 448 | process_mrelease | 释放退出进程残留的内存资源。 | 2 | int: 进程文件描述符 | unsigned int: 控制标志 | - | - | - | - | 未实现 | - |
| 449 | futex_waitv | 一次等待多个 futex。 | 5 | struct futex_waitv __user *: 参数1 | unsigned int: 参数2 | unsigned int: 控制标志 | struct timespec __user *: 超时 | clockid_t: 时钟编号 | - | 未实现 | - |
| 450 | set_mempolicy_home_node | 设置内存策略的 home node。 | 4 | unsigned long: 起始地址 | unsigned long: 长度 | unsigned long: 参数3 | unsigned long: 控制标志 | - | - | 未实现 | - |
| 451 | cachestat | 查询文件页缓存统计信息。 | 4 | unsigned int: 文件描述符 | struct cachestat_range __user *: 参数2 | struct cachestat __user *: 参数3 | unsigned int: 控制标志 | - | - | 未实现 | - |
| 452 | fchmodat2 | 按扩展参数相对目录文件描述符修改权限位。 | 4 | int: 目录文件描述符 | const char __user *: 路径字符串 | mode_t: 权限位或模式 | unsigned int: 控制标志 | - | - | 未实现 | - |
| 453 | map_shadow_stack | 创建或映射 shadow stack。 | 3 | unsigned long: 地址 | unsigned long: 大小 | unsigned int: 控制标志 | - | - | - | 未实现 | - |
| 454 | futex_wake | 执行独立的 futex 唤醒操作。 | 4 | void __user *: 用户态地址 | unsigned long: 掩码 | int: 编号 | unsigned int: 控制标志 | - | - | 未实现 | - |
| 455 | futex_wait | 执行独立的 futex 等待操作。 | 6 | void __user *: 用户态地址 | unsigned long: 数值参数 | unsigned long: 掩码 | unsigned int: 控制标志 | struct timespec __user *: 参数5 | clockid_t: 时钟编号 | 未实现 | - |
| 456 | futex_requeue | 执行独立的 futex 重排队操作。 | 4 | struct futex_waitv __user *: 参数1 | unsigned int: 控制标志 | int: 参数3 | int: 参数4 | - | - | 未实现 | - |
| 457 | statmount | 查询挂载点状态信息。 | 4 | const struct mnt_id_req __user *: 请求结构 | struct statmount __user *: 输出缓冲区 | size_t: 大小或数量 | unsigned int: 控制标志 | - | - | 未实现 | - |
| 458 | listmount | 列出挂载点信息。 | 4 | const struct mnt_id_req __user *: 请求结构 | u64 __user *: 参数2 | size_t: 参数3 | unsigned int: 控制标志 | - | - | 未实现 | - |
| 459 | lsm_get_self_attr | 读取当前任务的 LSM 属性。 | 4 | unsigned int: 参数1 | struct lsm_ctx __user *: 参数2 | u32 __user *: 大小 | u32: 控制标志 | - | - | 未实现 | - |
| 460 | lsm_set_self_attr | 设置当前任务的 LSM 属性。 | 4 | unsigned int: 参数1 | struct lsm_ctx __user *: 参数2 | u32: 大小 | u32: 控制标志 | - | - | 未实现 | - |
| 461 | lsm_list_modules | 列出当前启用的 LSM 模块。 | 3 | u64 __user *: 参数1 | u32 __user *: 大小 | u32: 控制标志 | - | - | - | 未实现 | - |
| 462 | mseal | 给匿名内存施加 seal 限制。 | 3 | unsigned long: 起始地址 | size_t: 长度 | unsigned long: 控制标志 | - | - | - | 未实现 | - |
| 463 | setxattrat | 相对目录文件描述符设置扩展属性。 | 6 | int: 目录文件描述符 | const char __user *: 路径字符串 | unsigned int: 控制标志 | const char __user *: 名称字符串 | const struct xattr_args __user *: 参数5 | size_t: 大小 | 未实现 | - |
| 464 | getxattrat | 相对目录文件描述符读取扩展属性。 | 6 | int: 目录文件描述符 | const char __user *: 路径字符串 | unsigned int: 控制标志 | const char __user *: 名称字符串 | struct xattr_args __user *: 参数5 | size_t: 大小 | 未实现 | - |
| 465 | listxattrat | 相对目录文件描述符列出扩展属性。 | 5 | int: 目录文件描述符 | const char __user *: 路径字符串 | unsigned int: 控制标志 | char __user *: 参数4 | size_t: 大小 | - | 未实现 | - |
| 466 | removexattrat | 相对目录文件描述符删除扩展属性。 | 4 | int: 目录文件描述符 | const char __user *: 路径字符串 | unsigned int: 控制标志 | const char __user *: 名称字符串 | - | - | 未实现 | - |
| 467 | open_tree_attr | 按扩展属性复制或打开挂载树。 | 5 | int: 目录文件描述符 | const char __user *: 路径字符串 | unsigned: 控制标志 | struct mount_attr __user *: 参数4 | size_t: 大小或数量 | - | 未实现 | - |
| 468 | file_getattr | 读取文件属性信息。 | 5 | int: 目录文件描述符 | const char __user *: 路径字符串 | struct file_attr __user *: 参数3 | size_t: 大小或数量 | unsigned int: 控制标志 | - | 未实现 | - |
| 469 | file_setattr | 设置文件属性信息。 | 5 | int: 目录文件描述符 | const char __user *: 路径字符串 | struct file_attr __user *: 参数3 | size_t: 大小或数量 | unsigned int: 控制标志 | - | 未实现 | - |
| 470 | listns | 列出命名空间信息。 | 4 | const struct ns_id_req __user *: 请求结构 | u64 __user *: 参数2 | size_t: 参数3 | unsigned int: 控制标志 | - | - | 未实现 | - |
| 471 | rseq_slice_yield | 执行 `rseq_slice_yield` 相关内核操作。 | 0 | - | - | - | - | - | - | 未实现 | - |
