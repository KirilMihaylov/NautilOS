use core::{
	slice::from_raw_parts,
	mem::size_of,
};

use crate::{
	*,
	protocols::device_path::{
		EfiDevicePathProcotol,
		EfiDevicePathRepr,
	},
};

#[repr(C)]
pub struct EfiVendorDefinedDevicePath {
	base: EfiDevicePathProcotol,
	guid: [u8; size_of::<EfiGuid>()],
	data: (),
}

impl EfiVendorDefinedDevicePath {
	pub fn guid(&self) -> EfiGuid {
		let mut guid: EfiGuidTuple = (0, 0, 0, [0; 8]);
		unsafe {
			let mut ptr: *const u8 = &self.data as *const () as _;
			guid.0 = *(ptr as *const u32);
			ptr = (ptr as *const u32).offset(1) as _;
			guid.1 = *(ptr as *const u16);
			ptr = (ptr as *const u16).offset(1) as _;
			guid.2 = *(ptr as *const u16);
			ptr = (ptr as *const u16).offset(1) as _;
			for i in 0..8 {
				guid.3[i] = *ptr;
				ptr = ptr.offset(1);
			}
		}
		EfiGuid::from_tuple(guid)
	}

	pub fn data(&self) -> &[u8] {
		unsafe {
			from_raw_parts(
				&self.data as *const () as *const u8,
				self.len()
			)
		}
	}

	pub fn len(&self) -> usize {
		self.base.len() as usize - size_of::<Self>()
	}

	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}
}

impl EfiDevicePathRepr for EfiVendorDefinedDevicePath {}
