.equ NUM_INT, 256

# TrapFrame 偏移（x86_64, repr(C), 每项 8 字节）
.equ TF_CS, 160
.equ TF_K_CR3, 0
.equ TF_K_SP, 8
.equ TF_SIZE, 0xC0

.extern user_trap_vector
.extern kernel_trap_handler

.altmacro
.macro DEF_HANDLER, i
.Ltrap_handler_\i:
.if \i == 8 || (\i >= 10 && \i <= 14) || \i == 17
    # CPU 已压入 error_code
    push \i
    jmp .Ltrap_common
.else
    # 统一补齐 error_code，保持 TrapFrame 布局一致
    push 0
    push \i
    jmp .Ltrap_common
.endif
.endm

.macro DEF_TABLE_ENTRY, i
    .quad .Ltrap_handler_\i
.endm

.section .text.trampoline
.code64

.set i, 0
.rept NUM_INT
    DEF_HANDLER %i
    .set i, i + 1
.endr

.Ltrap_common:
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

    # 让rsp指向TrapFrame
    sub rsp, 2 * 8

    # 统一入口：根据保存的 CS.DPL 分流内核态/用户态路径。
    mov rax, [rsp + TF_CS]
    and rax, 0x3
    cmp rax, 0x3
    je .Lfrom_user

    mov rdi, rsp
    lea r13, [rip + kernel_trap_handler]
    call r13

    # 跳过k_sp和k_cr3
    add rsp, 2 * 8

    pop r15
    pop r14
    pop r13
    pop r12
    pop rbp
    pop rbx
    pop r11
    pop r10
    pop r9
    pop r8
    pop rsi
    pop rdi
    pop rdx
    pop rcx
    pop rax

    # 跳过 error_code/vector
    add rsp, 2 * 8
    iretq

.Lfrom_user:
    # 用户态进入内核后切到内核 GS 基址（percpu）。
    swapgs

    # 从任务 TrapFrame 取内核 CR3/栈。
    # 注意：切 CR3 前先取完所需值，避免切换后地址不可见。
    mov r14, [rsp + TF_K_SP]
    mov r15, [rsp + TF_K_CR3]
    mov cr3, r15

    # 从此处开始进入内核地址空间，rsp切换到k_sp，指向内核栈顶。
    mov rsp, r14

    # 调用 user_trap_vector，返回后继续执行恢复逻辑
    lea r13, [rip + user_trap_vector]
    call r13

    # handler 返回约定：rax=user_cr3, rdx=trap_cx_ptr
    mov cr3, rax
    # 切 rsp 回用户 TrapFrame
    mov rsp, rdx
    # 跳过前置内核字段区，按寄存器布局恢复。
    add rsp, 2 * 8

    # 恢复用户寄存器
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbp
    pop rbx
    pop r11
    pop r10
    pop r9
    pop r8
    pop rsi
    pop rdi
    pop rdx
    pop rcx
    pop rax

    # 恢复用户 GS 基址
    swapgs

    add rsp, 2 * 8  # 跳过 error_code/vector
    iretq

.global x86_trampoline_return
x86_trampoline_return:
    # rdi: user_cr3, rsi: trap_frame_virt_ptr
    mov cr3, rdi
    mov rsp, rsi

    # 返回用户态前恢复用户 GS 基址。
    swapgs

    # 跳过前置内核字段区k_cr3/k_sp，按寄存器布局恢复。
    add rsp, 2 * 8

    pop r15
    pop r14
    pop r13
    pop r12
    pop rbp
    pop rbx
    pop r11
    pop r10
    pop r9
    pop r8
    pop rsi
    pop rdi
    pop rdx
    pop rcx
    pop rax

    # 跳过 error_code/vector
    add rsp, 2 * 8
    iretq
.section .rodata
.global trap_handler_table
trap_handler_table:
.set i, 0
.rept NUM_INT
    DEF_TABLE_ENTRY %i
    .set i, i + 1
.endr
