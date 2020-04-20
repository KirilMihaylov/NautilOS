use core::slice::from_raw_parts;

use crate::protocols::device_path::{
	EfiDevicePathProcotol,
	EfiDevicePathInto,
};

#[repr(C)]
pub struct EfiAddressDevicePath {
	base: EfiDevicePathProcotol,
	first_address: [u8; 4], /* u32 */
}

impl EfiAddressDevicePath {
	pub fn addresses<'a>(&'a self) -> &'a [u32] {
		unsafe {
			from_raw_parts(
				self.first_address.as_ptr() as *const u32,
				(self.base.len() / 4) as usize
			)
		}
	}
}

impl EfiDevicePathInto<EfiAddressDevicePath> for EfiAddressDevicePath {}
