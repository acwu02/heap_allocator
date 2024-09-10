use std::alloc::{GlobalAlloc, Layout};

struct Block {
    header: usize,
    payload: usize
}

impl Block {
    pub const fn new(header: usize, payload: usize) -> Self {
        Block {
            header,
            payload
        }
    }
}

struct ImplicitAllocator {
    start: usize,
    end: usize,
}

impl ImplicitAllocator {
    pub const fn new(start: usize, end: usize) -> Self {
        ImplicitAllocator {
            start,
            end
        }
    }
}

unsafe impl GlobalAlloc for ImplicitAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let offset = 0;
        loop {
            let mut header = &self.start + offset;
            let is_free = header & 1;
            if is_free == 1 {
                header = header | 1;
                let ptr = header + 8;
                return ptr as *mut u8;
            } else {

            }
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}