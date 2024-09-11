pub fn is_free(ptr: *mut usize) -> bool {
    unsafe { *ptr & 1 == 1 }
}

pub fn get_payload_size(ptr: *mut usize) -> usize {
    unsafe { *ptr & !1 }
}