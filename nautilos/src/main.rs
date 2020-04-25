#![no_std]
#![no_main]

mod panic;

use efi::types::EfiHandle;
use efi::status::EfiStatus;
use efi::system_table::EfiSystemTable;

#[no_mangle]
fn efi_main(_image_handle: EfiHandle, _system_table: &mut EfiSystemTable) -> EfiStatus {
	if !native::features::detection::detection_mechanism_present() {
		panic!();
	}
	loop {}
}
