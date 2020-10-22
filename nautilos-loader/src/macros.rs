/// Macro for printing formatted strings on the general console output.
/// It uses [`panic_handling`]'s [`CON_OUT`] to acquire pointer to the console output protocol's interface.
///
/// [`panic_handling`]: module@crate::panic_handling
/// [`CON_OUT`]: static@crate::panic_handling::CON_OUT
#[macro_export]
macro_rules! print {
	($($args:tt)+) => {
		{
			let con_out:
				*mut efi::protocols::console::EfiSimpleTextOutputProtocol =
				crate::panic_handling::CON_OUT.load(core::sync::atomic::Ordering::Relaxed);

			if !con_out.is_null() && (con_out as usize) % core::mem::align_of::<efi::protocols::console::EfiSimpleTextOutputProtocol>() == 0 {
				match core::fmt::write(unsafe { &mut *con_out }, format_args!($($args)+)) { _ => () }
			}
		}
	};
}

/// Equivalent of [`print!`] that appends new line character (`'\n'; 10; 0x0A`) in the end of the formatted string.
#[macro_export]
macro_rules! println {
	() => {
		$crate::print!("\n");
	};
	($($args:tt)+) => {
		$crate::print!("{}\n", format_args!($($args)+));
	};
}

/// Equivalent of [`println!`] that appends `[DEBUG] ` in the beginning of the passed formatted string.
#[macro_export]
macro_rules! debug_info {
	($($args:tt)+) => {
		$crate::println!("[DEBUG] {}", format_args!($($args)+));
	}
}

/// Equivalent of [`println!`] that appends `[LOG] ` in the beginning of the passed formatted string.
#[macro_export]
macro_rules! log {
	($($args:tt)+) => {
		$crate::println!("[LOG] {}", format_args!($($args)+));
	}
}

/// Equivalent of [`println!`] that appends `[WARN] ` in the beginning of the passed formatted string.
#[macro_export]
macro_rules! warn {
	($($args:tt)+) => {
		$crate::println!("[WARN] {}", format_args!($($args)+));
	}
}

/// Equivalent of [`warn!`] that appends `(EFI) ` in the beginning of the passed formatted string.
#[macro_export]
macro_rules! efi_warn {
	($($args:tt)+) => {
		$crate::warn!(
			"(EFI) {}",
			format_args!($($args)+)
		);
	};
}

/// Equivalent of [`panic!`] that appends `(EFI) ` in the beginning of the passed formatted string.
#[macro_export]
macro_rules! efi_panic {
	($($args:tt)+) => {
		panic!(
			"(EFI) {}",
			format_args!($($args)+)
		);
	};
}

/// Equivalent of [`assert!`] that appends `(EFI) ` in the beginning of the passed formatted string.
#[macro_export]
macro_rules! efi_assert {
	($expr:expr, $($args:tt)+) => {
		assert!(
			$expr,
			"(EFI) {}",
			format_args!($($args)+)
		);
	};
}
