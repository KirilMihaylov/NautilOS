use crate::{
	*,
	protocols::device_path::{
		EfiDevicePathProcotol,
		EfiDevicePathRepr,
	},
};

#[repr(C)]
pub struct EfiMediaProtocolDevicePath {
	base: EfiDevicePathProcotol,
	protocol_guid: [u8; 16],
}

impl EfiMediaProtocolDevicePath {
	pub fn protocol_guid(&self) -> EfiGuid {
		EfiGuid::from_array(&self.protocol_guid)
	}
}

impl EfiDevicePathRepr for EfiMediaProtocolDevicePath {}
