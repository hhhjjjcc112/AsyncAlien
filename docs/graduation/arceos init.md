# x86_64初始化：
BSP: BIOS/GRUB(multiboot1) -> _start -> rust_entry -> axplat::call_main -> rust_main

_start:
- GRUB按multiboot1协议加载内核后进入_start，eax传入magic，ebx传入multiboot info地址
- 先进入32位入口bsp_entry32，加载临时GDT并设置段寄存器
- 打开CR4里的PAE/PGE，加载临时页表，设置EFER(LME/NXE)，再打开CR0分页，完成进入长模式前置条件
- 通过长跳转进入bsp_entry64
- 在64位入口清理段寄存器，设置启动栈，调用rust_entry(magic, mbi)

rust_entry:
- 校验multiboot magic是否正确
- 读取当前CPU的local APIC ID作为cpu_id
- 调用axplat::call_main(cpu_id, mbi)进入rust_main

rust_main:
- 清零.bss段
- 初始化percpu数据
- 存储multiboot指针 + init early
  - 初始化Trap子系统：GDT、IDT，若启用用户态，再初始化syscall MSR（LSTAR/STAR/SFMASK/KERNEL_GS_BASE）
  - 初始化串口控制台
  - 初始化时间：TSC频率、初始tick，若启用RTC则计算TSC与RTC时间的偏移，用于计算绝对时间
  - 解析multiboot内存信息，提取可用RAM区域
- 初始化日志系统
- 初始化物理内存，区分内核镜像各段、boot stack、.bss、MMIO、保留内存和空闲内存
  - x86平台额外保留低1MiB物理内存
  - RAM范围来自multiboot内存图里的Available区间
- 初始化全局物理内存分配器，使用上一步得到的空闲内存区域
- 建立内核虚拟地址空间，切换到正式页表
- init later + 初始化CPU核数
  - 初始化local APIC和IO APIC，屏蔽8259A
  - 根据CPU能力选择x2APIC或xAPIC模式
  - 初始化APIC定时器（one-shot）
  - CPU数量取平台声明值，并受MAX_CPU_NUM限制
- 初始化任务调度器
- 发现外部设备，分为NetDevice, BlockDevice, DisplayDevice
  - 初始化文件系统：BlockDevice
  - 初始化网络协议栈：NetDevice
  - 初始化图形界面：DisplayDevice
- 启动从核
  - 主核通过APIC发送INIT-SIPI-SIPI序列唤醒从核
  - 从核先进入ap_entry32/ap_entry64，再调用rust_entry_secondary
  - rust_entry_secondary调用axplat::call_secondary_main，进入rust_main_secondary
  - 从核继续完成percpu、early/later secondary、内存管理、调度器和IPI相关初始化
- 设置irq相关处理函数（时钟，IPI）
  - 时钟中断走local APIC timer向量
  - IPI通过APIC发送到指定核或广播
  - 外部设备中断经IO APIC路由到中断向量
- 初始化线程存储tls
- 调用.init.array构造函数（ctor）
- 进入用户程序main函数

AP: APIC INIT-SIPI-SIPI -> ap_start(16位实模式) -> ap_start32 -> ap_entry32 -> ap_entry64 -> secondary_entry -> rust_entry_secondary -> axplat::call_secondary_main -> rust_main_secondary

ap_start:
- AP被SIPI唤醒后以16位实模式进入，清零段寄存器并加载临时GDT
- 设置CR0.PE进入保护模式，远跳转到ap_start32

ap_start32:
- 从启动页末尾读取BSP写入的栈顶地址和入口地址
- 跳转到ap_entry32

ap_entry32/ap_entry64:
- 复用BSP的32位到64位切换流程（PAE/PGE、临时页表、EFER、CR0分页）
- 在64位入口设置高地址栈并进入secondary_entry

secondary_entry:
- 初始化本核Local APIC与APIC Timer
- 调用rust_entry_secondary，再进入axplat::call_secondary_main

rust_main_secondary:
- init_percpu_secondary(cpu_id)
- init_early_secondary(cpu_id)
- ENTERED_CPUS+1，通知BSP继续拉起下一个AP
- init_memory_management_secondary()
- init_later_secondary(cpu_id)
- init_scheduler_secondary()
- axipi::init()
- INITED_CPUS+1，等待所有CPU完成初始化
- enable_irqs()，进入run_idle

# riscv64初始化：
BSP: QEMU virt/OpenSBI -> _start -> rust_entry -> axplat::call_main -> rust_main

_start:
- OpenSBI跳入内核入口，a0传入hartid，a1传入dtb地址
- 保存hartid和dtb指针，建立启动栈BOOT_STACK
- 构造临时Sv39启动页表，同时映射低地址和内核高半区地址
- 写入启动页表并刷新TLB，提前启用MMU
- 用PHYS_VIRT_OFFSET将栈和入口地址修正到高半区
- 跳转到rust_entry(cpu_id, dtb)

rust_entry:
- 读取hartid作为cpu_id
- 调用axplat::call_main(cpu_id, dtb)进入rust_main

rust_main:
- 清零.bss段
- 初始化percpu数据
- 存储dtb指针 + init early
  - 保存boot arg，后续可通过axhal::get_bootarg()获取dtb地址
  - 初始化Trap入口
  - 初始化早期时间子系统；若启用RTC，则读取goldfish rtc并计算墙上时间偏移
- 初始化日志系统
- 初始化物理内存，区分内核镜像各段、boot stack、.bss、MMIO区域和空闲内存
  - RAM/MMIO范围主要来自平台配置
  - 当前riscv64-qemu-virt实现通常按配置值切分可用内存
- 初始化全局物理内存分配器，使用上一步得到的空闲内存区域
- 建立内核虚拟地址空间，切换到正式页表
- init later + 初始化CPU核数
  - 打开S态软中断、时钟中断、外部中断使能位
  - 初始化每核定时器；若启用irq，则通过SBI设置定时器初值
  - CPU数量取平台声明值，并受MAX_CPU_NUM限制
- 初始化任务调度器
- 发现外部设备，分为NetDevice, BlockDevice, DisplayDevice
  - 初始化文件系统：BlockDevice
  - 初始化网络协议栈：NetDevice
  - 初始化图形界面：DisplayDevice
- 启动从核
  - 主核通过SBI HSM扩展调用hart_start
  - 从核从_start_secondary进入，调用axplat::call_secondary_main
  - 进入rust_main_secondary完成从核初始化
- 设置irq相关处理函数（时钟，IPI）
  - 时钟中断来自S态timer interrupt，通过SBI设置下一次deadline
  - IPI使用S态software interrupt
  - 外部中断走PLIC
- 初始化线程存储tls
- 调用.init.array构造函数（ctor）
- 进入用户程序main函数

AP: SBI HSM hart_start -> _start_secondary -> rust_entry_secondary -> axplat::call_secondary_main -> rust_main_secondary

_start_secondary:
- 保存hartid到tp，按hart号切分启动栈
- 启用启动页表并切换到高半区
- 跳转到rust_entry_secondary(cpu_id)

rust_entry_secondary:
- 调用axplat::call_secondary_main(cpu_id)

rust_main_secondary:
- init_percpu_secondary(cpu_id)
- init_early_secondary(cpu_id)
- ENTERED_CPUS+1，通知BSP继续启动下一个hart
- init_memory_management_secondary()
- init_later_secondary(cpu_id)，初始化本核定时器
- init_scheduler_secondary()
- IPI相关初始化（S态软中断）
- INITED_CPUS+1，等待所有核就绪
- enable_irqs()，进入调度
