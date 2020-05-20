//! This module defines a basic UEFI-compatible panic handler.

use core::{
	sync::atomic::{
		AtomicPtr,
		Ordering::Relaxed,
	},
	mem::align_of,
};

use efi::protocols::console::simple_text_output_protocol::EfiSimpleTextOutputProtocol;

/// Stores pointer to UEFI's console output protocol interface.
pub static CON_OUT: AtomicPtr<EfiSimpleTextOutputProtocol> = AtomicPtr::new(0 as _);

/// Panic handler's implementation.
///
/// It uses directly [`CON_OUT`] to retrieve a pointer to console output protocol's interface.
/// It checks whether the pointer is valid (non-null & properly aligned).
/// 
/// **Warning**: Dangling pointers **cannot** be checked, so resetting to a null pointer before doing any memory map changes is recommended.
/// 
/// ['CON_OUT']: panic/static.CON_OUT.html
#[cfg_attr(not(test), panic_handler)]
#[cfg_attr(test, allow(dead_code))]
fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
	let con_out: *mut EfiSimpleTextOutputProtocol = CON_OUT.load(Relaxed);
	
	if !con_out.is_null() && (con_out as usize) % align_of::<EfiSimpleTextOutputProtocol>() == 0 {
		use core::fmt::write;

		let con_out: &mut EfiSimpleTextOutputProtocol = unsafe { &mut *con_out };

		con_out.output_string("\nPanic!");

		if let Some(location) = panic_info.location() {
			match write(
				con_out,
				format_args!(
					" [{} -> Line {} : Column {}]",
					location.file(),
					location.line(),
					location.column()
				)
			) { _ => (), }
		}

		if let Some(message) = panic_info.message() {
			match write(
				con_out,
				format_args!(
					"\nError: {}",
					message
				)
			) { _ => (), }
		}
	}

	loop {}
}
