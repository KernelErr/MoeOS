use core::mem::size_of;
use core::ptr::null_mut;

static mut HEAP_START: usize = 0;
static mut HEAP_SIZE: usize = 0;
static mut ALLOC_START: usize = 0;
const PAGE_ORDER: usize = 12;
pub const PAGE_SIZE: usize = 1 << 12; // 4096 = 4KBytes

fn align_val(val: usize, order: usize) -> usize {
    let o = (1usize << order) - 1;
    (val + o) & !o
}

#[repr(u8)]
enum PageBits {
    Empty = 0,
    Taken = 1,
    Last = 2,
}

impl PageBits {
    pub fn val(self) -> u8 {
        self as u8
    }
}

impl From<PageBits> for u8 {
    fn from(pb: PageBits) -> u8 {
        pb as u8
    }
}

struct Page {
    flags: u8,
}

impl Page {
    pub fn set_flags(&mut self, flags: PageBits) {
        self.flags = flags.into();
    }

    pub fn clear(&mut self) {
        self.flags = PageBits::Empty.into();
    }

    pub fn is_taken(&self) -> bool {
        self.flags & PageBits::Taken.val() != 0u8
    }

    pub fn is_free(&self) -> bool {
        !self.is_taken()
    }

    pub fn is_last(&self) -> bool {
        self.flags & PageBits::Last.val() != 0u8
    }
}

pub fn init(heap_start: usize, heap_size: usize) {
    unsafe {
        HEAP_START = heap_start;
        HEAP_SIZE = heap_size;
        let num_pages = HEAP_SIZE / PAGE_SIZE;
        let p = HEAP_START as *mut Page;
        for i in 0..num_pages {
            (*p.add(i)).clear();
        }
        ALLOC_START = align_val(HEAP_START + num_pages * size_of::<Page>(), PAGE_ORDER);
    }
}

pub fn alloc(pages: usize) -> *mut u8 {
    if pages == 0 {
        return null_mut();
    }
    unsafe {
        let num_pages = HEAP_SIZE / PAGE_SIZE;
        let p = HEAP_START as *mut Page;
        for i in 0..num_pages - pages {
            let mut found = false;
            if (*p.add(i)).is_free() {
                found = true;
                for j in i..i + pages {
                    if (*p.add(j)).is_taken() {
                        found = false;
                        break;
                    }
                }
            }
            if found {
                for k in i..=i + pages - 1 {
                    (*p.add(k)).set_flags(PageBits::Taken);
                }
                (*p.add(i + pages - 1)).set_flags(PageBits::Last);
                return (i * PAGE_SIZE + ALLOC_START) as *mut u8;
            }
        }
    }
    null_mut()
}

pub fn zalloc(pages: usize) -> *mut u8 {
    let ret = alloc(pages);
    if !ret.is_null() {
        let size = (PAGE_SIZE * pages) / 8;
        let p = ret as *mut u64;
        for i in 0..size {
            unsafe {
                *p.add(i) = 0;
            }
        }
    }
    ret
}

pub fn dealloc(ptr: *mut u8) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let addr = HEAP_START + (ptr as usize - ALLOC_START) / PAGE_SIZE;
        if (addr < HEAP_START) || (addr >= HEAP_START + HEAP_SIZE) {
            return;
        }
        let mut p = addr as *mut Page;
        while (*p).is_taken() && !(*p).is_last() {
            (*p).clear();
            p = p.add(1);
        }
        if !(*p).is_last() {
            panic!("Possible double-free detected.");
        }
        (*p).clear();
    }
}
