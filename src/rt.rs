#[no_mangle]
#[inline(never)]
unsafe fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *s.offset(i as isize) = c as u8;
        i += 1;
    }
    return s;
}

#[no_mangle]
pub unsafe extern "C" fn __aeabi_memclr4(s: *mut u32, n: usize) {
    let mut n = n;
    let mut dest = s;
    while n != 0 {
        *dest = 0;
        dest = dest.offset(1);
        n -= 4;
    }
}

#[no_mangle]
pub unsafe extern "C" fn __aeabi_memcpy4(dest: *mut u32, src: *const u32, n: usize) {
    let mut i = 0;
    let mut dest = dest;
    let mut src = src;
    while i < n {
        *dest = *src;
        dest = dest.offset(1);
        src = src.offset(1);
        i += 4;
    }
}
