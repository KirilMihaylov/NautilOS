use core::slice::from_raw_parts;

use crate::protocols::device_path::{
	EfiDevicePathProcotol,
	EfiDevicePathInto,
};

#[repr(C)]
pub struct EfiExtendedAcpiDevicePath {
	base: EfiDevicePathProcotol,
	_hid: [u8; 4],
	_uid: [u8; 4],
	_cid: [u8; 4],
	_hidstr: [u8; 1],
}

impl EfiExtendedAcpiDevicePath {
	unsafe fn str_len(mut string: *const u8) -> usize {
		for length in 0usize.. {
			if *string == 0 {
				return length;
			}
			string = string.offset(1);
		}
		
		unreachable!();
	}

	unsafe fn _uidstr(&self) -> *const u8 {
		let _hidstr_ptr: *const u8 = self._hidstr.as_ptr();

		_hidstr_ptr.offset(
			Self::str_len(_hidstr_ptr) as isize + 1
		)
	}

	unsafe fn _cidstr(&self) -> *const u8 {
		let _uidstr_ptr: *const u8 = self._uidstr();

		_uidstr_ptr.offset(
			Self::str_len(_uidstr_ptr) as isize + 1
		)
	}

	pub fn _hid<'a>(&'a self) -> &'a [u8] {
		if self._hidstr[0] != 0 {
			let _hid_ptr: *const u8 = self._hidstr.as_ptr();
			unsafe {
				from_raw_parts(
					_hid_ptr,
					Self::str_len(_hid_ptr)
				)
			}
		} else {
			&self._hid
		}
	}

	pub fn _uid<'a>(&'a self) -> &'a [u8] {
		unsafe {
			let _uidstr_ptr: *const u8 = self._uidstr();
			
			if *_uidstr_ptr == 0 {
				&self._uid
			} else {
				from_raw_parts(
					_uidstr_ptr,
					Self::str_len(_uidstr_ptr)
				)
			}
		}
	}

	pub fn _cid<'a>(&'a self) -> &'a [u8] {
		unsafe {
			let _cidstr_ptr: *const u8 = self._cidstr();
			
			if *_cidstr_ptr == 0 {
				&self._cid
			} else {
				from_raw_parts(
					_cidstr_ptr,
					Self::str_len(_cidstr_ptr)
				)
			}
		}
	}
}

impl EfiDevicePathInto<EfiExtendedAcpiDevicePath> for EfiExtendedAcpiDevicePath {}

/* PRIVATE EFI STRUCTURES, TRAITS & IMPLEMENTATIONS */
