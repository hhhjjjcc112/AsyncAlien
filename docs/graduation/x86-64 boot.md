[axplat-x86-pc]()

# multiboot协议
[multiboot manual](https://www.gnu.org/software/grub/manual/multiboot/multiboot.html)

- 需要在内核镜像中包含multiboot头部, bootloader会根据multiboot头部的信息加载内核
- multiboot header包含了内核入口，以及段布局等信息，bootloader根据这些信息将内核加载到指定的物理内存上
- bootloader执行完成后，会传递固定魔数和multiboot信息结构体的地址给内核入口函数, 此时处于32位保护模式

# 主核进入长模式

- 为进入长模式，主核需要设置分页和启用长模式
- 加载一个简单的gdt，包含32位和64位代码段描述符和32位数据段描述符
- 设置一个简单的页表，将0x0-0x7fffff和0xffff800000000000-0xffff800007ffffff映射为相同的物理地址, 内核使用其中的高地址
- 启用分页和长模式，跳转到64位代码段，即rust入口函数，传入multiboot信息结构体的地址

# 多核启动

- 主核通过ipi唤醒其他核，顺序为init ipi -> sleep 10ms -> startup ipi -> sleep 200us -> startup ipi
- 从核启动后为16位实模式，加载简单gdt后跳转到32位保护模式，然后剩余步骤与主核相同
- 进入长模式后跳转到另外的rust入口函数，因为一些初始化工作只需要一次，应当由主核完成。

# APIC初始化

- 使用x2apic模式，如果cpu支持x2apic则启用，否则使用xapic模式
- 主核初始化local apic，并启用apic, 通过apic发送ipi唤醒从核
- 从核进入长模式后也初始化local apic，并启用apic

# 定时器初始化

- 使用local apic的定时器和tsc寄存器管理时间
