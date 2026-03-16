
.altmacro
.section .text
.global kernel_v
.align 2
kernel_v:
        # save context
        addi sp, sp, -16 * 17
        sd x1, 0 * 8(sp)
        # sd x2, 1 * 8(sp)
        sd x4, 2 * 8(sp)
        sd x5, 3 * 8(sp)
        sd x6, 4 * 8(sp)
        sd x7, 5 * 8(sp)
        sd x8, 6 * 8(sp)
        sd x9, 7 * 8(sp)
        sd x10, 8 * 8(sp)
        sd x11, 9 * 8(sp)
        sd x12, 10 * 8(sp)
        sd x13, 11 * 8(sp)
        sd x14, 12 * 8(sp)
        sd x15, 13 * 8(sp)
        sd x16, 14 * 8(sp)
        sd x17, 15 * 8(sp)

        # call the C trap handler in trap.c
        mv a0, sp
        call kernel_trap_vector

.global __return_to_kernel
__return_to_kernel:
        # restore context
        ld x1, 0 * 8(sp)
        # ld x2, 1 * 8(sp)
        ld x4, 2 * 8(sp)
        ld x5, 3 * 8(sp)
        ld x6, 4 * 8(sp)
        ld x7, 5 * 8(sp)
        ld x8, 6 * 8(sp)
        ld x9, 7 * 8(sp)
        ld x10, 8 * 8(sp)
        ld x11, 9 * 8(sp)
        ld x12, 10 * 8(sp)
        ld x13, 11 * 8(sp)
        ld x14, 12 * 8(sp)
        ld x15, 13 * 8(sp)
        ld x16, 14 * 8(sp)
        ld x17, 15 * 8(sp)
        addi sp, sp, 16 * 17
        sret
