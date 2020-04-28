use crate::protocols::device_path::{
	EfiDevicePathProcotol,
	EfiDevicePathInto,
};

#[repr(C)]
pub struct EfiI2ODevicePath {
	base: EfiDevicePathProcotol,
	target_id: [u8; 4],
}

impl EfiI2ODevicePath {
	pub fn target_id(&self) -> u32 {
		unsafe {
			(
				self.target_id.as_ptr() as *const  u32
			).read_unaligned()
		}
	}
}

impl EfiDevicePathInto<EfiI2ODevicePath> for EfiI2ODevicePath {}