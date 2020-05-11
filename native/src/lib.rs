#![no_std]
#![allow(dead_code)]
#![doc(html_no_source)]

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

#[cfg(all(feature="memory_c",not(doc)))]
mod memory_c;

mod result;

#[doc(inline)]
pub use result::{
	Error,
	Result,
};

pub mod features;
pub mod input_output;
