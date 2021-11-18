use crate::device::fdt::read_fdt_mem;
use crate::println;

const BUILD_NAME: &str = env!("BUILD_NAME");
const BUILD_VERSION: &str = env!("BUILD_VERSION");
const BUILD_TIME: &str = env!("BUILD_TIME");
const BANNER: &str = include_str!("statics/banner.txt");

pub fn init(_a0: usize, a1: usize) {
    print_banner();
    print_memory_info(a1);

    let fdt_mem = read_fdt_mem(a1);
    let fdt = fdt::Fdt::new(&fdt_mem).unwrap();

    process_device_info(&fdt);
}

fn print_banner() {
    println!();
    println!("{}", BANNER);
    println!("{} {} {}", BUILD_NAME, BUILD_VERSION, BUILD_TIME);
    println!();
}

fn print_memory_info(fdt_addr: usize) {
    println!("Memory information:");
    extern "C" {
        fn _start();
        fn bootstacktop();
    }
    println!("_start vaddr = 0x{:x}", _start as usize);
    println!("bootstacktop vaddr = 0x{:x}", bootstacktop as usize);
    println!("FDT vaddr = 0x{:x}", fdt_addr);
    println!();
}

fn process_device_info(fdt: &fdt::Fdt) {
    println!("Device information:");

    println!("CPU count: {}", fdt.cpus().count());

    let memory = fdt.memory().regions().next().unwrap();
    extern "C" {
        fn _heap_start();
    }
    let heap_start = _heap_start as usize;
    let heap_end = memory.starting_address as usize + memory.size.unwrap();
    println!(
        "Available memory: 0x{:x} ~ 0x{:x} ({}MByte)",
        heap_start,
        heap_end,
        (heap_end - heap_start) / 1024 / 1024
    );
    crate::mem::page::init(heap_start, memory.size.unwrap());

    let chosen = fdt.chosen();
    if let Some(stdout) = chosen.stdout() {
        println!("Current stdout: {}", stdout.name);
    }

    // let soc = fdt.find_node("/soc");
    // if let Some(soc) = soc {
    //     println!("/soc");
    //     for child in soc.children() {
    //         println!("    {}", child.name);
    //     }
    // } else {
    //     println!("/soc not found");
    // }

    println!();
}
