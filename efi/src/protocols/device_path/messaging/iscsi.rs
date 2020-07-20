use core::slice::from_raw_parts;

use crate::protocols::device_path::{
	EfiDevicePathProcotol,
	EfiDevicePathRepr,
};

#[repr(C)]
pub struct EfiiScsiDevicePath {
	base: EfiDevicePathProcotol,
	protocol: [u8; 2],
	options: [u8; 2],
	logical_unit_number: [u8; 8],
	target_portal_group_tag: [u8; 2],
	target_name: (),
}

impl EfiiScsiDevicePath {
	pub fn protocol(&self) -> EfiiScsiDevicePathProtocol {
		use EfiiScsiDevicePathProtocol::*;

		let protocol: u16 = unsafe {
			(
				self.protocol.as_ptr() as *const u16
			).read_unaligned()
		};

		match protocol {
			0 => TCP,
			x => Unknown(x),
		}
	}

	pub fn options(&self) -> EfiiScsiDevicePathOptions {
		unsafe {
			(
				self.options.as_ptr() as *const EfiiScsiDevicePathOptions
			).read_unaligned()
		}
	}

	pub fn logical_unit_number(&self) -> [u8; 8] {
		self.logical_unit_number
	}

	pub fn target_portal_group_tag(&self) -> u16 {
		unsafe {
			(
				self.target_portal_group_tag.as_ptr() as *const u16
			).read_unaligned()
		}
	}

	pub fn target_name(&self) -> &[u8] {
		let offset: usize = (&self.target_name as *const () as usize) - (self as *const Self as usize);
		let length: usize;

		length = if self.base.len() as usize - offset <= 223 {
			self.base.len() as usize - offset
		} else {
			223
		};

		unsafe {
			from_raw_parts(
				&self.target_name as *const () as *const u8,
				length
			)
		}
	}
}

impl EfiDevicePathRepr for EfiiScsiDevicePath {}

#[non_exhaustive]
#[derive(Clone,Copy)]
pub enum EfiiScsiDevicePathProtocol {
	TCP,

	Unknown(u16),
}

#[non_exhaustive]
#[derive(Clone,Copy)]
pub enum EfiiScsiDevicePathDigestOption {
	No,
	CRC32C,

	Undefined,
}

#[non_exhaustive]
#[derive(Clone,Copy)]
pub enum EfiiScsiDevicePathAuthenticationMethod {
	None,
	ChapBi,
	ChapUni,

	Undefined,
}

#[repr(transparent)]
#[derive(Clone,Copy)]
pub struct EfiiScsiDevicePathOptions {
	options: u16,
}

impl EfiiScsiDevicePathOptions {
	pub fn header_digest(&self) -> EfiiScsiDevicePathDigestOption {
		use EfiiScsiDevicePathDigestOption::*;

		match self.options & 3 {
			0 => No,
			2 => CRC32C,

			_ => Undefined,
		}
	}

	pub fn data_digest(&self) -> EfiiScsiDevicePathDigestOption {
		use EfiiScsiDevicePathDigestOption::*;

		match self.options & 3 {
			0 => No,
			2 => CRC32C,

			_ => Undefined,
		}
	}

	pub fn authentication_method(&self) -> EfiiScsiDevicePathAuthenticationMethod {
		use EfiiScsiDevicePathAuthenticationMethod::*;

		match (self.options >> 10) & 3 {
			0 => match (self.options >> 12) & 1 {
				0 => ChapBi,
				1 => ChapUni,

				_ => unreachable!(),
			},
			2 => None,

			_ => Undefined,
		}
	}
}

impl From<EfiiScsiDevicePathOptions> for u16 {
	fn from(data: EfiiScsiDevicePathOptions) -> Self {
		data.options
	}
}
