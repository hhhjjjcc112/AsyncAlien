# Alien riscv64初始化（原有记录，保留）：
BSP: OpenSBI -> _start -> main

_start:
- OpenSBI跳入内核入口，a0传入hart_id，a1传入boot_info_ptr（DTB地址）
- 保存a0/a1到tp/gp，按hart号从全局栈数组切分本核启动栈
- 不再进入独立platform_init阶段，完成最小上下文后直接进入main(hart_id)

main:
- 通过STARTED原子变量区分BSP路径和AP路径
- BSP路径（先完成关键初始化，再唤醒从核）：
  - 保存boot_info并读取platform_machine_info()（内存范围、CPU数量、基础设备信息）
  - mem::init_memory_system(machine_info.memory.end, true)
  - arch::allow_access_user_memory()
  - bus::init_with_boot_info()（riscv64分支走init_with_dtb，探测RTC/UART/PLIC/PCI/VirtIO）
  - trap::init_trap_subsystem()（设置stvec/sscratch，打开外部中断、时钟中断、全局中断）
  - domain::load_domains()
  - 启动从核：通过SBI HSM hart_start唤醒其他hart，入口为_start_secondary
  - 释放STARTED，放行AP继续执行
- AP路径：
  - 等待BSP完成关键初始化并放行
  - mem::init_memory_system(0, false)
  - arch::allow_access_user_memory()
  - trap::init_trap_subsystem()

- 设置下一次时钟中断：timer::set_next_trigger()，底层通过SBI timer设置deadline
- 进入task::run_task()开始调度

AP: SBI HSM hart_start -> _start_secondary -> main(AP路径)

_start_secondary:
- 与_start类似：保存tp/gp并切到本核启动栈
- 直接进入main(cpu_id)

main(AP路径):
- 自旋等待BSP放行
- 完成本核内存、trap和中断初始化
- 进入统一调度路径

# Alien riscv64初始化（按当前代码更新）：
BSP: OpenSBI -> _start -> main

_start:
- OpenSBI跳入内核入口，a0传入hart_id，a1传入boot_info_ptr（DTB地址）
- 保存a0/a1到tp/gp，按hart号从全局栈数组切分本核启动栈
- 完成最小上下文后调用main(cpu_id, info_ptr)

main:
- 清空.bss段
- 先执行platform_init_primary(boot_cpu_id, boot_info_ptr)
  - 保存boot_info并解析machine_info
  - 初始化日志（x86还会在这里初始化APIC/时钟）
- BSP主流程：
  - 读取platform_machine_info()
  - mem::init_memory_system(machine_info.memory.end, true)
  - arch::allow_access_user_memory()
  - bus::init_with_boot_info()（riscv64分支走init_with_dtb）
  - trap::init_trap_subsystem()
  - domain::load_domains()
  - start_other_cpu(boot_cpu_id)：BSP在主初始化完成后再通过SBI HSM唤醒从核
  - 等待所有从核完成secondary_main的初始化阶段（SECONDARY_INIT_COUNT）
  - SECONDARY_RUN_RELEASED置true，统一放行从核进入调度
  - timer::set_next_trigger()
  - task::run_task()

AP: SBI HSM hart_start -> _start_secondary -> secondary_main

_start_secondary:
- 与_start类似：保存tp/gp并切到本核启动栈
- 直接进入secondary_main(cpu_id)

secondary_main:
- mem::init_memory_system(0, false)
- arch::allow_access_user_memory()
- trap::init_trap_subsystem()
- SECONDARY_INIT_COUNT加1，通知BSP“本核初始化完成”
- 等待SECONDARY_RUN_RELEASED放行
- timer::set_next_trigger()
- task::run_task()

# Alien x86_64初始化（计划，按当前代码结构）：
BSP: GRUB(multiboot1) -> _start -> main_entry -> main

_start:
- GRUB加载后进入_start，eax传入magic，ebx传入multiboot info地址
- 进入bsp_entry32：加载临时GDT，设置段寄存器
- 打开CR4.PAE/PGE，加载临时页表，设置EFER(LME/NXE)，打开CR0分页
- 长跳转进入bsp_entry64，设置BOOT_STACK，调用main_entry(magic, mbi)

main_entry:
- 校验multiboot magic
- 获取当前CPU的local APIC ID作为cpu_id
- 调用main(cpu_id, mbi)

main:
- 清空.bss段
- trap::init_trap_subsystem()（init_idt）
- main内部先调用platform_init_primary(cpu_id, mbi)
  - 初始化主核APIC/IOAPIC（屏蔽8259A）
  - 初始化时间子系统（TSC/RTC）和主核APIC Timer
  - 保存mbi并解析machine_info
  - 初始化日志
- BSP主流程：
  - mem::init_memory_system(machine_info.memory.end, true)
  - bus::init_with_boot_info()（x86分支走init_with_acpi）
  - domain::load_domains()
  - start_other_cpu(boot_cpu_id)（实际底层仍走APIC INIT-SIPI-SIPI）
  - 等待所有从核完成secondary_main初始化，再统一放行
  - timer::set_next_trigger()（APIC Timer）
  - task::run_task()

AP: BSP唤醒 -> ap_start -> ap_start32 -> ap_entry32 -> ap_entry64 -> secondary_entry -> secondary_main

ap_start:
- AP被SIPI唤醒后在16位实模式执行，清段寄存器并加载临时GDT
- 设置寄存器并远跳转到ap_start32

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
- 完成本核内存与trap初始化后上报SECONDARY_INIT_COUNT
- 等待BSP统一放行后，再进入timer与task调度
