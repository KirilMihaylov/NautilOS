use core::{
	char::{
		decode_utf16,
		DecodeUtf16,
	},
	slice::{
		from_raw_parts,
		from_raw_parts_mut,
	},
};

pub fn validate_string(data: &[u16]) -> Result<(), ()> {
	if data.len() == 0 {
		Err(())
	} else {
		let utf16_iter: DecodeUtf16<_> = decode_utf16(data.iter().map(|x| *x));

		if utf16_iter.clone().any(|c| c.is_err()) { /* Has invalid characters? */
			Err(())
		}
		/* All characters are valid */
		else if utf16_iter.clone().any(|c| if c.unwrap_or('?') == '\0' { true } else { false }) { /* Has terminating-null? */
			Ok(())
		}
		/* Doesn't have terminating-null */
		else {
			Err(())
		}
	}
}

pub unsafe fn string_length(mut string: *const u16) -> Result<usize, ()> {
	if string.is_null() {
		return Err(());
	}

	let mut length: usize = 0;
	while *string != 0 {
		string = string.offset(1);
		length += 1;
	}

	Ok(length)
}

pub unsafe fn string_from_raw<'a>(string: *const u16) -> Result<&'a [u16], ()> {
	if string.is_null() {
		return Err(());
	} else {
		let string: &'a [u16] = from_raw_parts(string, string_length(string)?);

		if decode_utf16(string.iter().map(|&x| x)).any(|c| c.is_err()) {
			Err(())
		} else {
			Ok(string)
		}
	}
}

pub unsafe fn string_from_raw_mut<'a>(string: *mut u16) -> Result<&'a mut [u16], ()> {
	if string.is_null() {
		return Err(());
	} else {
		let string: &'a mut [u16] = from_raw_parts_mut(string, string_length(string)?);

		if decode_utf16(string.iter().map(|&x| x)).any(|c| c.is_err()) {
			Err(())
		} else {
			Ok(string)
		}
	}
}
