use crate::device::fdt::read_fdt_mem;
use crate::mem::{mmu, page};
use crate::println;

const BUILD_NAME: &str = env!("BUILD_NAME");
const BUILD_VERSION: &str = env!("BUILD_VERSION");
const BUILD_TIME: &str = env!("BUILD_TIME");
const BANNER: &str = include_str!("statics/banner.txt");

extern "C" {
    static TEXT_START: usize;
    static TEXT_END: usize;
    static DATA_START: usize;
    static DATA_END: usize;
    static RODATA_START: usize;
    static RODATA_END: usize;
    static BSS_START: usize;
    static BSS_END: usize;
    static KERNEL_STACK_START: usize;
    static KERNEL_STACK_END: usize;
}

pub fn init(_a0: usize, a1: usize) {
    print_banner();

    print_memory_info(a1);

    let fdt_mem = read_fdt_mem(a1);
    let fdt = fdt::Fdt::new(&fdt_mem).unwrap();

    process_device_info(&fdt);

    init_mmu();
}

fn print_banner() {
    println!("\n{}", BANNER);
    println!("{} {} {}\n", BUILD_NAME, BUILD_VERSION, BUILD_TIME);
}

fn print_memory_info(fdt_addr: usize) {
    println!("Memory information:");
    extern "C" {
        fn _start();
        fn _heap_start();
    }
    println!("_start vaddr = 0x{:x}", _start as usize);
    println!("heap_start vaddr = 0x{:x}", _heap_start as usize);
    println!("FDT vaddr = 0x{:x}\n", fdt_addr);
}

fn process_device_info(fdt: &fdt::Fdt) {
    println!("Device information:");

    println!("CPU count: {}", fdt.cpus().count());

    let memory = fdt.memory().regions().next().unwrap();
    extern "C" {
        fn end();
        fn _heap_start();
    }
    let heap_start = _heap_start as usize;
    let heap_end = memory.starting_address as usize + memory.size.unwrap();
    println!(
        "Kernel memory: 0x{:x} ~ 0x{:x} ({}MByte)",
        memory.starting_address as usize,
        end as usize,
        (end as usize - memory.starting_address as usize) / 1024 / 1024
    );
    println!(
        "User memory: 0x{:x} ~ 0x{:x} ({}MByte)",
        heap_start,
        heap_end,
        (heap_end - heap_start) / 1024 / 1024
    );
    crate::mem::page::init(heap_start, memory.size.unwrap());

    let chosen = fdt.chosen();
    if let Some(stdout) = chosen.stdout() {
        println!("Current stdout: {}", stdout.name);
    }

    let soc = fdt.find_node("/soc");
    if let Some(soc) = soc {
        println!("/soc");
        for child in soc.children() {
            println!("  |--{}", child.name);
        }
    } else {
        println!("/soc node not found in FDT");
    }

    println!();
}

fn init_mmu() {
    mmu::init();
    let table = mmu::get_page_table();
    let table_ptr = table as usize;
    let mut table_ref = unsafe { table.as_mut().unwrap() };

    unsafe {
        mmu::map_range(
            &mut table_ref,
            TEXT_START,
            TEXT_END,
            mmu::EntryBits::ReadExecute.val(),
        );

        mmu::map_range(
            &mut table_ref,
            RODATA_START,
            RODATA_END,
            mmu::EntryBits::ReadExecute.val(),
        );

        mmu::map_range(
            &mut table_ref,
            DATA_START,
            DATA_END,
            mmu::EntryBits::ReadWrite.val(),
        );

        mmu::map_range(
            &mut table_ref,
            BSS_START,
            BSS_END,
            mmu::EntryBits::ReadWrite.val(),
        );

        mmu::map_range(
            &mut table_ref,
            KERNEL_STACK_START,
            KERNEL_STACK_END,
            mmu::EntryBits::ReadWrite.val(),
        );
    }

    mmu::set_satp(table_ptr);
}
