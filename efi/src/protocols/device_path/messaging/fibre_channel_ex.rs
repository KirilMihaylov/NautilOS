use crate::protocols::device_path::{
	EfiDevicePathProcotol,
	EfiDevicePathInto,
};

#[repr(C)]
pub struct EfiFibreChannelExDevicePath {
	base: EfiDevicePathProcotol,
	world_wide_name: [u8; 8],
	logical_unit_number: [u8; 8],
}

impl<'a> EfiFibreChannelExDevicePath {
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

impl EfiDevicePathInto<EfiFibreChannelExDevicePath> for EfiFibreChannelExDevicePath {}
