use crate::protocols::device_path::{
	EfiDevicePathProcotol,
	EfiDevicePathRepr,
};

#[repr(C)]
pub struct EfiAcpiDevicePath {
	base: EfiDevicePathProcotol,
	_hid: [u8; 4],
	_uid: [u8; 4],
}

impl EfiAcpiDevicePath {
	pub fn _hid(&self) -> u32 {
		unsafe {
			(
				self._hid.as_ptr() as *const u32
			).read_unaligned()
		}
	}

	pub fn _uid(&self) -> u32 {
		unsafe {
			(
				self._uid.as_ptr() as *const u32
			).read_unaligned()
		}
	}
}

impl EfiDevicePathRepr for EfiAcpiDevicePath {}
