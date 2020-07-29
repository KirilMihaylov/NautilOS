/// Macro for printing formatted strings on the general console output.
/// It uses [`panic_handling`]'s [`CON_OUT`] to acquire pointer to the console output protocol's interface.
#[macro_export]
macro_rules! print {
	($($args:tt)+) => {
		{
			let con_out: *mut EfiSimpleTextOutputProtocol = CON_OUT.load(Ordering::Relaxed);

			if !con_out.is_null() && (con_out as usize) % core::mem::align_of::<EfiSimpleTextOutputProtocol>() == 0 {
				match core::fmt::write(unsafe { &mut *con_out }, format_args!($($args)+)) { _ => () }
			}
		}
	};
}

/// Equivalent of [`print!`] that appends new line character (`'\n'; 10; 0x0A`) in the end of the formatted string.
#[macro_export]
macro_rules! println {
	() => {
		print!("\n");
	};
	($($args:tt)+) => {
		print!("{}\n", format_args!($($args)+));
	};
}

/// Equivalent of [`println!`] that appends `[DEBUG] ` in the beginning of the passed formatted string.
#[macro_export]
macro_rules! debug_info {
	($($args:tt)+) => {
		println!("[DEBUG] {}", format_args!($($args)+));
	}
}

/// Equivalent of [`println!`] that appends `[LOG] ` in the beginning of the passed formatted string.
#[macro_export]
macro_rules! log {
	($($args:tt)+) => {
		println!("[LOG] {}", format_args!($($args)+));
	}
}

/// Equivalent of [`println!`] that appends `[WARN] ` in the beginning of the passed formatted string.
#[macro_export]
macro_rules! warn {
	($($args:tt)+) => {
		println!("[WARN] {}", format_args!($($args)+));
	}
}

/// Equivalent of [`warn!`] that appends `(EFI) ` in the beginning of the passed formatted string.
#[macro_export]
macro_rules! efi_warn {
	($($args:tt)+) => {
		warn!(
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
