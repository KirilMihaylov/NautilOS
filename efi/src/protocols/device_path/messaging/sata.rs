use crate::protocols::device_path::{
	EfiDevicePathProcotol,
	EfiDevicePathInto,
};

#[repr(C)]
pub struct EfiSataDevicePath {
	base: EfiDevicePathProcotol,
	hba_port_number: [u8; 2],
	port_multiplier_port_number: [u8; 2],
	logical_unit_number: [u8; 2],
}

impl EfiSataDevicePath {
	pub fn hba_port_number(&self) -> u16 {
		unsafe {
			(
				self.hba_port_number.as_ptr() as *const  u16
			).read_unaligned()
		}
	}

	pub fn port_multiplier_port_number(&self) -> u16 {
		unsafe {
			(
				self.port_multiplier_port_number.as_ptr() as *const  u16
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

impl EfiDevicePathInto<EfiSataDevicePath> for EfiSataDevicePath {}
