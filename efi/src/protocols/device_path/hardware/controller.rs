use crate::protocols::device_path::{ EfiDevicePathProcotol, EfiDevicePathInto };

#[repr(C)]
pub struct EfiControllerDevicePath {
	base: EfiDevicePathProcotol,
	controller_number: [u8; 4],
}

impl EfiControllerDevicePath {
	pub fn controller_number(&self) -> u32 {
		unsafe {
			(
				self.controller_number.as_ptr() as *const u32
			).read_unaligned()
		}
	}
}

impl EfiDevicePathInto<EfiControllerDevicePath> for EfiControllerDevicePath {}
