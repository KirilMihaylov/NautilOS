use crate::protocols::device_path::{
	EfiDevicePathProcotol,
	EfiDevicePathInto,
};

#[repr(C)]
pub struct EfiSerialAttachedScsiExDevicePath {
	base: EfiDevicePathProcotol,
	vendor_guid: [u8; 16],
	_reserved: [u8; 4],
	sas_address: [u8; 8],
	logical_unit_number: [u8; 8],
	device_and_topology_info: [u8; 2],
	relative_target_port: [u8; 2],
}

impl EfiSerialAttachedScsiExDevicePath {
	pub fn sas_address(&self) -> [u8; 8] {
		self.sas_address
	}

	pub fn logical_unit_number(&self) -> [u8; 8] {
		self.logical_unit_number
	}

	pub fn device_and_topology_info(&self) -> EfiSerialAttachedScsiExDevicePathDeviceAndTopologyInfo {
		unsafe {
			(
				self.sas_address.as_ptr() as *const EfiSerialAttachedScsiExDevicePathDeviceAndTopologyInfo
			).read_unaligned()
		}
	}

	pub fn relative_target_port(&self) -> u16 {
		unsafe {
			(
				self.relative_target_port.as_ptr() as *const u16
			).read_unaligned()
		}
	}
}

impl EfiDevicePathInto<EfiSerialAttachedScsiExDevicePath> for EfiSerialAttachedScsiExDevicePath {}

#[non_exhaustive]
#[derive(Clone,Copy)]
pub enum EfiSerialAttachedScsiExDevicePathAdditionalInfoBytes {
	NoBytes,
	OneByte,
	TwoBytes,
}

#[non_exhaustive]
#[derive(Clone,Copy)]
pub enum EfiSerialAttachedScsiExDevicePathDeviceType {
	SAS {
		internal: bool,
	},
	SATA {
		internal: bool,
	},
}

#[non_exhaustive]
#[derive(Clone,Copy)]
pub enum EfiSerialAttachedScsiExDevicePathTopology {
	DirectConnect,
	ExpanderConnect,
}

#[repr(transparent)]
#[derive(Clone,Copy)]
pub struct EfiSerialAttachedScsiExDevicePathDeviceAndTopologyInfo {
	device_and_topology_info: u16,
}

impl EfiSerialAttachedScsiExDevicePathDeviceAndTopologyInfo {
	fn additional_info_bytes(&self) -> Result<EfiSerialAttachedScsiExDevicePathAdditionalInfoBytes, u16> {
		use EfiSerialAttachedScsiExDevicePathAdditionalInfoBytes::*;

		Ok(
			match self.device_and_topology_info & 0xF {
				0 => NoBytes,
				1 => OneByte,
				2 => TwoBytes,
				_ => return Err(self.device_and_topology_info), /* Overrides Ok */
			}
		)
	}

	pub fn device_type(&self) -> Result<EfiSerialAttachedScsiExDevicePathDeviceType, u16> {
		use EfiSerialAttachedScsiExDevicePathAdditionalInfoBytes::*;
		use EfiSerialAttachedScsiExDevicePathDeviceType::*;

		Ok(
			match self.additional_info_bytes()? {
				/* Specification defines field as valid when "additional_info_bytes" is non-zero */
				NoBytes => return Err(self.device_and_topology_info),
				_ => match (self.device_and_topology_info >> 4) & 3 {
					0 => SAS {
						internal: true,
					},
					1 => SATA {
						internal: true,
					},
					2 => SAS {
						internal: false,
					},
					3 => SATA {
						internal: false,
					},
					_ => unreachable!(),
				},
			}
		)
	}

	pub fn topology(&self) -> Result<EfiSerialAttachedScsiExDevicePathTopology, u16> {
		use EfiSerialAttachedScsiExDevicePathAdditionalInfoBytes::*;
		use EfiSerialAttachedScsiExDevicePathTopology::*;

		Ok(
			match self.additional_info_bytes()? {
				/* Specification defines field as valid when "additional_info_bytes" is non-zero */
				NoBytes => return Err(self.device_and_topology_info),
				_ => match (self.device_and_topology_info >> 6) & 3 {
					0 => DirectConnect,
					1 => ExpanderConnect,
					_ => return Err(self.device_and_topology_info),
				},
			}
		)
	}

	pub fn internal_drive_id(&self) -> Result<u16, u16> {
		use EfiSerialAttachedScsiExDevicePathAdditionalInfoBytes::*;
		use EfiSerialAttachedScsiExDevicePathDeviceType::*;

		match self.additional_info_bytes()? {
			TwoBytes => match self.device_type()? {
				SAS { internal } | SATA { internal } if internal => (),
				_ => return Err(self.device_and_topology_info),
			},
			_ => return Err(self.device_and_topology_info),
		}

		Ok(self.device_and_topology_info >> 8)
	}
}
