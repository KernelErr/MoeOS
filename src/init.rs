use crate::device::fdt::read_fdt_mem;
use crate::mem::mmu;
use crate::{info, println, warn};

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
}

pub fn init(_a0: usize, a1: usize) {
    print_banner();

    info!("Initializing kernel");
    let fdt_mem = read_fdt_mem(a1);
    match fdt::Fdt::new(&fdt_mem) {
        Ok(fdt) => {
            // TODO: Refactor
            unsafe {
                crate::timer::CLOCK_FREQ = 12500000;
            }
            process_device_info(&fdt);
        }
        Err(_) => {
            warn!("No FDT Found, switch to D1s mode");
            crate::device::d1s::config::init();
            crate::device::d1s::mq::blue_led_on();
            crate::device::d1s::mem::init();
        }
    };

    init_mmu();
    crate::trap::init();
    crate::timer::init();

    info!("Initialized kernel successfully");
}

fn print_banner() {
    println!("\n{}", BANNER);
    println!("{} {} {}\n", BUILD_NAME, BUILD_VERSION, BUILD_TIME);
}

fn process_device_info(fdt: &fdt::Fdt) {
    let memory = fdt.memory().regions().next().unwrap();
    extern "C" {
        fn boot_stack();
    }
    let heap_start = boot_stack as usize + 0x80000;
    let heap_end = memory.starting_address as usize + memory.size.unwrap();
    info!(
        "Kernel memory: 0x{:x} ~ 0x{:x} ({}MByte)",
        memory.starting_address as usize,
        heap_start as usize,
        (heap_start - memory.starting_address as usize) / 1024 / 1024
    );
    info!(
        "User memory: 0x{:x} ~ 0x{:x} ({}MByte)",
        heap_start,
        heap_end,
        (heap_end - heap_start) / 1024 / 1024
    );
    crate::mem::page::init(heap_start, memory.size.unwrap());
}

fn init_mmu() {
    mmu::init();
    let table = mmu::get_page_table();
    let table_ptr = table as usize;
    let mut table_ref = unsafe { table.as_mut().unwrap() };
    let page_bits = mmu::EntryBits::Access.val() | mmu::EntryBits::Dirty.val();

    extern "C" {
        fn boot_stack();
        fn boot_stack_top();
    }

    unsafe {
        mmu::map_range(
            &mut table_ref,
            TEXT_START,
            TEXT_END,
            mmu::EntryBits::ReadExecute.val() | page_bits,
        );

        mmu::map_range(
            &mut table_ref,
            RODATA_START,
            RODATA_END,
            mmu::EntryBits::Read.val() | page_bits,
        );

        mmu::map_range(
            &mut table_ref,
            DATA_START,
            DATA_END,
            mmu::EntryBits::ReadWrite.val() | page_bits,
        );

        mmu::map_range(
            &mut table_ref,
            BSS_START,
            BSS_END,
            mmu::EntryBits::ReadWrite.val() | page_bits,
        );

        mmu::map_range(
            &mut table_ref,
            boot_stack as usize,
            boot_stack_top as usize,
            mmu::EntryBits::ReadWriteExecute.val() | page_bits,
        );
    }

    mmu::set_satp(table_ptr);
    unsafe{
        asm!("sfence.vma");
    }

    info!("MMU ready");
}
