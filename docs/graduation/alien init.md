# Alien riscv64初始化：
BSP: OpenSBI -> _start -> platform_init -> main

_start:
- OpenSBI跳入内核入口，a0传入hart_id，a1传入boot_info_ptr（DTB地址）
- 保存a0/a1到tp/gp，按hart号从全局栈数组切分本核启动栈
- 完成最小上下文后调用platform_init(hart_id, boot_info)

platform_init:
- 清零.bss段
- 保存boot_info指针并解析machine_info（riscv64下基于DTB）
- 初始化日志
- BSP循环调用SBI HSM hart_start启动其他hart，入口为_start_secondary
- 调用main(hart_id)

main:
- 通过STARTED原子变量区分BSP路径和AP路径
- BSP路径：
  - 读取platform_machine_info()获取平台在早期保存的机器信息（内存范围、CPU数量、基础设备信息）。
  - mem::init_memory_system(machine_info.memory.end, true)初始化内存管理，建立后续分配和映射所需的基础状态。
  - arch::allow_access_user_memory()允许内核在需要时访问用户地址空间。
  - bus::init_with_boot_info()根据DTB探测并注册平台设备（如UART、PLIC、PCI、VirtIO等）。（riscv64分支走init_with_dtb，探测RTC/UART/PLIC/PCI/VirtIO）
  - trap::init_trap_subsystem()初始化异常/中断入口并打开中断使能。（设置stvec/sscratch，打开外部中断、时钟中断、全局中断）
  - domain::load_domains()加载并启动各个内核域与服务域。
  - 释放STARTED，放行次核
- AP路径：
  - 等待BSP完成关键初始化
  - mem::init_memory_system(0, false)
  - arch::allow_access_user_memory()
  - trap::init_trap_subsystem()

- 设置下一次时钟中断：timer::set_next_trigger()，底层通过SBI timer设置deadline
- 进入task::run_task()开始调度

AP: SBI HSM hart_start -> _start_secondary -> main(AP路径)

_start_secondary:
- 与_start类似：保存tp/gp并切到本核启动栈
- 进入main(cpu_id)

main(AP路径):
- 自旋等待BSP放行
- 完成本核内存、trap和中断初始化
- 进入统一调度路径

# Alien x86_64初始化（计划）：
BSP: GRUB(multiboot1) -> _start -> main_entry -> platform_init -> main

_start:
- GRUB加载后进入_start，eax传入magic，ebx传入multiboot info地址
- 进入bsp_entry32：加载临时GDT，设置段寄存器
- 打开CR4.PAE/PGE，加载临时页表，设置EFER(LME/NXE)，打开CR0分页
- 长跳转进入bsp_entry64，设置BOOT_STACK，调用main_entry(magic, mbi)

main_entry:
- 校验multiboot magic
- 获取当前CPU的local APIC ID作为cpu_id
- 调用platform_init(cpu_id, mbi)

platform_init:
- 清零.bss段
- 保存mbi指针并解析machine_info
- ACPI初始化并发现设备（MADT/IOAPIC/HPET等）
- 初始化主核Local APIC和IO APIC，屏蔽8259A，选择x2APIC/xAPIC模式
- 初始化时间子系统（TSC/RTC）和主核APIC Timer
- 初始化日志
- 调用main(cpu_id)

main:
- 通过STARTED区分BSP和AP路径
- BSP路径：
  - mem::init_memory_system(machine_info.memory.end, true)
  - bus::init_with_boot_info()（x86分支走init_with_acpi）
  - trap::init_trap_subsystem()（init_idt）
  - 启动从核：ap::start_aps()
  - domain::load_domains()
  - 释放STARTED
- AP路径：
  - 等待BSP放行
  - mem::init_memory_system(0, false)
  - trap::init_trap_subsystem()

- 设置下一次时钟中断（APIC Timer）
- 进入task::run_task()开始调度

AP: APIC INIT-SIPI-SIPI -> ap_start(16位) -> ap_start32 -> ap_entry32 -> ap_entry64 -> secondary_entry -> secondary_main -> main(AP路径)

ap_start:
- AP被SIPI唤醒后在16位实模式执行，清段寄存器并加载临时GDT
- 设置CR0.PE并远跳转到ap_start32

ap_start32:
- 从启动页尾部读取BSP写入的栈顶和入口地址
- 跳转到ap_entry32

ap_entry32/ap_entry64:
- 完成与BSP一致的32位到64位切换（PAE/PGE、临时页表、EFER、CR0分页）
- 进入secondary_entry

secondary_entry:
- init_secondary_apic()
- init_secondary_apic_timer()
- 调用secondary_main(cpu_id)

secondary_main:
- 进入main(cpu_id)的AP路径完成后续初始化
