use crate::protocols::device_path::{
	EfiDevicePathProcotol,
	EfiDevicePathInto,
};

#[repr(C)]
pub struct EfiRestServiceDevicePath {
	base: EfiDevicePathProcotol,
	service_type: u8,
	access_mode: u8,
}

impl EfiRestServiceDevicePath {
	pub fn service_type(&self) -> EfiRestServiceDevicePathServiceType {
		use EfiRestServiceDevicePathServiceType::*;

		match self.service_type {
			1 => Redfish,
			2 => OData,

			x => Unknown(x),
		}
	}

	pub fn address_type(&self) -> EfiRestServiceDevicePathAccessMode {
		use EfiRestServiceDevicePathAccessMode::*;

		match self.access_mode {
			1 => InBand,
			2 => OutOfBand,

			x => Unknown(x),
		}
	}
}

impl EfiDevicePathInto<EfiRestServiceDevicePath> for EfiRestServiceDevicePath {}

#[non_exhaustive]
#[derive(Clone,Copy)]
pub enum EfiRestServiceDevicePathServiceType {
	Redfish,
	OData,

	Unknown(u8),
}

#[non_exhaustive]
#[derive(Clone,Copy)]
pub enum EfiRestServiceDevicePathAccessMode {
	InBand,
	OutOfBand,

	Unknown(u8),
}
