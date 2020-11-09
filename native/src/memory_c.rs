#![cfg(not(test))]

#[no_mangle]
unsafe extern "C" fn memcmp(ptr1: *const i8, ptr2: *const i8, length: usize) -> isize {
    if ptr1 == ptr2 {
        return 0;
    }

    for _ in 0..length {
        let diff: i8 = *ptr2 - *ptr1;
        if diff != 0 {
            return diff as isize;
        }
    }

    0
}

#[no_mangle]
#[inline(always)]
unsafe extern "C" fn memcpy(destination: *mut u8, source: *const u8, length: usize) -> *mut u8 {
    memmove(destination, source, length)
}

#[no_mangle]
unsafe extern "C" fn memmove(
    mut destination: *mut u8,
    mut source: *const u8,
    length: usize,
) -> *mut u8 {
    if source == destination {
        return destination;
    }

    let result: *mut u8 = destination;

    let offset: isize = if (destination as usize) < (source as usize) {
        1
    } else {
        -1
    };

    for _ in 0..length {
        *destination = *source;
        destination = destination.offset(offset);
        source = source.offset(offset);
    }

    result
}

#[no_mangle]
unsafe extern "C" fn memset(mut ptr: *mut u8, byte: u8, length: usize) -> *mut u8 {
    let result: *mut u8 = ptr;

    for _ in 0..length {
        *ptr = byte;
        ptr = ptr.offset(1);
    }

    result
}
