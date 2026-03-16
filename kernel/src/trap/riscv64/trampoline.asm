
.altmacro
.section trampsec
.globl user_v
.globl user_r
.align 2

# low-level trap entry
define __alltraps
user_v:
        # swap a0 and sscratch
        csrrw a0, sscratch, a0

        # save user register
        # save other general purpose registers
        sd ra, 40(a0)
        sd gp, 48(a0)
        sd tp, 56(a0)
        sd t0, 64(a0)
        sd t1, 72(a0)
        sd t2, 80(a0)
        sd s0, 88(a0)
        sd s1, 96(a0)
        sd a1, 112(a0)
        sd a2, 120(a0)
        sd a3, 128(a0)
        sd a4, 136(a0)
        sd a5, 144(a0)
        sd a6, 152(a0)
        sd a7, 160(a0)
        sd s2, 168(a0)
        sd s3, 176(a0)
        sd s4, 184(a0)
        sd s5, 192(a0)
        sd s6, 200(a0)
        sd s7, 208(a0)
        sd s8, 216(a0)
        sd s9, 224(a0)
        sd s10, 232(a0)
        sd s11, 240(a0)
        sd t3, 248(a0)
        sd t4, 256(a0)
        sd t5, 264(a0)
        sd t6, 272(a0)
        # save user stack pointer in u_trap->regs.sp
        csrr t0, sscratch
        sd t0, 16(a0)

        # save the kernel task trap handler in u_trap->trap_handler
        ld t1, 288(a0)
        sd t1, 304(a0)

        # save the user trap before entering user mode in u_trap->epc
        csrr t2, sepc
        sd t2, 280(a0)

        # save the kernel stack pointer in u_trap->kernel_sp
        ld sp, 296(a0)
        addi sp, sp, 272

        # save user satp in u_trap->user_satp
        csrr t0, satp
        sd t0, 312(a0)

        # save kernel satp in u_trap->kernel_satp
        # csrr t0, satp
        # sd t0, 320(a0)

        # load trap_handler into t1
        ld t1, 304(a0)

        # move user trap frame into a0
        ld a0, -272(sp)

        # save user a0 in u_trap->regs.a0
        csrrw t0, sscratch, t0
        sd t0, 104(a0)

        # save user tp in u_trap->regs.tp
        # csrr t0, tp
        # sd t0, 56(a0)

        # jump to trap_handler
        jr t1

# low-level trap return
user_r:
        # switch to user page table
        csrw satp, a1
        sfence.vma

        # put trap frame in sscratch
        csrw sscratch, a0

        # restore general purpose register except for a0 and a1 and t0
        ld ra, 40(a0)
        ld gp, 48(a0)
        ld tp, 56(a0)
        ld t0, 64(a0)
        ld t1, 72(a0)
        ld t2, 80(a0)
        ld s0, 88(a0)
        ld s1, 96(a0)
        # restore user a0 to t0
        ld t0, 104(a0)
        ld a2, 120(a0)
        ld a3, 128(a0)
        ld a4, 136(a0)
        ld a5, 144(a0)
        ld a6, 152(a0)
        ld a7, 160(a0)
        ld s2, 168(a0)
        ld s3, 176(a0)
        ld s4, 184(a0)
        ld s5, 192(a0)
        ld s6, 200(a0)
        ld s7, 208(a0)
        ld s8, 216(a0)
        ld s9, 224(a0)
        ld s10, 232(a0)
        ld s11, 240(a0)
        ld t3, 248(a0)
        ld t4, 256(a0)
        ld t5, 264(a0)
        ld t6, 272(a0)

        # set SPP to User for user mode
        li t1, 1 << 8
        csrrc x0, sstatus, t1

        # set SPIE for interrupt
        li t1, 1 << 5
        csrrs x0, sstatus, t1

        # set sepc to the value in trap frame
        ld t1, 280(a0)
        csrw sepc, t1

        # set stack pointer to user stack pointer
        ld sp, 16(a0)

        # swap a0 and sscratch so that a0 is user a0 and sscratch is trapframe
        csrrw a0, sscratch, t0

        # jump to user mode
        sret
