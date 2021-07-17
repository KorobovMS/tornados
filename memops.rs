#[no_mangle]
pub unsafe extern "C" fn memcpy(dst: *mut u8, src: *const u8, size: usize) -> *mut u8 {
    let mut i = 0usize;
    while i < size {
        *dst.add(i) = *src.add(i);
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
            *dst.add(i) = *src.add(i);
        }
    } else {
        let mut i = 0usize;
        while i < size {
            *dst.add(i) = *src.add(i);
            i += 1;
        }
    }
    dst
}

#[no_mangle]
pub unsafe extern "C" fn memset(mem: *mut u8, c: i32, size: usize) -> *mut u8 {
    let mut i = 0;
    while i < size {
        *mem.add(i) = c as u8;
        i += 1;
    }
    mem
}

#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        let a = *s1.add(i);
        let b = *s2.add(i);
        if a != b {
            return a as i32 - b as i32;
        }
        i += 1;
    }
    0
}
