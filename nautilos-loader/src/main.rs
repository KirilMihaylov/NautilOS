//! This crate represents the kernel loader.
//! 
//! It handles the following functionality:
//! 1. Verifying the EFI tables passed by the firmware.
//! 1. Verifying the system meets the minimum requirements.
//! 1. Finding kernel's partition.
//! 1. Finding configuration file in kernel's partition.
//!     * If it exists, loads configuration.
//!     * If it doesn't exist, uses default configuration.
//! 1. Setting desired (stated in the configuration) video mode (resolution, color map, etc.).
//! 1. Finding and loading suitable kernel binaries (kernel, core/generic drivers, etc.).
//! 1. Taking the ownership from the EFI firmware.
//! 1. Building memory map using EFI's one.
//! 1. Transfering control to the kernel initializer.

#![no_std]
#![cfg_attr(not(doc), no_main)]
#![doc(html_no_source)]

#![feature(panic_info_message)]

mod panic_handling;

use {
	panic_handling::CON_OUT,
	core::sync::atomic::Ordering,
	efi::{
		EfiHandle,
		EfiStatus,
		EfiSystemTable,
		protocols::console::EfiSimpleTextOutputProtocol,
	},
	native::{
		Error,
		features::detection::{
			*,
			state_storing::*,
		},
	},
};

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

/// Equivalent of [`println!`] that appends `[WARN] ` in the beginning of the formatted string.
#[macro_export]
macro_rules! warn {
	($($args:tt)+) => {
		println!("[WARN] {}", format_args!($($args)+));
	}
}

/// Loader's main function.
/// 
/// This function acts as EFI's entry point.
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
			warn!("Clearing screen failed with EFI status: {}", status);
		}
	}

	match enable_detection_mechanism() {
		Ok(FeatureState::Enabled) => log!("Feature detection mechanism enabled."),
		Ok(FeatureState::Disabled) => warn!("Feature detection mechanism couldn't be enabled. It may be required later."),
		Err(Error::Unavailable) => warn!("Feature detection mechanism unavailable. It may be required later."),
		Err(error) => panic!("Error occured while enabling feature detection mechanism!\nError: {:?}", error),
	}

	/* Requirement: State storing mechanism */
	match state_storing_available() {
		Ok(FeatureState::Enabled) => log!("State storing mechanism available."),
		Ok(FeatureState::Disabled) => {
			warn!("State storing mechanism available but is disabled.");
			log!("Enabling state storing mechanism...");
			match enable_detection_mechanism() {
				Ok(FeatureState::Enabled) => log!("State storing mechanism enabled."),
				Ok(FeatureState::Disabled) => panic!("State storing mechanism couldn't be enabled!"),
				Err(error) => panic!("Error occured while enabling feature detection mechanism!\nError: {:?}", error),
			}
		},
		Err(Error::Unavailable) => panic!("State storing mechanism unavailable!"),
		Err(Error::FeatureDisabled) => panic!("Feature detection mechanism required to determine whether state storing is available, but is disabled!"),
		Err(error) => panic!("Error occured while testing for state storing mechanism!\nError: {:?}", error),
	}

	loop {}
}
