# x86-64 task switch implementation
#
# Switches between two task contexts by saving and restoring
# callee-saved registers (rbx, rbp, r12-r15) and stack pointer.
#
# Arguments:
#   rdi = pointer to current TaskContext (to save)
#   rsi = pointer to next TaskContext (to restore)
#
# TaskContext layout on x86-64:
#   0x00: rip (return address)
#   0x08: rsp (stack pointer)
#   0x10: rbx
#   0x18: rbp
#   0x20: r12
#   0x28: r13
#   0x30: r14
#   0x38: r15
#   0x40: fp_simd (fxsave64/fxrstor64, 512 bytes)

.section .text
.globl __switch
__switch:
    # Save current task context
    # Save return address (after this function returns)
    lea rax, [rip + .Lswitch_return]
    mov [rdi + 0x00], rax
    
    # Save callee-saved registers
    mov [rdi + 0x08], rsp
    mov [rdi + 0x10], rbx
    mov [rdi + 0x18], rbp
    mov [rdi + 0x20], r12
    mov [rdi + 0x28], r13
    mov [rdi + 0x30], r14
    mov [rdi + 0x38], r15
    fxsave64 [rdi + 0x40]
    
    # Restore next task context
    # Restore callee-saved registers
    mov rbx, [rsi + 0x10]
    mov rbp, [rsi + 0x18]
    mov r12, [rsi + 0x20]
    mov r13, [rsi + 0x28]
    mov r14, [rsi + 0x30]
    mov r15, [rsi + 0x38]
    fxrstor64 [rsi + 0x40]
    mov rsp, [rsi + 0x08]
    
    # Jump to saved return address
    jmp [rsi + 0x00]

.Lswitch_return:
    ret
