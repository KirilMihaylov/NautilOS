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
		if let Some(guid) = EfiGuid::from_buffer(&self.protocol_guid) {
			guid
		} else {
			unreachable!("GUID must be valid!");
		}
	}
}

impl EfiDevicePathRepr for EfiMediaProtocolDevicePath {}
