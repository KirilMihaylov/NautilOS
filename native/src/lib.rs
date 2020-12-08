//! This crate provides abstraction interface over the platform allowing more portability.
//! # Features
//! This crate provides the following features:
//! * `kernel_mode`
//!     
//!     This mode is recommended on "bare metal" environments and when it will be running with supervisor permissions.
//!     
//!     **Warning:** This mode can lead to causing exceptions (e.g.: "General Protection Exception" on IA-32 (x86) and AMD64 (x86_64)).
//!     Use with caution.
//! # Kernel mode
//! When used in kernel mode, this crate will assume supervisor permissions.

#![no_std]
#![allow(dead_code)]
#![doc(html_no_source)]
/* For platform-specific operations */
#![feature(asm)]
#![forbid(warnings)]

#[macro_use]
mod macros;

pub use native_macros::*;

/*
    [...], [...] -> Targets
    [...] -> Target
    ... -> Target options (Anything that can be used with "#[cfg]"; e.g.: not, any, target_width)
*/
supported_targets! {
    [target_arch="x86"],
    [target_arch="x86_64"]
}

mod result;

#[doc(inline)]
pub use result::{Error, Result};

pub mod features;
pub mod input_output;
