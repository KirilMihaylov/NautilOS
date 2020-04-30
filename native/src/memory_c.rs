#![cfg(not(test))]

#[no_mangle]
unsafe extern "C" fn memcmp(ptr1: *const i8, ptr2: *const i8, length: usize) -> isize {
	for _ in 0..length {
		let diff: i8 = *ptr2 - *ptr1;
		if diff != 0 {
			return diff as isize;
		}
	}
	0
}

#[no_mangle]
unsafe extern "C" fn memcpy(mut destination: *mut u8, mut source: *const u8, length: usize) -> *mut u8 {
	let result: *mut u8 = destination;
	for _ in 0..length {
		*destination = *source;
		destination = destination.offset(1);
		source = source.offset(1);
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
