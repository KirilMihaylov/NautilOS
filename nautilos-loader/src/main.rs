//! This crate represents the kernel loader.

#![no_std]
#![cfg_attr(not(doc), no_main)]
#![doc(html_no_source)]

#![feature(panic_info_message)]

mod panic;

use efi::types::EfiHandle;
use efi::status::EfiStatus;
use efi::system_table::EfiSystemTable;

/// Loader's main function.
/// 
/// This function acts as EFI's entry point.
/// It handles the following functionality:
/// 1. Verifying the EFI tables passed by the firmware.
/// 1. Verifying the system meets the minimum requirements.
/// 1. Finding kernel's partition.
/// 1. Finding configuration file in kernel's partition.
///     * If it exists, loads configuration.
///     * If it doesn't exist, uses default configuration.
/// 1. Setting desired (stated in the configuration) video mode (resolution, color map, etc.).
/// 1. Finding and loading suitable kernel binaries (kernel, core/generic drivers, etc.).
/// 1. Taking the ownership over the EFI firmware.
/// 1. Transfering control to the kernel initializer.
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
