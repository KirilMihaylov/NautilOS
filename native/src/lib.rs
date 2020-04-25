#![no_std]
#![allow(dead_code)]

/* For platform-specific operations */
#![feature(llvm_asm)]

#[macro_use]
mod macros;

/*
	[...], [...] -> Targets
	[...] -> Target
	... -> Target options (Anything that can be used with "#[cfg]"; e.g.: not, any, target_width)
*/
supported_targets!{
	[target_arch="x86"],
	[target_arch="x86_64"]
}

#[cfg(feature="memory_c")]
mod memory_c;

pub mod features;
