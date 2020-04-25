#[repr(transparent)]
#[derive(Clone,Copy)]
pub struct NVDIMMDeviceHandle {
	handle: u32,
}

impl NVDIMMDeviceHandle {
	pub fn dimm_number(&self) -> u8 {
		(self.handle & 15) as u8
	}

	pub fn channel_number(&self) -> u8 {
		((self.handle >> 4) & 15) as u8
	}

	pub fn controller_id(&self) -> u8 {
		((self.handle >> 8) & 15) as u8
	}

	pub fn socket_id(&self) -> u8 {
		((self.handle >> 12) & 15) as u8
	}

	pub fn node_controller_id(&self) -> u16 {
		((self.handle >> 16) & 4095) as u16
	}
}

#[cfg(feature="efi_interops")]
use efi_interops::{
	EfiObject,
	traits::{
		acpi::device::NvdimmDeviceHandle,
		conversion::FromEfiObject,
	},
};

#[cfg(feature="efi_interops")]
unsafe impl FromEfiObject for NVDIMMDeviceHandle {
	fn convert(object: EfiObject) -> Option<Self> {
		use core::{
			mem::size_of,
			slice::from_raw_parts_mut,
		};

		if object.len() != size_of::<Self>() {
			None
		} else {
			let mut new_self: Self = Self {
				handle: 0,
			};
			
			object.copy_over(
				unsafe {
					from_raw_parts_mut(
						&mut new_self as *mut Self as *mut u8,
						size_of::<Self>()
					)
				}
			);
			
			Some(new_self)
		}
	}
}

#[cfg(feature="efi_interops")]
unsafe impl NvdimmDeviceHandle for NVDIMMDeviceHandle {}
