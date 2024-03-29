use {
    crate::{
        guid::EfiGuid,
        protocols::EfiProtocol,
        status::{EfiStatus, EfiStatusEnum},
        types::NonNullVoidPtr,
    },
    core::fmt::{Error, Write},
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
        (self.reset)(self, extended_verification).into_enum()
    }

    /// Prints UTF-16 encoded string to the screen.
    /// # Safety
    /// The caller must ensure that the pointer will not be pointing invalid memory, is null-terminated and will only contain valid UTF-16 characters until is null-terminated.
    ///
    /// If those conditions are violated, this will result in Undefined Behaviour.
    pub unsafe fn output_string_raw(&self, string: *const u16) -> EfiStatusEnum {
        (self.output_string)(self, string).into_enum()
    }

    /// Prints UTF-16 encoded string to the screen.
    /// # Safety
    /// The caller must ensure that the string is null-terminated and will only contain valid UTF-16 characters until is null-terminated.
    ///
    /// If those conditions are violated, this will result in Undefined Behaviour.
    pub unsafe fn output_string_slice(&self, string: &[u16]) -> EfiStatusEnum {
        (self.output_string)(self, string.as_ptr()).into_enum()
    }

    /// Splits string into parts and prints them until the first error (and returns it) or until the string is finished (and returns the last returned status)
    pub fn output_string(&self, mut string: &str) -> EfiStatusEnum {
        let mut status: EfiStatus;

        loop {
            const BUFFER_LEN: usize = 64;

            let mut buffer: [u16; BUFFER_LEN] = [0; BUFFER_LEN];

            let mut char_count: usize = 0;

            let mut index: usize = 0;

            for ch in string.chars() {
                if ch == '\n' {
                    if index + 2 >= BUFFER_LEN {
                        break;
                    }

                    '\n'.encode_utf16(&mut buffer[index..]);

                    '\r'.encode_utf16(&mut buffer[(index + 1)..]);

                    index += 2;

                    char_count += 1;

                    continue;
                }

                let char_utf16_len = ch.len_utf16();

                if index + char_utf16_len >= BUFFER_LEN {
                    break;
                }

                char_count += 1;

                ch.encode_utf16(&mut buffer[index..]);

                index += char_utf16_len;
            }

            status = (self.output_string)(self, buffer.as_ptr());

            if status.is_error() {
                return status.into_enum();
            }

            // string = string.split_at(index).1;

            string = &string[char_count..];

            if string.is_empty() {
                break;
            }
        }

        status.into_enum()
    }

    /// Tests whether the UTF-16 encoded string contains only printable characters.
    /// # Safety
    /// The caller must ensure that the pointer will not be pointing invalid memory, is null-terminated and will only contain valid UTF-16 characters until is null-terminated.
    ///
    /// If those conditions are violated, this will result in Undefined Behaviour.
    pub unsafe fn test_string_raw(&self, string: *const u16) -> EfiStatusEnum {
        (self.test_string)(self, string).into_enum()
    }

    /// Tests whether the UTF-16 encoded string contains only printable characters.
    /// # Safety
    /// The caller must ensure that the string is null-terminated and will only contain valid UTF-16 characters until is null-terminated.
    ///
    /// If those conditions are violated, this will result in Undefined Behaviour.
    pub unsafe fn test_string_slice(&self, string: &[u16]) -> EfiStatusEnum {
        (self.test_string)(self, string.as_ptr()).into_enum()
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
                    continue;
                }

                if buffer[index] != 0 {
                    index += 1;
                }
            }

            status = (self.test_string)(self, buffer.as_ptr());

            if status.is_error() {
                return status.into_enum();
            }

            string = string.split_at(index).1;

            if string.is_empty() {
                break;
            }
        }

        status.into_enum()
    }

    pub fn query_mode(&self, mode: usize, columns: &mut usize, rows: &mut usize) -> EfiStatusEnum {
        (self.query_mode)(self, mode, columns, rows).into_enum()
    }

    pub fn set_mode(&self, mode: usize) -> EfiStatusEnum {
        (self.set_mode)(self, mode).into_enum()
    }

    pub fn set_attribute(&self, attribute: usize) -> EfiStatusEnum {
        (self.set_attribute)(self, attribute).into_enum()
    }

    pub fn clear_screen(&self) -> EfiStatusEnum {
        (self.clear_screen)(self).into_enum()
    }

    pub fn set_cursor_position(&self, column: usize, row: usize) -> EfiStatusEnum {
        (self.set_cursor_position)(self, column, row).into_enum()
    }

    pub fn enable_cursor(&self, visible: bool) -> EfiStatusEnum {
        (self.enable_cursor)(self, visible).into_enum()
    }

    pub fn get_mode(&self) -> EfiSimpleTextOutputMode {
        unsafe { self.mode.read_unaligned() }
    }
}

impl EfiProtocol for EfiSimpleTextOutputProtocol {
    type Parsed = &'static Self;
    type Error = !;

    fn guid() -> EfiGuid {
        crate::guids::EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL
    }

    unsafe fn parse(
        ptr: NonNullVoidPtr,
    ) -> Result<<Self as EfiProtocol>::Parsed, <Self as EfiProtocol>::Error> {
        Ok(&*ptr.cast().as_ptr())
    }
}

impl Write for EfiSimpleTextOutputProtocol {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        if self.output_string(s).is_error() {
            Err(Error)
        } else {
            Ok(())
        }
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
