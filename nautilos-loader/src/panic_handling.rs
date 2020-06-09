//! This module defines a basic EFI-compatible panic handler.

use core::{
	sync::atomic::{
		AtomicBool,
		AtomicPtr,
		Ordering::Relaxed,
	},
	mem::align_of,
};

use efi::protocols::console::EfiSimpleTextOutputProtocol;

/// Stores pointer to EFI's console output protocol interface.
pub static CON_OUT: AtomicPtr<EfiSimpleTextOutputProtocol> = AtomicPtr::new(0 as _);

#[doc(hidden)]
static IN_PANIC: AtomicBool = AtomicBool::new(false);

/// Panic handler's implementation.
///
/// Acquires the pointer to the console output protocol's interface from [`CON_OUT`].
/// It checks whether the pointer is non-null & properly aligned.
/// 
/// **Warning**: Dangling pointers **cannot** be validated, so resetting to a null pointer before doing any memory map changes is recommended.
#[cfg_attr(not(test), panic_handler)]
#[cfg_attr(test, allow(dead_code))]
fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
	/* Stops recursive panics */
	while !IN_PANIC.compare_and_swap(false, true, Relaxed) {}

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

	/* Reset state in case of multithreading */
	IN_PANIC.store(false, Relaxed);

	loop {}
}
