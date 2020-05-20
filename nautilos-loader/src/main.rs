//! This crate represents the kernel loader.

#![no_std]
#![cfg_attr(not(doc), no_main)]
#![doc(html_no_source)]

#![feature(panic_info_message)]

mod panic;

use core::sync::atomic::Ordering;

use efi::{
	EfiHandle,
	EfiStatus,
	EfiSystemTable,
	protocols::console::EfiSimpleTextOutputProtocol,
};

/// Macro for printing formatted strings on the general console output. It uses [`panic`]'s [`CON_OUT`] to acquire pointer to the console output protocol's interface.
/// 
/// [`panic`]: panic/index.html
/// [`CON_OUT`]: panic/static.CON_OUT.html
#[macro_export]
macro_rules! print {
	($($args:tt)+) => {
		{
			let con_out: *mut EfiSimpleTextOutputProtocol = panic::CON_OUT.load(Ordering::Relaxed);

			if !con_out.is_null() && (con_out as usize) % core::mem::align_of::<EfiSimpleTextOutputProtocol>() == 0 {
				match core::fmt::write(unsafe { &mut *con_out }, format_args!($($args)+)) { _ => () }
			}
		}
	};
}

/// Equivalent of [`print!()`] that appends new line character (`'\n'; 10; 0x0A`) in the end of the formatted string.
/// 
/// [`print!()`]: macro.print.html
#[macro_export]
macro_rules! println {
	() => {
		print!("\n");
	};
	($($args:tt)+) => {
		print!("{}\n", format_args!($($args)+));
	};
}

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
/// 1. Building memory map using EFI's one.
/// 1. Transfering control to the kernel initializer.
#[no_mangle]
fn efi_main(_image_handle: EfiHandle, system_table: &mut EfiSystemTable) -> EfiStatus {
	/* Verify that the system table is valid */
	if !system_table.verify_table() {
		return EfiStatus::error(0);
	}

	/* Set the output interface for the panic handler (also used by "print!()" and "println!()") */
	if let Some(con_out) = system_table.con_out() {
		panic::CON_OUT.store(con_out, Ordering::Relaxed);

		if let efi::EfiStatusEnum::Error(status, _) = con_out.clear_screen() {
			println!("[WARN] Clearing screen failed with EFI status: {}", status);
		}
	}
	
	/* Requirement: Feature detection mechanism */
	if !native::features::detection::detection_mechanism_present() {
		panic!("No feature detection mechanism present!");
	}
	
	loop {}
}
