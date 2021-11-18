use core::ptr::read;
use byteorder::{ByteOrder, BigEndian};

pub fn read_fdt_mem(fdt_addr: usize) -> [u8; 8192] {
    let tmp: [u8; 4] = unsafe {read((fdt_addr + 4) as *const [u8; 4])};
    let size = BigEndian::read_u32(&tmp);
    let mut buf: [u8; 8192] = [0; 8192];

    for (i, p) in buf.iter_mut().enumerate().take(size as usize) {
        *p = unsafe {read((fdt_addr + i) as *const u8)};
    }

    buf
}