use core::alloc::{GlobalAlloc, Layout};
use core::ptr;
use std::cell::Cell;

struct BumpAllocator {
    start: usize,
    end: usize,
    offset: Cell<usize>,
}

impl BumpAllocator {
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end, offset: Cell::new(0) }
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let alloc_offset = self.offset.get();
        let alloc_start = self.start + alloc_offset;
        let alloc_end = alloc_start + layout.size();
        if alloc_end > self.end {
            return ptr::null_mut();
        } else {
            let ptr: usize = alloc_start;
            self.offset.set(alloc_offset + layout.size());
            return ptr as *mut u8;
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

static mut ALLOCATOR: BumpAllocator = BumpAllocator::new(0x1000_0000, 0x1000_1000);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alloc() {
        let layout = Layout::from_size_align(16, 8).unwrap();
        unsafe {
            let ptr1 = ALLOCATOR.alloc(layout);
            assert!(!ptr1.is_null());
            let ptr2 = ALLOCATOR.alloc(layout);
            assert!(!ptr2.is_null());
            assert!(!(ptr1 == ptr2));
        }
    }
}
