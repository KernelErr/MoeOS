.section .text.entry
.globl _start

sfence.vma x0, x0

_start:
    la sp, boot_stack_top
    call kstart

    .section .bss.stack
    .global boot_stack
boot_stack:
    .space 4096 * 16
    .global boot_stack_top
boot_stack_top: