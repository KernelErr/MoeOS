.section .rodata

.global TEXT_START
TEXT_START: .dword stext

.global TEXT_END
TEXT_END: .dword etext

.global DATA_START
DATA_START: .dword sdata

.global DATA_END
DATA_END: .dword edata

.global RODATA_START
RODATA_START: .dword srodata

.global RODATA_END
RODATA_END: .dword erodata

.global BSS_START
BSS_START: .dword sbss

.global BSS_END
BSS_END: .dword ebss
