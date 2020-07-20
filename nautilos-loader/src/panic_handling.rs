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
pub static CON_OUT: AtomicPtr<EfiSimpleTextOutputProtocol> = AtomicPtr::new(core::ptr::null_mut());

#[doc(hidden)]
static IN_PANIC: AtomicBool = AtomicBool::new(false);

/// Panic handler's implementation.
///
/// Acquires the pointer to the console output protocol's interface from [`CON_OUT`].
/// It checks whether the pointer is non-null & properly aligned.
/// 
/// **Note**: Since dangling pointers **can not** be validated, so setting to a null pointer while doing any memory map changes is mandatory.
#[cfg_attr(not(test), panic_handler)]
#[cfg_attr(test, allow(dead_code))]
fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
	/* Stops recursive panics and allows multi-threading */
	while !IN_PANIC.compare_and_swap(false, true, Relaxed) {}

	let con_out: *mut EfiSimpleTextOutputProtocol = CON_OUT.load(Relaxed);
	
	if con_out.align_offset(align_of::<EfiSimpleTextOutputProtocol>()) == 0 {
		if let Some(con_out) = unsafe { con_out.as_mut() } {
			use core::fmt::write;
			
			let (file, line, column): (&str, u32, u32) =
				if let Some(location) = panic_info.location() {
					(location.file(), location.line(), location.column())
				} else {
					("", 0, 0)
				};

			let message: core::fmt::Arguments =
				if let Some(message) = panic_info.message() {
					*message
				} else {
					format_args!("(No message)")
				};

			let _ = write(
				con_out,
				format_args!(
					"\nPanic [{} -> Line {} : Column {}]\nError message: {}",
					file,
					line,
					column,
					message
				)
			);
		}
	}

	/* Reset state in case of multithreading */
	IN_PANIC.store(false, Relaxed);

	loop {}
}
