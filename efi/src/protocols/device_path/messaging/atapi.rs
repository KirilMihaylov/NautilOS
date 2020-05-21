use crate::protocols::device_path::{
	EfiDevicePathProcotol,
	EfiDevicePathRepr,
};

#[derive(Clone,Copy)]
#[repr(u8)]
#[non_exhaustive]
pub enum EfiAtapiDevicePathTypeMode {
	PrimaryMaster,
	PrimarySlave,
	SecondaryMaster,
	SecondarySlave,
}

#[repr(C)]
pub struct EfiAtapiDevicePath {
	base: EfiDevicePathProcotol,
	primary_seconary: u8,
	master_slave: u8,
	logical_unit_number: [u8; 2],
}

impl EfiAtapiDevicePath {
	pub fn type_mode(&self) -> EfiAtapiDevicePathTypeMode {
		use EfiAtapiDevicePathTypeMode::*;

		match (self.primary_seconary, self.master_slave) {
			(0, 0) => PrimaryMaster,
			(0, _) => PrimarySlave,
			(_, 0) => SecondaryMaster,
			(_, _) => SecondarySlave,
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

impl EfiDevicePathRepr for EfiAtapiDevicePath {}
