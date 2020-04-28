use crate::{
	guid::EfiGuid,
	status::{
		EfiStatus,
		EfiStatusEnum,
	},
	protocols::EfiProtocol,
};

#[repr(C)]
pub struct EfiSimpleTextOutputProtocol {
	reset: extern "efiapi" fn(*const Self, bool) -> EfiStatus,
	output_string: extern "efiapi" fn(*const Self, *const u16) -> EfiStatus,
	test_string: extern "efiapi" fn(*const Self, *const u16) -> EfiStatus,
	query_mode: extern "efiapi" fn(*const Self, usize, *mut usize, *mut usize) -> EfiStatus,
	set_mode: extern "efiapi" fn(*const Self, usize) -> EfiStatus,
	set_attribute: extern "efiapi" fn(*const Self, usize) -> EfiStatus,
	clear_screen: extern "efiapi" fn(*const Self) -> EfiStatus,
	set_cursor_position: extern "efiapi" fn(*const Self, usize, usize) -> EfiStatus,
	enable_cursor: extern "efiapi" fn(*const Self, bool) -> EfiStatus,
	mode: *const EfiSimpleTextOutputMode,
}

impl EfiSimpleTextOutputProtocol {
	pub fn reset(&self, extended_verification: bool) -> EfiStatusEnum {
		(self.reset)(
			self,
			extended_verification
		).into_enum()
	}

	pub unsafe fn output_string_raw(&self, string: *const u16) -> EfiStatusEnum {
		(self.output_string)(
			self,
			string
		).into_enum()
	}

	pub fn output_string_slice(&self, string: &[u16]) -> EfiStatusEnum {
		(self.output_string)(
			self,
			string.as_ptr()
		).into_enum()
	}

	/* Splits string into parts and executes the function until the first error (and returns it) or until the string is finished (and returns the last returned status) */
	pub fn output_string(&self, mut string: &str) -> EfiStatusEnum {
		let mut status: EfiStatus;

		loop {
			let mut buffer: [u16; 32] = [0; 32];

			let mut index: usize = 0;

			for ch in string.chars() {
				if (buffer.len() - index) < 2 {
					break;
				}
				
				ch.encode_utf16(&mut buffer[index..]);

				index += 1;
				
				if index != buffer.len() {
					if buffer[index] != 0 {
						index += 1;
					}
				}
			}

			status = (self.output_string)(
				self,
				buffer.as_ptr()
			);

			if status.is_error() {
				return status.into_enum();
			}

			string = string.split_at(index).1;

			if string.len() == 0 {
				break;
			}
		}

		status.into_enum()
	}

	pub unsafe fn test_string_raw(&self, string: *const u16) -> EfiStatusEnum {
		(self.test_string)(
			self,
			string
		).into_enum()
	}

	pub fn test_string_slice(&self, string: &[u16]) -> EfiStatusEnum {
		(self.test_string)(
			self,
			string.as_ptr()
		).into_enum()
	}

	/* Splits string into parts and executes the function until the first error (and returns it) or until the string is finished (and returns the last returned status) */
	pub fn test_string(&self, mut string: &str) -> EfiStatusEnum {
		let mut status: EfiStatus;

		loop {
			let mut buffer: [u16; 33] = [0; 33];

			let mut index: usize = 0;

			for ch in string.chars() {
				if (buffer.len() - index) < 2 {
					break;
				}
				
				ch.encode_utf16(&mut buffer[index..]);

				index += 1;
				
				if index != buffer.len() - 1 {
					if buffer[index] != 0 {
						index += 1;
					}
				}
			}

			status = (self.test_string)(
				self,
				buffer.as_ptr()
			);

			if status.is_error() {
				return status.into_enum();
			}

			string = string.split_at(index).1;

			if string.len() == 0 {
				break;
			}
		}

		status.into_enum()
	}

	pub fn query_mode(&self, mode: usize, columns: &mut usize, rows: &mut usize) -> EfiStatusEnum {
		(self.query_mode)(
			self,
			mode,
			columns,
			rows
		).into_enum()
	}

	pub fn set_mode(&self, mode: usize) -> EfiStatusEnum {
		(self.set_mode)(
			self,
			mode
		).into_enum()
	}

	pub fn set_attribute(&self, attribute: usize) -> EfiStatusEnum {
		(self.set_attribute)(
			self,
			attribute
		).into_enum()
	}

	pub fn clear_screen(&self) -> EfiStatusEnum {
		(self.clear_screen)(
			self
		).into_enum()
	}

	pub fn set_cursor_position(&self, column: usize, row: usize) -> EfiStatusEnum {
		(self.set_cursor_position)(
			self,
			column,
			row
		).into_enum()
	}

	pub fn enable_cursor(&self, visible: bool) -> EfiStatusEnum {
		(self.enable_cursor)(
			self,
			visible
		).into_enum()
	}

	pub fn get_mode(&self) -> EfiSimpleTextOutputMode {
		unsafe {
			self.mode.read_unaligned()
		}
	}
}

impl EfiProtocol for EfiSimpleTextOutputProtocol {
	type Interface = Self;

	fn guid() -> EfiGuid {
		EfiGuid::from_tuple((0x387477c2, 0x69c7, 0x11d2, [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b]))
	}
}

#[repr(C)]
pub struct EfiSimpleTextOutputMode {
	max_mode: u32,
	mode: u32,
	attribute: u32,
	cursor_column: u32,
	cursor_row: u32,
	cursor_visible: bool,
}

impl EfiSimpleTextOutputMode {
	pub fn max_mode(&self) -> u32 {
		self.max_mode
	}

	pub fn mode(&self) -> u32 {
		self.mode
	}

	pub fn attribute(&self) -> u32 {
		self.attribute
	}

	pub fn cursor_column(&self) -> u32 {
		self.cursor_column
	}

	pub fn cursor_row(&self) -> u32 {
		self.cursor_row
	}

	pub fn cursor_visible(&self) -> bool {
		self.cursor_visible
	}
}
