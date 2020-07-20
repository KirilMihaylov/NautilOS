use core::slice::from_raw_parts;

use crate::{
	guid::EfiGuid,
	protocols::device_path::{
		EfiDevicePathProcotol,
		EfiDevicePathRepr,
	},
};

#[repr(C)]
pub struct EfiVendorDefinedDevicePath {
	base: EfiDevicePathProcotol,
	vendor_guid: [u8; 16],
	vendor_defined_data: (),
}

impl EfiVendorDefinedDevicePath {
	pub fn vendor_guid(&self) -> EfiGuid {
		unsafe {
			EfiGuid::from_array(&self.vendor_guid)
		}
	}

	pub fn vendor_defined_data(&self) -> &[u8] {
		unsafe {
			from_raw_parts(
				&self.vendor_defined_data as *const () as *const u8,
				(self.base.len() as usize) - 20
			)
		}
	}
}

impl EfiDevicePathRepr for EfiVendorDefinedDevicePath {}
