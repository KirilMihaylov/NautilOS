#![cfg_attr(not(test), no_std)]

#[cfg(any(test,doc))]
mod tests {
	mod acpi;
	mod efi;
	mod efi_interops;
	mod native;
}
