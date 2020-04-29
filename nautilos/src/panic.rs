use core::sync::atomic::{
	AtomicPtr,
	Ordering::Relaxed,
};

use efi::protocols::console::simple_text_output_protocol::EfiSimpleTextOutputProtocol;

pub static CON_OUT: AtomicPtr<EfiSimpleTextOutputProtocol> = AtomicPtr::new(0 as _);

#[cfg_attr(not(test), panic_handler)]
#[cfg_attr(test, allow(dead_code))]
fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
	let con_out: *mut EfiSimpleTextOutputProtocol = CON_OUT.load(Relaxed);
	
	if !con_out.is_null() {
		use core::fmt::write;

		let con_out: &mut EfiSimpleTextOutputProtocol = unsafe { &mut *con_out };

		con_out.output_string("\r\nPanic!");

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
					"\r\nError: {}",
					message
				)
			) { _ => (), }
		}
	}

	loop {}
}
