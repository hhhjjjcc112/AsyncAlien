# x86-64 task switch implementation
#
# 使用与 axcpu 一致的切换方式：
# - 当前任务把 callee-saved 寄存器压栈后保存 rsp
# - 下一个任务恢复 rsp 后弹栈并 ret 到其保存的 rip
#
# Arguments:
#   rdi = pointer to current TaskContext (to save)
#   rsi = pointer to next TaskContext (to restore)
#
# TaskContext layout on x86-64:
#   0x00: kstack_top
#   0x08: rsp
#   0x10: fs_base
#   0x18: gs_base

.section .text
.globl __switch
__switch:
    # 当前任务：压栈并记录切换后的 rsp
    push rbp
    push rbx
    push r12
    push r13
    push r14
    push r15
    mov [rdi + 0x08], rsp

    # 切换到下一个任务的栈
    mov rsp, [rsi + 0x08]

    # 恢复下一个任务的 callee-saved 寄存器
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    pop rbp

    # 返回到下一个任务保存的 rip
    ret
