.section .text.trampoline
.code64

.global syscall_entry

# syscall 入口仅依赖这两个 percpu 符号：用户 rsp 暂存 + TSS.rsp0 读取。
.extern __PERCPU_USER_RSP
.extern __PERCPU_TSS
.extern x86_syscall_handler

# TrapFrame 槽位偏移（单位：字节）
.equ TF_VECTOR, 136
.equ TF_RSP, 176

# syscall handler 的地址也存储在 trampoline 中
syscall_handler_ptr:
    .quad x86_syscall_handler


.align 8
syscall_entry:
    # 硬件状态：RCX <- RIP、R11 <- RFLAGS

    swapgs

    # 入口早期：仅在 percpu 保存用户栈指针。
    mov qword ptr gs:[offset __PERCPU_USER_RSP], rsp

    # 从 per-cpu TSS 读取 rsp0（TrapContext 末尾）。
    mov rsp, qword ptr gs:[offset __PERCPU_TSS + {tss_rsp0_offset}]

    # 使用 push 直接构造 TrapContext。
    # 跳过不需要的槽位：ss/cs/error_code/vector。
    sub rsp, 8  # 跳过 ss
    push qword ptr gs:[offset __PERCPU_USER_RSP] # rsp
    push r11    # rflags
    sub rsp, 8  # 跳过 cs
    push rcx   # rip
    sub rsp, 2 * 8  # 跳过 error_code/vector

    push rax
    push rcx
    push rdx
    push rdi
    push rsi
    push r8
    push r9
    push r10
    push r11
    push rbx
    push rbp
    push r12
    push r13
    push r14
    push r15

    # 旧 TrapContext 基址在当前 rsp 的 -16 位置。
    mov r13, qword ptr [rsp - 16]
    mov r14, qword ptr [rsp - 8]

    # 切到内核 CR3/栈
    mov cr3, r13
    # 此处进入内核地址空间
    mov rsp, r14

    # 调用 syscall handler
    mov r15, [syscall_handler_ptr]
    call r15

    # handler 返回约定：rax=user_cr3, rdx=trap_cx_ptr
    
    mov cr3, rax
    # 切 rsp 回用户 TrapFrame
    mov rsp, rdx
    # 跳过前置内核字段区，按寄存器布局恢复。
    add rsp, 16

    pop r15
    pop r14
    pop r13
    pop r12
    pop rbp
    pop rbx
    pop r11                    # RFLAGS（for sysret）
    pop r10
    pop r9
    pop r8
    pop rsi
    pop rdi
    pop rdx
    pop rcx                    # RIP（for sysret）
    pop rax                    # syscall 返回值（handler 已写回）

    # 此时 rsp 指向 TrapContext 的 vector 槽
    add rsp, 7 * 8
    mov rcx, [rsp - 5 * 8]  // rip
    mov r11, [rsp - 3 * 8]  // rflags
    mov rsp, [rsp - 2 * 8]  // user rsp

    swapgs
    sysretq
