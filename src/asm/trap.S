# Disable the generation of compressed instructions
.option norvc
# Enable alternate macro mode
.altmacro

# Save s register to sp+8*t
.macro SAVE_REG n
	sd	x\n, \n*8(sp)
.endm

# Load sp+8*s to t register
.macro LOAD_REG n
	ld	x\n, \n*8(sp)
.endm

.section .text
.global _traps
.align 4

_traps:
    # Currently we only have kernel stack
    csrw sscratch, sp
    # Right sp -> sscratch -> Left sp
    csrrw sp, sscratch, sp

    addi sp, sp, -34*8
    sd x1, 1*8(sp)
    sd x3, 3*8(sp)
    .set n,5
    .rept 27
        SAVE_REG %n
        .set n, n+1
    .endr

    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)

    csrr t2, sscratch
    sd t2, 2*8(sp)
    mv a0, sp
    call trap_handler

.global _traps_end
_traps_end:
    mv sp, a0
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    ld t2, 2*8(sp)
    csrw sstatus, t0
    csrw sepc, t1
    csrw sscratch, t2
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    .set n,5
    .rept 27
        LOAD_REG %n
        .set n, n+1
    .endr
    addi sp, sp, 34*8
    csrrw sp, sscratch, sp
    sret