use crate::types::EfiEvent;
use crate::guid::EfiGuid;
use crate::status::{
	EfiStatus,
	EfiStatusEnum,
};
use crate::protocols::EfiProtocol;

#[repr(C)]
pub struct EfiSimpleTextInputProtocol {
	reset: extern "efiapi" fn(*const Self, extended_verification: bool) -> EfiStatus,
	read_key_stroke: extern "efiapi" fn(*const Self, key: *mut EfiInputKey) -> EfiStatus,
	wait_for_key: EfiEvent,
}

impl EfiSimpleTextInputProtocol {
	pub fn reset(&self, extended_verification: bool) -> EfiStatusEnum {
		(self.reset)(
			self,
			extended_verification
		).into_enum()
	}

	pub fn read_key_stroke(&self) -> EfiStatusEnum<EfiInputKey> {
		let mut key: EfiInputKey = EfiInputKey {
			scan_code: 0,
			unicode_char: 0,
		};
		
		(self.read_key_stroke)(
			self,
			&mut key
		).into_enum_data(key)
	}

	pub fn wait_for_key(&self) -> EfiEvent {
		self.wait_for_key
	}

	pub fn wait_for_key_mut(&self) -> &mut EfiEvent {
		unsafe {
			&mut*(&self.wait_for_key as *const EfiEvent as *mut EfiEvent)
		}
	}
}

impl EfiProtocol for EfiSimpleTextInputProtocol {
	type Interface = Self;

	fn guid() -> EfiGuid {
		EfiGuid::from_tuple((0x387477c1, 0x69c7, 0x11d2, [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b]))
	}
}

#[repr(C)]
pub struct EfiInputKey {
	scan_code: u16,
	unicode_char: u16,
}
