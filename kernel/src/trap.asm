.equ XLENB, 8

.macro LOAD reg, offset
    ld \reg, \offset*XLENB(sp)
.endm

.macro SAVE reg, offset
    sd \reg, \offset*XLENB(sp)
.endm

.macro SAVE_ALL
    csrrw sp, sscratch, sp
    bnez sp, from_user
from_kernel:
    csrr sp, sscratch
from_user:
    addi sp, sp, -34*XLENB
    SAVE ra, 1
    SAVE gp, 3
    SAVE tp, 4
    SAVE t0, 5
    SAVE t1, 6
    SAVE t2, 7
    SAVE s0, 8
    SAVE s1, 9
    SAVE a0, 10
    SAVE a1, 11
    SAVE a2, 12
    SAVE a3, 13
    SAVE a4, 14
    SAVE a5, 15
    SAVE a6, 16
    SAVE a7, 17
    SAVE s2, 18
    SAVE s3, 19
    SAVE s4, 20
    SAVE s5, 21
    SAVE s6, 22
    SAVE s7, 23
    SAVE s8, 24
    SAVE s9, 25
    SAVE s10, 26
    SAVE s11, 27
    SAVE t3, 28
    SAVE t4, 29
    SAVE t5, 30
    SAVE t6, 31
    csrr t0, sstatus
    SAVE t0, 32
    csrr t1, sepc
    SAVE t1, 33
    csrr t2, sscratch
    SAVE t2, 2
.endm

.macro LOAD_ALL
    LOAD t1, 32
    LOAD t2, 33
    andi t0, t1, 1 << 8
    bnez t0, to_kernel
to_user:
    addi t0, sp, 34 * XLENB
    csrw sscratch, t0
to_kernel:
    csrw sstatus, t1
    csrw sepc, t2
    LOAD ra, 1
    LOAD gp, 3
    LOAD tp, 4
    LOAD t0, 5
    LOAD t1, 6
    LOAD t2, 7
    LOAD s0, 8
    LOAD s1, 9
    LOAD a0, 10
    LOAD a1, 11
    LOAD a2, 12
    LOAD a3, 13
    LOAD a4, 14
    LOAD a5, 15
    LOAD a6, 16
    LOAD a7, 17
    LOAD s2, 18
    LOAD s3, 19
    LOAD s4, 20
    LOAD s5, 21
    LOAD s6, 22
    LOAD s7, 23
    LOAD s8, 24
    LOAD s9, 25
    LOAD s10, 26
    LOAD s11, 27
    LOAD t3, 28
    LOAD t4, 29
    LOAD t5, 30
    LOAD t6, 31
    LOAD sp, 2
.endm

    .section .text
    .globl __alltraps
    .align 4
__alltraps:
    SAVE_ALL
    mv a0, sp
    jal trap_handler

    .globl __trapret
__trapret:
    LOAD_ALL
    sret
