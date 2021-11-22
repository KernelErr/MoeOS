    .section .text.entry
    .globl _start

    sfence.vma x0

_start:
    la sp, _stack_end
    call kstart

    .section .bss.stack
    .align 12
    .global bootstack
