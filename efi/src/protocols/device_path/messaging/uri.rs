use core::slice::from_raw_parts;

use crate::protocols::device_path::{
	EfiDevicePathProcotol,
	EfiDevicePathRepr,
};

#[repr(C)]
pub struct EfiUniversalResourceIdentifierDevicePath {
	base: EfiDevicePathProcotol,
	data: (),
}

impl EfiUniversalResourceIdentifierDevicePath {
	pub fn data<'a>(&'a self) -> &'a [u8] {
		unsafe {
			from_raw_parts(
				&self.data as *const () as *const u8,
				self.base.len() as usize - 4
			)
		}
	}
}

impl EfiDevicePathRepr for EfiUniversalResourceIdentifierDevicePath {}
