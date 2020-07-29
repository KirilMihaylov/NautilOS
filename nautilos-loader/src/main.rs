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

mod macros;

use {
    core::{mem::size_of, slice::from_raw_parts_mut, sync::atomic::Ordering},
    efi::{
        boot_services::{
            memory::EfiMemoryType,
            protocol_handler::{EfiLocateSearchType, EfiProtocolBinding},
        },
        protocols::{
            console::EfiSimpleTextOutputProtocol,
            media::{EfiBlockIOProtocol, EfiDiskIOProtocol},
            EfiProtocol,
        },
        EfiHandle, EfiStatus, EfiStatusEnum, EfiSystemTable,
    },
    native::{
        features::detection::{state_storing::*, *},
        Error,
    },
    panic_handling::CON_OUT,
};

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
    if let Some(con_out) = system_table.con_out_mut() {
        CON_OUT.store(con_out, Ordering::Relaxed);

        if let efi::EfiStatusEnum::Error(status, _) = con_out.clear_screen() {
            efi_warn!(
                "Clearing screen failed with error: {:?}",
                EfiStatus::from(status).get_error()
            );
        }
    }

    match enable_detection_mechanism() {
        Ok(FeatureState::Enabled) => log!("Feature detection mechanism enabled."),
        Ok(FeatureState::Disabled) => {
            warn!("Feature detection mechanism couldn't be enabled. It may be required later.")
        }
        Err(Error::Unavailable) => {
            warn!("Feature detection mechanism unavailable. It may be required later.")
        }
        Err(error) => panic!(
            "Error occured while enabling feature detection mechanism!\nError: {:?}",
            error
        ),
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

    let boot_services = system_table.boot_services_mut().revision_1_0_mut();

    let (mut handles_slice, mut handles_buffer): (&[EfiHandle], &mut [EfiHandle]);
    handles_slice = &[];

    {
        const MAX_ATTEMPT_COUNT: u8 = 2;

        let mut required_size: usize = 32 * size_of::<EfiHandle>();

        for attempt in 1..=MAX_ATTEMPT_COUNT {
            handles_buffer =
                match boot_services.allocate_pool(EfiMemoryType::custom(0), required_size) {
                    EfiStatusEnum::Success(ptr) => unsafe {
                        from_raw_parts_mut(
                            ptr as *mut EfiHandle,
                            required_size / size_of::<EfiHandle>(),
                        )
                    },
                    EfiStatusEnum::Warning(status, ptr) => {
                        warn!(
                            "(EFI) Warning occured while allocating memory.\tWarning: {:?}",
                            EfiStatus::from(status).get_warning()
                        );

                        unsafe {
                            from_raw_parts_mut(
                                ptr as *mut EfiHandle,
                                required_size / size_of::<EfiHandle>(),
                            )
                        }
                    }
                    EfiStatusEnum::Error(status, _) => {
                        panic!(
                            "(EFI) Error occured while allocating memory!\nError: {:?}",
                            EfiStatus::from(status).get_error()
                        );
                    }
                };

            let result: EfiStatusEnum<&[EfiHandle], usize> = boot_services.locate_handle(
                EfiLocateSearchType::ByProtocol,
                Some(&EfiDiskIOProtocol::guid()),
                None,
                handles_buffer,
            );

            if let EfiStatusEnum::Warning(status, _) = result {
                efi_warn!(
					"Warning status returned while retrieving disk I/O device handles.\tWarning: {:?}",
					EfiStatus::from(status).get_warning()
				);
            }

            match result {
                EfiStatusEnum::Success(handles) | EfiStatusEnum::Warning(_, handles) => {
                    handles_slice = handles;

                    break;
                }
                EfiStatusEnum::Error(status, required_bytes) => {
                    required_size = required_bytes;

                    handles_slice = &[];

                    efi_assert!(
                        !boot_services
                            .free_pool(handles_buffer.as_ptr() as efi::VoidPtr)
                            .map_result()
                            .is_err(),
                        "Error occured while freeing memory pool!"
                    );

                    efi_assert!(
                        attempt != MAX_ATTEMPT_COUNT,
                        "Error occured while retrieving disk I/O device handles!\nError: {:?}",
                        EfiStatus::from(status).get_error()
                    );
                }
            };
        }
    }

    efi_assert!(
        !handles_slice.is_empty(),
        "No devices that implement disk I/O protocol found!"
    );

    handles_slice.iter().for_each(|&handle| {
        debug_info!("Handle pointer: {:?}", handle);

        let result: EfiStatusEnum<EfiProtocolBinding> =
            boot_services.handle_protocol(handles_slice[0], &EfiBlockIOProtocol::guid());

        if let EfiStatusEnum::Warning(status, _) = result {
            efi_warn!(
                "Warning status returned while getting block I/O protocol.\tWarning: {:?}",
                EfiStatus::from(status).get_warning()
            );
        }

        match result {
            EfiStatusEnum::Success(block_io_binding)
            | EfiStatusEnum::Warning(_, block_io_binding) => {
                let block_io: &EfiBlockIOProtocol =
                    block_io_binding.resolve().expect("Internal error occured!");

                {
                    let media: &dyn efi::protocols::media::EfiBlockIOMediaRevision1 =
                        block_io.media_revision_1();

                    debug_info!("Media:\n{:?}", media);
                }
            }
            EfiStatusEnum::Error(status, _) => efi_panic!(
                "Error occured while getting block I/O protocol.\nError: {:?}",
                EfiStatus::from(status).get_error()
            ),
        }
    });

    loop {}
}
