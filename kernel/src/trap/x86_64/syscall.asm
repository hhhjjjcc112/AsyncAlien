.section .text
.code64
.global syscall_entry

# TrapFrame 偏移（x86_64, repr(C), 每项 8 字节）
.equ TF_R15, 0
.equ TF_R14, 8
.equ TF_R13, 16
.equ TF_R12, 24
.equ TF_RBP, 32
.equ TF_RBX, 40
.equ TF_R11, 48
.equ TF_R10, 56
.equ TF_R9, 64
.equ TF_R8, 72
.equ TF_RSI, 80
.equ TF_RDI, 88
.equ TF_RDX, 96
.equ TF_RCX, 104
.equ TF_RAX, 112
.equ TF_VECTOR, 120
.equ TF_ERROR_CODE, 128
.equ TF_RIP, 136
.equ TF_RFLAGS, 152
.equ TF_RSP, 160
.equ TF_K_SP, 184

.equ MSR_KERNEL_GS_BASE, 0xC0000102
.equ SYSCALL_VECTOR, 0x80

syscall_entry:
    # 保存会被 rdmsr 覆盖的寄存器，以及一个临时寄存器。
    push rax
    push rdx
    push rcx
    push rbx
    push r12

    mov ecx, MSR_KERNEL_GS_BASE
    rdmsr
    shl rdx, 32
    or rax, rdx
    mov r12, rax                  # r12 = 当前任务 TrapFrame 虚拟地址

    mov rax, [rsp]
    mov [r12 + TF_R12], rax
    mov rax, [rsp + 8]
    mov [r12 + TF_RBX], rax
    mov rax, [rsp + 16]
    mov [r12 + TF_RIP], rax
    mov [r12 + TF_RCX], rax
    mov rax, [rsp + 24]
    mov [r12 + TF_RDX], rax
    mov rax, [rsp + 32]
    mov [r12 + TF_RAX], rax

    mov [r12 + TF_R15], r15
    mov [r12 + TF_R14], r14
    mov [r12 + TF_R13], r13
    mov [r12 + TF_RBP], rbp
    mov [r12 + TF_R11], r11
    mov [r12 + TF_R10], r10
    mov [r12 + TF_R9], r9
    mov [r12 + TF_R8], r8
    mov [r12 + TF_RSI], rsi
    mov [r12 + TF_RDI], rdi

    lea rax, [rsp + 40]
    mov [r12 + TF_RSP], rax
    mov [r12 + TF_RFLAGS], r11
    mov qword ptr [r12 + TF_VECTOR], SYSCALL_VECTOR
    mov qword ptr [r12 + TF_ERROR_CODE], 0

    mov rsp, [r12 + TF_K_SP]
    call x86_syscall_handler

    # 从 TrapFrame 恢复用户上下文，用 sysretq 返回用户态。
    mov r15, [r12 + TF_R15]
    mov r14, [r12 + TF_R14]
    mov r13, [r12 + TF_R13]
    mov rbp, [r12 + TF_RBP]
    mov rbx, [r12 + TF_RBX]
    mov r10, [r12 + TF_R10]
    mov r9, [r12 + TF_R9]
    mov r8, [r12 + TF_R8]
    mov rsi, [r12 + TF_RSI]
    mov rdi, [r12 + TF_RDI]
    mov rdx, [r12 + TF_RDX]
    mov rax, [r12 + TF_RAX]

    mov rcx, [r12 + TF_RIP]
    mov r11, [r12 + TF_RFLAGS]
    mov rsp, [r12 + TF_RSP]
    mov r12, [r12 + TF_R12]
    sysretq
