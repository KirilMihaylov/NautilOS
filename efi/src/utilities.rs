use core::{
    char::{decode_utf16, DecodeUtf16},
    slice::{from_raw_parts, from_raw_parts_mut},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct RawUtf16StringError;

pub fn validate_string(data: &[u16]) -> Result<(), RawUtf16StringError> {
    if data.is_empty() {
        Err(RawUtf16StringError)
    } else {
        let utf16_iter: DecodeUtf16<_> = decode_utf16(data.iter().cloned());

        if utf16_iter.clone().any(|c| c.is_err()) {
            /* Has invalid characters? */
            Err(RawUtf16StringError)
        }
        /* All characters are valid */
        else if utf16_iter.clone().any(|c| c.unwrap_or('?') == '\0') {
            /* Has terminating-null? */
            Ok(())
        }
        /* Doesn't have terminating-null */
        else {
            Err(RawUtf16StringError)
        }
    }
}

/// Returns the length of raw `u16` null-terminated string.
/// # Safety
/// Returns `Err` if the pointer is null-pointer or is not properly aligned.
/// The called must ensure the pointer will not be pointing to invalid memory until it is null-terminated.
pub unsafe fn string_length(string: *const u16) -> Result<usize, RawUtf16StringError> {
    string_length_max(string, usize::max_value())
}

/// Returns the length of raw `u16` null-terminated string with an end bound.
/// # Safety
/// Returns `Err` if the pointer is null-pointer, is not properly aligned or the end bound is reached before reaching a null-terminator.
/// The called must ensure the pointer will not be pointing to invalid memory until the end bound comes.
pub unsafe fn string_length_max(
    mut string: *const u16,
    buffer_length: usize,
) -> Result<usize, RawUtf16StringError> {
    if string.is_null() || string.align_offset(core::mem::align_of::<u16>()) != 0 {
        return Err(RawUtf16StringError);
    }

    for length in 0usize..=buffer_length {
        if *string == 0 {
            return Ok(length);
        }
        string = string.offset(1);
    }

    Err(RawUtf16StringError)
}

/// Returns a checked slice of raw `u16` null-terminated string.
/// # Safety
/// Returns `Err` if the pointer is null-pointer, is not properly aligned or the string contains invalid UTF-16 characters.
/// The called must ensure the pointer will not be pointing to invalid memory until it is null-terminated.
pub unsafe fn string_from_raw<'a>(string: *const u16) -> Result<&'a [u16], RawUtf16StringError> {
    if string.is_null() || string.align_offset(core::mem::align_of::<u16>()) != 0 {
        return Err(RawUtf16StringError);
    }

    let string: &'a [u16] = from_raw_parts(
        string,
        match string_length(string) {
            Ok(data) => data,
            Err(error) => return Err(error),
        },
    );

    if decode_utf16(string.iter().cloned()).any(|c| c.is_err()) {
        Err(RawUtf16StringError)
    } else {
        Ok(string)
    }
}

/// Returns a checked mutable slice of raw `u16` null-terminated string.
/// # Safety
/// Returns `Err` if the pointer is null-pointer, is not properly aligned or the string contains invalid UTF-16 characters.
/// The called must ensure the pointer will not be pointing to invalid memory until it is null-terminated.
pub unsafe fn string_from_raw_mut<'a>(
    string: *mut u16,
) -> Result<&'a mut [u16], RawUtf16StringError> {
    if string.is_null() {
        Err(RawUtf16StringError)
    } else {
        let string: &'a mut [u16] = from_raw_parts_mut(
            string,
            match string_length(string) {
                Ok(data) => data,
                Err(error) => return Err(error),
            },
        );

        if decode_utf16(string.iter().cloned()).any(|c| c.is_err()) {
            Err(RawUtf16StringError)
        } else {
            Ok(string)
        }
    }
}
