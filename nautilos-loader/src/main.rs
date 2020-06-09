//! This crate represents the kernel loader.

#![no_std]
#![cfg_attr(not(doc), no_main)]
#![doc(html_no_source)]

#![feature(panic_info_message)]

mod panic_handling;
use panic_handling::CON_OUT;

use core::sync::atomic::Ordering;

use efi::{
	EfiHandle,
	EfiStatus,
	EfiSystemTable,
	protocols::console::EfiSimpleTextOutputProtocol,
};

use native::features::detection::*;

/// Macro for printing formatted strings on the general console output.
/// It uses [`panic_handling`]'s [`CON_OUT`] to acquire pointer to the console output protocol's interface.
#[macro_export]
macro_rules! print {
	($($args:tt)+) => {
		{
			let con_out: *mut EfiSimpleTextOutputProtocol = CON_OUT.load(Ordering::Relaxed);

			if !con_out.is_null() && (con_out as usize) % core::mem::align_of::<EfiSimpleTextOutputProtocol>() == 0 {
				match core::fmt::write(unsafe { &mut *con_out }, format_args!($($args)+)) { _ => () }
			}
		}
	};
}

/// Equivalent of [`print!`] that appends new line character (`'\n'; 10; 0x0A`) in the end of the formatted string.
#[macro_export]
macro_rules! println {
	() => {
		print!("\n");
	};
	($($args:tt)+) => {
		print!("{}\n", format_args!($($args)+));
	};
}

/// Equivalent of [`println!`] that appends `[LOG] ` in the beginning of the formatted string.
#[macro_export]
macro_rules! log {
	($($args:tt)+) => {
		println!("[LOG] {}", format_args!($($args)+));
	}
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

	/* Set the output interface for the panic handler (also used by "print!" and "println!") */
	if let Some(con_out) = system_table.con_out() {
		CON_OUT.store(con_out, Ordering::Relaxed);

		if let efi::EfiStatusEnum::Error(status, _) = con_out.clear_screen() {
			println!("[WARN] Clearing screen failed with EFI status: {}", status);
		}
	}
	
	/* Requirement: Feature detection mechanism */
	match detection_mechanism_available() {
		Ok(FeatureState::Enabled) => log!("Feature detection mechanism available."),
		Ok(FeatureState::Disabled) => {
			log!("Feature detection mechanism available but is disabled.");
			log!("Enabling feature detection mechanism...");
			match enable_detection_mechanism() {
				Ok(FeatureState::Enabled) => (),
				_ => panic!("Can not enable feature detection mechanism!"),
			}
		},
		Err(native::Error::Unavailable) => panic!("Feature detection mechanism unavailable!"),
		Err(_) => panic!("Error occured while testing for feature detection mechanism!"),
	}
	
	loop {}
}
