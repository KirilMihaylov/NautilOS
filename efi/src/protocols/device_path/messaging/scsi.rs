use crate::protocols::device_path::{
	EfiDevicePathProcotol,
	EfiDevicePathInto,
};

#[repr(C)]
pub struct EfiScsiDevicePath {
	base: EfiDevicePathProcotol,
	target_id: [u8; 2],
	logical_unit_number: [u8; 2],
}

impl EfiScsiDevicePath {
	pub fn target_id(&self) -> u16 {
		unsafe {
			(
				self.target_id.as_ptr() as *const u16
			).read_unaligned()
		}
	}

	pub fn logical_unit_number(&self) -> u16 {
		unsafe {
			(
				self.logical_unit_number.as_ptr() as *const u16
			).read_unaligned()
		}
	}
}

impl EfiDevicePathInto<EfiScsiDevicePath> for EfiScsiDevicePath {}
