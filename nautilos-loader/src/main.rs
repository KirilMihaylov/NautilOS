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
#![no_main]
#![doc(html_no_source)]
#![feature(panic_info_message, never_type)]
#![forbid(warnings, missing_docs, clippy::pedantic)]

extern crate alloc;

mod defs;
mod macros;

pub mod panic_handling;

use {
    core::sync::atomic::Ordering,
    efi::{
        boot_services::EfiBootServices, runtime_services::EfiRuntimeServices, EfiHandle, EfiStatus,
        EfiSystemTable,
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
            efi_warn!("Clearing screen failed with error: {:?}", status);
        }
    }

    let boot_services: &mut EfiBootServices = system_table.boot_services_mut();
    let runtime_services: &mut EfiRuntimeServices = system_table.runtime_services_mut();

    stages::start_up(boot_services);

    let boot_device_handle: EfiHandle =
        stages::get_boot_device_handle(boot_services, runtime_services);

    debug_info!(
        "Boot Device Handle: 0x{:0>width$X}",
        boot_device_handle as usize,
        width = core::mem::size_of::<usize>() * 2
    );

    loop {}
}

mod stages {
    use crate::{efi_panic, efi_warn, log, warn};

    pub fn start_up(boot_services: &mut efi::boot_services::EfiBootServices1x0) {
        setup_detection_mechanism();

        setup_state_storing();

        setup_allocator(boot_services);
    }

    fn setup_detection_mechanism() {
        use native::{
            features::detection::{enable, FeatureState},
            Error,
        };

        match enable() {
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
    }

    fn setup_state_storing() {
        use native::{
            features::detection::{
                state_storing::{available, enable},
                FeatureState,
            },
            Error,
        };

        /* Requirement: State storing mechanism */
        match available() {
            Ok(FeatureState::Enabled) => log!("State storing mechanism available."),
            Ok(FeatureState::Disabled) => {
                warn!("State storing mechanism available but is disabled.");
                log!("Enabling state storing mechanism...");
                match enable() {
                    Ok(FeatureState::Enabled) => log!("State storing mechanism enabled."),
                    Ok(FeatureState::Disabled) => panic!("State storing mechanism couldn't be enabled!"),
                    Err(error) => panic!("Error occured while enabling feature detection mechanism!\nError: {:?}", error),
                }
            },
            Err(Error::Unavailable) => panic!("State storing mechanism unavailable!"),
            Err(Error::FeatureDisabled) => panic!("Feature detection mechanism required to determine whether state storing is available, but is disabled!"),
            Err(error) => panic!("Error occured while testing for state storing mechanism!\nError: {:?}", error),
        }
    }

    fn setup_allocator(boot_services: &efi::boot_services::EfiBootServices1x0) {
        use {
            crate::defs::OsMemoryType,
            core::slice::from_raw_parts_mut,
            efi::{
                boot_services::types::memory::EfiMemoryType, EfiPhysicalAddress, EfiStatusError,
                EfiStatusWarning, VoidPtr,
            },
            nautilos_allocator::{initialize as initialize_allocator, Heap},
        };

        const REQUIRED_HEAP_SIZE: usize = Heap::UNALIGNED_REQUIRED_INITIAL_SIZE;
        const ADDITIONAL_HEAP_SIZE: usize = 0x0010_0000; // 1 MB
        const TOTAL_HEAP_SIZE: usize = REQUIRED_HEAP_SIZE + ADDITIONAL_HEAP_SIZE;

        let address: EfiPhysicalAddress;

        let result: Result<(Option<EfiStatusWarning>, VoidPtr), (EfiStatusError, ())> =
            boot_services
                .allocate_pool(
                    EfiMemoryType::custom(OsMemoryType::LoaderHeap.into()),
                    TOTAL_HEAP_SIZE,
                )
                .unfold();

        match result {
            Ok((status, allocation_address)) => {
                if let Some(status) = status {
                    efi_warn!(
                        "Warning status returned while allocating memory pages.\tWarning: {:?}",
                        status
                    );
                }

                address = allocation_address as u64;
            }
            Err((status, ())) => efi_panic!(
                "Error occured while retrieving disk I/O device handles!\nError: {:?}",
                status
            ),
        }

        if let Err(error) =
            initialize_allocator(unsafe { from_raw_parts_mut(address as *mut u8, TOTAL_HEAP_SIZE) })
        {
            panic!("Error occured while initializing heap!\nError: {:?}", error);
        }
    }

    pub fn get_boot_device_handle(
        boot_services: &efi::boot_services::EfiBootServices1x0,
        runtime_services: &efi::runtime_services::EfiRuntimeServices,
    ) -> efi::EfiHandle {
        use {
            alloc::vec::Vec,
            efi::{
                guids::{EFI_DISK_IO_PROTOCOL, EFI_GLOBAL_VARIABLE},
                protocols::{device_path::EfiDevicePathProtocolRaw, EfiProtocol},
                structures::load_option::EfiLoadOption,
                EfiStatusEnum, EfiStatusError, NonNullVoidPtr, VoidMutPtr,
            },
            utf16_utils::{macros::c_utf16, ArrayEncoder},
        };

        let boot_device_number: &mut [u8] = &mut [0; 2];

        match runtime_services.revision_1_0().get_variable(
            &c_utf16!("BootCurrent\0"),
            &EFI_GLOBAL_VARIABLE,
            Some(boot_device_number),
        ) {
            EfiStatusEnum::Success(_) => (),
            EfiStatusEnum::Warning(status, _) => efi_warn!(
                "Reading \"BootCurrent\" returned with warning status: {:?}",
                status
            ),
            EfiStatusEnum::Error(status, _) => {
                efi_panic!("Couldn't read \"BootCurrent\" variable with: {:?}!", status)
            }
        }

        let boot_xxxx: &mut [u16] = &mut [0; 9];

        ArrayEncoder::new(&mut boot_xxxx[..8])
            .write_formatted(format_args!(
                "Boot{:0>2X}{:0>2X}",
                boot_device_number[1], boot_device_number[0],
            ))
            .expect("Internal error occured while formatting boot device's variable name!");

        let mut variable_data: Vec<u8> = Vec::new();

        for z in 0..2 {
            match runtime_services
                .revision_1_0()
                .get_variable(
                    boot_xxxx,
                    &EFI_GLOBAL_VARIABLE,
                    if z == 0 {
                        None
                    } else {
                        Some(&mut variable_data)
                    },
                )
                .unfold()
            {
                Ok((_, (length, _))) => variable_data.truncate(length),
                Err((EfiStatusError::EfiBufferTooSmall, (length, _))) if z == 0 => {
                    variable_data.resize(length, 0)
                }
                Err((status, _)) => efi_panic!(
                    "Couldn't read \"Boot{:0>2X}{:0>2X}\" variable with: {:?}!",
                    boot_device_number[1],
                    boot_device_number[0],
                    status
                ),
            }
        }

        if let Some(load_option) = EfiLoadOption::parse(&variable_data) {
            if let Ok(mut device_path) = unsafe {
                let ptr: VoidMutPtr = load_option.file_path_list().as_ptr() as VoidMutPtr;
                let ptr: NonNullVoidPtr =
                    NonNullVoidPtr::new(ptr).expect("Internal error occured!");
                EfiDevicePathProtocolRaw::parse(ptr)
            } {
                match boot_services.locate_device_path(&EFI_DISK_IO_PROTOCOL, &mut device_path) {
                    EfiStatusEnum::Success(handle) => handle,
                    EfiStatusEnum::Warning(status, handle) => {
                        efi_warn!(
                            "Retrieving handle from device path returned with warning status: {:?}",
                            status
                        );

                        handle
                    }
                    EfiStatusEnum::Error(status, _) => efi_panic!(
                        "Error occured while retrieving handle from device path! Error: {:?}",
                        status
                    ),
                }
            } else {
                unreachable!();
            }
        } else {
            efi_panic!("Error occured while parsing the boot device's load option!");
        }
    }
}
