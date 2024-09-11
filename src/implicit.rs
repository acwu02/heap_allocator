use std::alloc::{GlobalAlloc, Layout};
use crate::utils::is_free;
use crate::utils::get_payload_size;
struct ImplicitAllocator {
    start_heap: *mut usize,
    end_heap: *mut usize,
}

impl ImplicitAllocator {
    pub fn new(buffer: &mut [u8]) -> Self {
        let start_heap = buffer.as_mut_ptr() as *mut usize;
        let mut heap_size = buffer.len();
        unsafe {
            let end_heap = unsafe { start_heap.add(heap_size / std::mem::size_of::<usize>()) };
            *start_heap = heap_size as usize | 1;
            ImplicitAllocator {
                start_heap,
                end_heap,
            }

        }
    }
}

unsafe impl GlobalAlloc for ImplicitAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let requested_size = layout.size();
        let mut offset = 0;
        loop {
            let header_ptr = self.start_heap.add(offset);
            if header_ptr > self.end_heap {
                return std::ptr::null_mut();
            }
            let mut header = *header_ptr;
            let block_size = get_payload_size(header_ptr);
            if is_free(header_ptr) && block_size >= requested_size {
                let next_free_header = header_ptr.add(requested_size);
                *next_free_header = (block_size - requested_size) | 1;
                header = requested_size | 0;
                *header_ptr = header;
                let ptr = header_ptr;
                return ptr as *mut u8;
            } else {
                offset += block_size;
            }
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut header = *ptr;
        header = header | 1;
        *ptr = header;
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        self.dealloc(ptr, layout);
        let new_ptr = self.alloc(Layout::from_size_align(new_size, 8).unwrap());
        new_ptr
    }
}

fn init_heap_allocator() -> ImplicitAllocator {
    const HEAP_SIZE: usize = 4096;
    let mut heap_buffer: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
    return ImplicitAllocator::new(&mut heap_buffer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alloc() {
        let mut ALLOCATOR: ImplicitAllocator = init_heap_allocator();
        let layout1 = Layout::from_size_align(16, 8).unwrap();
        let layout2 = Layout::from_size_align(32, 8).unwrap();
        unsafe {
            let ptr1 = ALLOCATOR.alloc(layout1);
            assert!(!ptr1.is_null());
            let ptr2 = ALLOCATOR.alloc(layout2);
            assert!(!ptr2.is_null());
            assert!(!(ptr1 == ptr2));
        }
    }

    #[test]
    fn test_dealloc() {
        let mut ALLOCATOR: ImplicitAllocator = init_heap_allocator();
        let layout = Layout::from_size_align(16, 8).unwrap();
        unsafe {
            let ptr1 = ALLOCATOR.alloc(layout);
            ALLOCATOR.dealloc(ptr1, layout);
            // TODO: assertions
            assert!(!ptr1.is_null());
            assert!(is_free(ptr1 as *mut usize));
            let ptr2 = ALLOCATOR.alloc(layout);
            assert!(ptr1 == ptr2);
        }
    }

    #[test]
    fn test_realloc() {
        let mut ALLOCATOR: ImplicitAllocator = init_heap_allocator();
        let layout = Layout::from_size_align(16, 8).unwrap();
        unsafe {
            let ptr1: *mut u8 = ALLOCATOR.alloc(layout);
            let new_ptr1 = ALLOCATOR.realloc(ptr1, layout, 32);
            assert!(ptr1 != new_ptr1);
            assert!(get_payload_size(new_ptr1 as *mut usize) == 32);
            let ptr2 = ALLOCATOR.alloc(layout);
            let new_ptr2 = ALLOCATOR.realloc(ptr2, layout, 8);
            assert!(ptr2 == new_ptr2);
            assert!(get_payload_size(new_ptr2 as *mut usize) == 8);
        }
    }

    #[test]
    fn test_heap_overflow() {
        let mut ALLOCATOR: ImplicitAllocator = init_heap_allocator();
        let layout_overflow = Layout::from_size_align(8192, 8).unwrap();
        unsafe {
            let ptr_overflow = ALLOCATOR.alloc(layout_overflow);
            assert!(ptr_overflow == std::ptr::null_mut());
            assert!(is_free(ALLOCATOR.start_heap as *mut usize));
        }
    }
}
