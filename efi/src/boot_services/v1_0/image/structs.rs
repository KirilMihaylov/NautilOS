use core::slice::from_raw_parts;

use crate::{
	types::{
		EfiHandle,
		VoidPtr,
	},
	status::{
		EfiStatus,
		EfiStatusEnum,
	}
};
use crate::protocols::device_path::EfiDevicePathProcotol;

#[repr(C)]
pub struct EfiImage {
	load_image: extern "efiapi" fn(bool, EfiHandle, *const EfiDevicePathProcotol, VoidPtr, usize, *const EfiHandle) -> EfiStatus,
	start_image: extern "efiapi" fn(EfiHandle, *mut usize, *mut *const u16) -> EfiStatus,
	exit: extern "efiapi" fn(EfiHandle, EfiStatus, usize, *const u16) -> EfiStatus,
	unload_image: extern "efiapi" fn(EfiHandle) -> EfiStatus,
	exit_boot_services: extern "efiapi" fn(EfiHandle, usize) -> EfiStatus,
}

impl EfiImage {
	pub fn load_image(&self, boot_policy: bool, parent_image_handle: EfiHandle, device_path: &EfiDevicePathProcotol, source_buffer: Option<&[u8]>) -> EfiStatusEnum<EfiHandle> {
		let mut image_handle: EfiHandle = 0 as _;

		let (source_buffer_ptr, source_buffer_len): (VoidPtr, usize) = if let Some(source_buffer) = source_buffer {
			(source_buffer.as_ptr() as _, source_buffer.len())
		} else {
			(0 as _, 0)
		};

		(self.load_image)(
			boot_policy,
			parent_image_handle,
			device_path,
			source_buffer_ptr,
			source_buffer_len,
			&mut image_handle
		).into_enum_data(image_handle)
	}

	pub fn start_image(&self, image_handle: EfiHandle) -> EfiStatusEnum<(&[u16], &[u8])> {
		let (mut exit_data, mut exit_data_size): (*const u16, usize) = (0 as _, 0);

		let efi_status: EfiStatus = (self.start_image)(
			image_handle,
			&mut exit_data_size,
			&mut exit_data
		);

		let (utf_16_data, raw_data): (&[u16], &[u8]);

		if exit_data.is_null() {
			utf_16_data = &[];
			raw_data = &[];
		} else {
			let utf_16_data_length: usize = {
				let (mut ptr, mut utf_16_data_length): (*const u16, usize) = (exit_data, 2);
				loop {
					if exit_data_size <= utf_16_data_length {
						unsafe {
							if *ptr == 0 {
								break utf_16_data_length;
							} else {
								ptr = ptr.offset(1);
								utf_16_data_length += 2;
							}
						}
					} else {
						break 0;
					}
				}
			};
			utf_16_data = unsafe {
				from_raw_parts(
					exit_data,
					utf_16_data_length / 2
				)
			};
			raw_data = unsafe {
				from_raw_parts(
					exit_data as _,
					exit_data_size - utf_16_data_length
				)
			};
		}

		efi_status.into_enum_data(
			(
				utf_16_data,
				raw_data
			)
		)
	}

	pub fn exit(&self, image_handle: EfiHandle, exit_status: EfiStatus, exit_data: Option<&[u16]>) -> EfiStatusEnum {
		let (exit_data_ptr, exit_data_len): (*const u16, usize) = if let Some(exit_data) = exit_data {
			(exit_data.as_ptr(), exit_data.len() * 2)
		} else {
			(0 as _, 0)
		};

		(self.exit)(
			image_handle,
			exit_status,
			exit_data_len,
			exit_data_ptr
		).into_enum()
	}

	pub fn unload_image(&self, image_handle: EfiHandle) -> EfiStatusEnum {
		(self.unload_image)(
			image_handle
		).into_enum()
	}

	pub fn exit_boot_services(&self, image_handle: EfiHandle, memory_map_key: usize) -> EfiStatusEnum {
		(self.exit_boot_services)(
			image_handle,
			memory_map_key
		).into_enum()
	}
}
