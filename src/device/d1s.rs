use crate::println;

pub fn memory_init() {
    extern "C" {
        fn end();
        fn _heap_start();
    }
    let heap_start = _heap_start as usize;
    let memory_size = 64 * 1024 * 1024;
    let memory_start = 0x80000000;
    let heap_end = memory_size + memory_start;
    println!(
        "Kernel memory: 0x{:x} ~ 0x{:x} ({}MByte)",
        memory_start,
        end as usize,
        (end as usize - memory_start) / 1024 / 1024
    );
    println!(
        "User memory: 0x{:x} ~ 0x{:x} ({}MByte)",
        heap_start,
        heap_end,
        (heap_end - heap_start) / 1024 / 1024
    );
    crate::mem::page::init(heap_start, memory_size);
}