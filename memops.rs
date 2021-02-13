#[no_mangle]
pub unsafe extern "C" fn memcpy(dst: *mut u8, src: *const u8, size: usize) -> *mut u8 {
    let mut i = 0usize;
    while i < size {
        *dst.offset(i as isize) = *src.offset(i as isize);
        i += 1;
    }
    dst
}

#[no_mangle]
pub unsafe extern "C" fn memmove(dst: *mut u8, src: *const u8, size: usize) -> *mut u8 {
    if src < dst as *const u8 {
        let mut i = size;
        while i != 0 {
            i -= 1;
            *dst.offset(i as isize) = *src.offset(i as isize);
        }
    } else {
        let mut i = 0usize;
        while i < size {
            *dst.offset(i as isize) = *src.offset(i as isize);
            i += 1;
        }
    }
    dst
}

#[no_mangle]
pub unsafe extern "C" fn memset(mem: *mut u8, c: i32, size: usize) -> *mut u8 {
    let mut i = 0;
    while i < size {
        *mem.offset(i as isize) = c as u8;
        i += 1;
    }
    mem
}
