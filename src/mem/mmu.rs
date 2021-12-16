use super::page::{align, dealloc, zalloc, PAGE_SIZE};
use core::ptr::null_mut;
use riscv::register::satp;

static mut PAGE_TABLE: *mut Table = null_mut();

pub fn init() {
    unsafe {
        PAGE_TABLE = zalloc(1) as *mut Table;
    }
}

pub fn get_page_table() -> *mut Table {
    unsafe { PAGE_TABLE }
}

pub fn set_satp(addr: usize) {
    let ppn = addr >> 12;
    unsafe {
        satp::set(satp::Mode::Sv39, 0, ppn);
    }
}

#[repr(u64)]
#[derive(Clone, Copy, Debug)]
pub enum EntryBits {
    None = 0,
    Valid = 1 << 0,
    Read = 1 << 1,
    Write = 1 << 2,
    Execute = 1 << 3,
    User = 1 << 4,
    Global = 1 << 5,
    Access = 1 << 6,
    Dirty = 1 << 7,

    ReadWrite = 1 << 1 | 1 << 2,
    ReadExecute = 1 << 1 | 1 << 3,
    ReadWriteExecute = 1 << 1 | 1 << 2 | 1 << 3,

    UserReadWrite = 1 << 1 | 1 << 2 | 1 << 4,
    UserReadExecute = 1 << 1 | 1 << 3 | 1 << 4,
    UserReadWriteExecute = 1 << 1 | 1 << 2 | 1 << 3 | 1 << 4,

    // T-HEAD Extend
    Sec = 1 << 59,
    Buffer = 1 << 61,
    Cacheable = 1 << 62,
    StrongOrder = 1 << 63,
}

impl EntryBits {
    pub fn val(self) -> u64 {
        self as u64
    }
}

pub struct Entry {
    pub entry: u64,
}

impl Entry {
    pub fn is_valid(&self) -> bool {
        self.entry & EntryBits::Valid.val() != 0
    }

    pub fn is_invaliad(&self) -> bool {
        !self.is_valid()
    }

    pub fn is_leaf(&self) -> bool {
        self.entry & 0xe != 0
    }

    pub fn is_branch(&self) -> bool {
        !self.is_leaf()
    }

    pub fn set_entry(&mut self, entry: u64) {
        self.entry = entry;
    }
}

pub struct Table {
    pub entries: [Entry; 512],
}

impl Table {
    pub fn len() -> usize {
        512
    }
}

pub fn map(root: &mut Table, vaddr: usize, paddr: usize, bits: u64, level: usize) {
    if bits & 0xe == 0 {
        panic!("Invalid PTE bits found");
    }

    let vpn = [
        // VPN[0] = vaddr[20:12]
        (vaddr >> 12) & 0x1ff,
        // VPN[1] = vaddr[29:21]
        (vaddr >> 21) & 0x1ff,
        // VPN[2] = vaddr[38:30]
        (vaddr >> 30) & 0x1ff,
    ];

    let ppn = [
        // PPN[0] = paddr[20:12]
        (paddr >> 12) & 0x1ff,
        // PPN[1] = paddr[29:21]
        (paddr >> 21) & 0x1ff,
        // PPN[2] = paddr[55:30]
        (paddr >> 30) & 0x3ff_ffff,
    ];

    let mut v = &mut root.entries[vpn[2]];

    for i in (level..2).rev() {
        if !v.is_valid() {
            let page = zalloc(1);
            v.set_entry(
                // Set PTE's PPN
                // As page size if 4kb, we have 12 zeros
                // Set the address start from 10th bit
                (page as u64 >> 2) | EntryBits::Valid.val(),
            );
        }
        // Store next level PTE
        let entry = ((v.entry & !0x3ff) << 2) as *mut Entry;
        v = unsafe { entry.add(vpn[i]).as_mut().unwrap() };
    }

    // TODO: Complete C-SKY Extentions
    let entry = (ppn[2] << 28) as u64 |   // PPN[2] = [53:28]
	            (ppn[1] << 19) as u64 |   // PPN[1] = [27:19]
				(ppn[0] << 10) as u64 |   // PPN[0] = [18:10]
				bits |
				EntryBits::Valid.val();
    v.set_entry(entry);
}

pub fn unmap(root: &mut Table) {
    for v2 in 0..Table::len() {
        let entry_v2 = &root.entries[v2];
        if entry_v2.is_valid() && entry_v2.is_branch() {
            let v1_addr = (entry_v2.entry & !0x3ff) << 2;
            let v1_table = unsafe { &mut *(v1_addr as *mut Table) };
            for v1 in 0..Table::len() {
                let entry_v1 = &v1_table.entries[v1];
                if entry_v1.is_valid() && entry_v1.is_branch() {
                    let v0_addr = (entry_v1.entry & !0x3ff) << 2;
                    dealloc(v0_addr as *mut u8);
                }
            }
            dealloc(v1_addr as *mut u8);
        }
    }
}

pub fn virt_to_phys(root: &Table, vaddr: usize) -> Option<usize> {
    let vpn = [
        // VPN[0] = vaddr[20:12]
        (vaddr >> 12) & 0x1ff,
        // VPN[1] = vaddr[29:21]
        (vaddr >> 21) & 0x1ff,
        // VPN[2] = vaddr[38:30]
        (vaddr >> 30) & 0x1ff,
    ];

    let mut v = &root.entries[vpn[2]];
    for i in (0..=2).rev() {
        if !v.is_valid() {
            break;
        } else if v.is_leaf() {
            let off_mask = (1 << (12 + i * 9)) - 1;
            let vaddr = vaddr & off_mask;
            let addr = ((v.entry << 2) as usize) & !off_mask;
            return Some(addr | vaddr);
        }
        let entry = ((v.entry & !0x3ff) << 2) as *mut Entry;
        v = unsafe { entry.add(vpn[i - 1]).as_mut().unwrap() };
    }

    None
}

pub fn map_range(root: &mut Table, start: usize, end: usize, bits: u64) {
    let mut addr = start & !(PAGE_SIZE - 1);
    let num_pages = (align(end, 12) - addr) / PAGE_SIZE;

    for _ in 0..num_pages {
        map(root, addr, addr, bits, 0);
        addr += PAGE_SIZE;
    }
}
