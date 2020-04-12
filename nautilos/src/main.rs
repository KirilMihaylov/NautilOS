#![no_std]
#![no_main]

mod panic;

#[no_mangle]
fn efi_main() -> usize {
	loop {}
}
