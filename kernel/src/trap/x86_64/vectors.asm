.equ NUM_INT, 256

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

.section .text
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

    mov rdi, rsp
    call x86_trap_dispatch

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
.global trap_handler_table
trap_handler_table:
.set i, 0
.rept NUM_INT
    DEF_TABLE_ENTRY %i
    .set i, i + 1
.endr
