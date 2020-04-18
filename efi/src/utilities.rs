pub fn validate_string(data: &[u16]) -> Result<(), ()> {
	use core::char::{
		decode_utf16,
		DecodeUtf16,
	};

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
