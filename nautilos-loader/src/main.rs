//! This crate represents the kernel loader.

#![no_std]
#![cfg_attr(not(doc), no_main)]
#![doc(html_no_source)]

#![feature(panic_info_message)]

mod panic;

use efi::types::EfiHandle;
use efi::status::EfiStatus;
use efi::system_table::EfiSystemTable;

#[no_mangle]
fn efi_main(_image_handle: EfiHandle, system_table: &mut EfiSystemTable) -> EfiStatus {
	/* Verify that the system table is valid */
	if !system_table.verify_table() {
		return EfiStatus::error(0);
	}

	/* Set output for the panic handler */
	panic::CON_OUT.store(system_table.con_out() as *const _ as *mut _, core::sync::atomic::Ordering::Relaxed);
	
	/* Requirement: Feature detection mechanism */
	if !native::features::detection::detection_mechanism_present() {
		panic!("No feature detection mechanism present!");
	}
	
	loop {}
}
