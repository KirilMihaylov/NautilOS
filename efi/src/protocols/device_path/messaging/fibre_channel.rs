use crate::protocols::device_path::{
	EfiDevicePathProcotol,
	EfiDevicePathRepr,
};

#[repr(C)]
pub struct EfiFibreChannelDevicePath {
	base: EfiDevicePathProcotol,
	_reserved: [u8; 4],
	world_wide_name: [u8; 8],
	logical_unit_number: [u8; 8],
}

impl<'a> EfiFibreChannelDevicePath {
	pub fn world_wide_name(&'a self) -> &'a [u8] {
		&self.world_wide_name
	}

	pub fn logical_unit_number(&self) -> u64 {
		unsafe {
			(
				self.logical_unit_number.as_ptr() as *const u64
			).read_unaligned()
		}
	}
}

impl EfiDevicePathRepr for EfiFibreChannelDevicePath {}
