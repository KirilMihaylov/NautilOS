use crate::types::EfiGuidTuple;

#[repr(C)]
#[derive(PartialEq,Eq,Clone,Copy)]
pub struct EfiGuid {
	data_1: u32,
	data_2: u16,
	data_3: u16,
	data_4: [u8; 8]
}

impl EfiGuid {
	pub const fn as_tuple(&self) -> EfiGuidTuple {
		(self.data_1, self.data_2, self.data_3, self.data_4)
	}

	pub const fn from_tuple(data: EfiGuidTuple) -> Self {
		Self {
			data_1: data.0,
			data_2: data.1,
			data_3: data.2,
			data_4: data.3,
		}
	}

	pub fn from_array(data: &[u8; 16]) -> Self {
		unsafe {
			core::mem::transmute(data)
		}
	}

	pub fn from_slice(data: &[u8]) -> Option<Self> {
		if data.len() < 16 {
			None
		} else {
			unsafe {
				Some(
					Self {
						data_1: (data.as_ptr() as *const u32).read_unaligned(),
						data_2: (data.as_ptr().offset(4) as *const u16).read_unaligned(),
						data_3: (data.as_ptr().offset(6) as *const u16).read_unaligned(),
						data_4: (data.as_ptr().offset(8) as *const [u8; 8]).read_unaligned(),
					}
				)
			}
		}
	}
}

impl PartialEq<&EfiGuid> for EfiGuid {
	fn eq(&self, other: &&EfiGuid) -> bool {
		*self == **other
	}
}

impl PartialEq<EfiGuid> for &EfiGuid {
	fn eq(&self, other: &EfiGuid) -> bool {
		**self == *other
	}
}

impl PartialEq<EfiGuidTuple> for EfiGuid {
	fn eq(&self, other: &EfiGuidTuple) -> bool {
		self.as_tuple() == *other
	}
}

impl PartialEq<&EfiGuidTuple> for EfiGuid {
	fn eq(&self, other: &&EfiGuidTuple) -> bool {
		self.as_tuple() == **other
	}
}

impl PartialEq<EfiGuidTuple> for &EfiGuid {
	fn eq(&self, other: &EfiGuidTuple) -> bool {
		self.as_tuple() == *other
	}
}

impl PartialEq<EfiGuid> for EfiGuidTuple {
	fn eq(&self, other: &EfiGuid) -> bool {
		*self == other.as_tuple()
	}
}

impl PartialEq<&EfiGuid> for EfiGuidTuple {
	fn eq(&self, other: &&EfiGuid) -> bool {
		*self == other.as_tuple()
	}
}

impl PartialEq<EfiGuid> for &EfiGuidTuple {
	fn eq(&self, other: &EfiGuid) -> bool {
		**self == other.as_tuple()
	}
}

impl From<EfiGuidTuple> for EfiGuid {
	fn from(data: (u32, u16, u16, [u8; 8])) -> Self {
		Self {
			data_1: data.0,
			data_2: data.1,
			data_3: data.2,
			data_4: data.3,
		}
	}
}

impl From<&EfiGuidTuple> for EfiGuid {
	fn from(data: &EfiGuidTuple) -> Self {
		Self {
			data_1: data.0,
			data_2: data.1,
			data_3: data.2,
			data_4: data.3,
		}
	}
}

impl From<EfiGuid> for EfiGuidTuple {
	fn from(guid: EfiGuid) -> Self {
		(guid.data_1, guid.data_2, guid.data_3, guid.data_4)
	}
}

impl From<&EfiGuid> for EfiGuidTuple {
	fn from(guid: &EfiGuid) -> Self {
		(guid.data_1, guid.data_2, guid.data_3, guid.data_4)
	}
}
