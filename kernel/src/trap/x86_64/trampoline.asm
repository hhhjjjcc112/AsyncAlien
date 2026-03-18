.equ NUM_INT, 256

# TrapFrame 偏移（x86_64, repr(C), 每项 8 字节）
.equ TF_K_CR3, 176
.equ TF_K_SP, 184
.equ TF_TRAP_HANDLER, 192

.altmacro
.macro DEF_USER_HANDLER, i
.Luser_trap_handler_\i:
.if \i == 8 || (\i >= 10 && \i <= 14) || \i == 17
    # CPU 已压入 error_code
    push \i
    jmp .Luser_trap_common
.else
    # 统一补齐 error_code，保持 TrapFrame 布局一致
    push 0
    push \i
    jmp .Luser_trap_common
.endif
.endm

.macro DEF_USER_TABLE_ENTRY, i
    .quad .Luser_trap_handler_\i
.endm

.section .text.trampoline
.code64

.set i, 0
.rept NUM_INT
    DEF_USER_HANDLER %i
    .set i, i + 1
.endr

.Luser_trap_common:
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

    # TSS.rsp0 已指向 TrapFrame 顶部，压栈完成后 rsp 即 TrapFrame 基址。
    mov r12, rsp

    # 从任务 TrapFrame 取内核 CR3/栈，先切到内核地址空间再进入 Rust
    mov rax, [r12 + TF_K_CR3]
    mov cr3, rax
    mov rsp, [r12 + TF_K_SP]

    call [r12 + TF_TRAP_HANDLER]

    ud2

.global x86_trampoline_return
x86_trampoline_return:
    # rdi: user_cr3, rsi: trap_frame_virt_ptr
    mov cr3, rdi
    mov rsp, rsi

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

    add rsp, 16
    iretq

.section .rodata
.global user_trap_handler_table
user_trap_handler_table:
.set i, 0
.rept NUM_INT
    DEF_USER_TABLE_ENTRY %i
    .set i, i + 1
.endr
