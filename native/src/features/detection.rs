/* For features and feature sets that are obsolete. */
#![allow(unused_imports,deprecated)]

//! Provides methods for detecting, enabling and disabling specific features.
//! 
//! Feature availability functions have platform-specific behaviour.
//! They follow this general behaviour:
//! * Detection mechanism is required (feature is an extension).
//!     * Detection mechanism is available.
//!         * Check whether feature is available.
//!             * Feature is available.
//!                 * Check feature's state (enabled or disabled).
//!                 * Return `Ok` with respective [`FeatureState`] value.
//!             * Feature is unavailable.
//!                 * Return `Err` with [`Error::Unavailable`].
//!     * Detection mechanism is unavailable.
//!         * Return `Err` with [`Error::Unavailable`].
//! * Detection mechanism is not required (feature is built-in of the platform).
//!     * Check whether feature is available.
//!         * Feature is available.
//!             * Check feature's state (enabled or disabled).
//!             * Return `Ok` with respective [`FeatureState`] value.
//!         * Feature is unavailable.
//!             * Return `Err` with [`Error::Unavailable`].
//! 
//! [`Error::Unavailable`]: ../../enum.Error.html#variant.Unavailable

use core::sync::atomic::{
	AtomicBool,
	Ordering::Relaxed,
};

use crate::result::{
	Result,
	Error,
};

/// Defines feature states
#[derive(Debug)]
pub enum FeatureState {
	/// Feature is available but disabled
	Disabled,
	/// Feature is available and enabled
	Enabled,
}

static DETECTION_MECHANISM: AtomicBool = AtomicBool::new(false);

/// Checks whether there is available feature detection mechanism.
/// 
/// It returns `Ok` when mechanism is available.
/// Returns `Err(Unavailable)` when mechanism is unavailable.
/// Returns `Err` with respective [`Error`] value when an error occured while checking.
/// 
/// [`Error`]: ../../enum.Error.html
pub fn detection_mechanism_available() -> Result<FeatureState> {
	use Error::*;
	use FeatureState::*;

	target_arch_else_unimplemented_error!{
		["x86"] {
			if DETECTION_MECHANISM.load(Relaxed) { Ok(Enabled) }
			else {
				let flags: usize;

				unsafe {
					llvm_asm!(
						"pushf
						pop $0" :
						"=r"(flags)
					);
				}

				let updated_flags: usize;

				unsafe {
					llvm_asm!(
						"push $1
						popf

						pushf
						pop $0" :
						"=r"(updated_flags) :
						"r"(flags ^ (1usize << 21))
					);
				}

				(|value: bool| -> Result<FeatureState> {
					DETECTION_MECHANISM.store(value, Relaxed);

					if value {
						Ok(Enabled)
					} else {
						Err(Unavailable)
					}
				})(((flags ^ updated_flags) >> 21) & 1 == 1)
			}
		},
		["x86_64"] {
			Ok(Enabled)
		}
	}
}

/// This function attempts to enable the feature detection mechanism and returns the new state when no errors occured.
pub fn enable_detection_mechanism() -> Result<FeatureState> {
	use Error::*;
	use FeatureState::*;

	not_target_arch! {
		["x86_64"] {
			DETECTION_MECHANISM.store(false, Relaxed);
		}
	}

	target_arch_else_unimplemented_error! {
		["x86"] {
			if let Ok(_) = detection_mechanism_available() {
				DETECTION_MECHANISM.store(true, Relaxed);
				Ok(Enabled)
			} else {
				Err(Unavailable)
			}
		},
		["x86_64"] {
			Ok(Enabled)
		}
	}
}

/// This function attempts to disable the feature detection mechanism and returns the new state when no errors occured.
pub fn disable_detection_mechanism() -> Result<FeatureState> {
	use Error::*;
	use FeatureState::*;

	not_target_arch! {
		["x86_64"] {
			DETECTION_MECHANISM.store(false, Relaxed);
		}
	}

	target_arch_else_unimplemented_error! {
		["x86"] {
			if let Ok(_) = detection_mechanism_available() {
				DETECTION_MECHANISM.store(true, Relaxed);
				Ok(Enabled)
			} else {
				Err(Unavailable)
			}
		},
		["x86_64"] {
			Ok(Enabled)
		}
	}
}

/// Checks whether CPU vendor's identification is available.
pub fn cpu_vendor_id_available() -> Result<FeatureState> {
	use Error::*;
	use FeatureState::*;

	target_arch_else_unimplemented_error!{
		["x86"] {
			if let Ok(_) = detection_mechanism_available() {
				Ok(Enabled)
			} else {
				Err(Unavailable)
			}
		},
		["x86_64"] {
			Ok(Enabled)
		}
	}
}

pub use super::simd64::{
	simd_64_min_available,
	simd_64_available,
	simd_64_max_available,
};

/// Checks whether the recommended 128-bit SIMD instructions are supported.
/// 
/// Returns `Err` with [`Error::FeatureDisabled`] when feature detection mechanism is disabled.
/// Returns error value returned by [`detection_mechanism_available`] when feature detection mechanism is unavailable.
/// 
/// [`Error::FeatureDisabled`]: ../../enum.Error.html#variant.FeatureDisabled
pub fn simd_128_min_available() -> Result<FeatureState> {
	use Error::*;
	use FeatureState::*;

	target_arch_else_unimplemented_error!{
		["x86", "x86_64"] {
			/* SSE */

			match detection_mechanism_available() {
				Ok(Enabled) => {
					/*
					SSE: D[25]
					*/

					let result: u32;

					unsafe { llvm_asm!("cpuid" : "={edx}"(result) : "{eax}"(1)); }

					if result >> 25 & 1 == 1 { Ok(Enabled) } else { Err(Unavailable) }
				},
				Ok(Disabled) => Err(FeatureDisabled),
				error => error,
			}
		}
	}
}

/// Checks whether the recommended 128-bit SIMD instructions are supported.
/// 
/// Returns `Err` with [`Error::FeatureDisabled`] when feature detection mechanism is disabled.
/// Returns error value returned by [`detection_mechanism_available`] when feature detection mechanism is unavailable.
/// 
/// [`Error::FeatureDisabled`]: ../../enum.Error.html#variant.FeatureDisabled
pub fn simd_128_available() -> Result<FeatureState> {
	use Error::*;
	use FeatureState::*;

	target_arch_else_unimplemented_error!{
		["x86", "x86_64"] {
			/* SSE2 */

			match detection_mechanism_available() {
				Ok(Enabled) => {
					/*
					SSE: D[25]
					SSE2: D[26]
					*/

					let result: u32;

					unsafe { llvm_asm!("cpuid" : "={edx}"(result) : "{eax}"(1)); }

					if result >> 25 & 3 == 3 { Ok(Enabled) } else { Err(Unavailable) }
				},
				Ok(Disabled) => Err(FeatureDisabled),
				error => error,
			}
		}
	}
}

/// Checks whether the recommended 128-bit SIMD instructions are supported.
/// 
/// Returns `Err` with [`Error::FeatureDisabled`] when feature detection mechanism is disabled.
/// Returns error value returned by [`detection_mechanism_available`] when feature detection mechanism is unavailable.
/// 
/// [`Error::FeatureDisabled`]: ../../enum.Error.html#variant.FeatureDisabled
pub fn simd_128_max_available() -> Result<FeatureState> {
	use Error::*;
	use FeatureState::*;

	target_arch_else_unimplemented_error!{
		["x86", "x86_64"] {
			match detection_mechanism_available() {
				Ok(Enabled) => {
					/*
					SSE: D[25]
					SSE2: D[26]
					SSE3: C[0]
					SSSE3: C[9]
					SSE4.1: C[19]
					SSE4.2: C[20]
					*/

					let (result_c, result_d): (u32, u32);

					unsafe { llvm_asm!("cpuid" : "={ecx}"(result_c), "={edx}"(result_d) : "{eax}"(1)); }

					if result_c & 0x180201 == 0x180201 && result_d >> 25 & 3 == 3 { Ok(Enabled) } else { Err(Unavailable) }
				},
				Ok(Disabled) => Err(FeatureDisabled),
				error => error,
			}
		}
	}
}

/// Checks whether the recommended 256-bit SIMD instructions are supported.
/// 
/// Returns `Err` with [`Error::FeatureDisabled`] when feature detection mechanism is disabled.
/// Returns error value returned by [`detection_mechanism_available`] when feature detection mechanism is unavailable.
/// 
/// [`Error::FeatureDisabled`]: ../../enum.Error.html#variant.FeatureDisabled
pub fn simd_256_min_available() -> Result<FeatureState> {
	use Error::*;
	use FeatureState::*;

	target_arch_else_unimplemented_error!{
		["x86", "x86_64"] {
			/* AVX */

			match detection_mechanism_available() {
				Ok(Enabled) => {
					/*
					AVX: C[28]
					*/

					let result: u32;

					unsafe { llvm_asm!("cpuid" : "={ecx}"(result) : "{eax}"(1)); }

					if result >> 28 & 1 == 1 {
						#[cfg(not(feature="kernel_mode"))]
						{
							/*
							SSE: XCR0[1]
							AVX: XCR0[2]
							*/

							if result >> 27 & 1 == 1 {
								let result: u32;

								unsafe { llvm_asm!("xgetbv" : "={eax}"(result) : "{ecx}"(0)); }

								return Ok(if result & 6 == 6 { Enabled } else { Disabled });
							}

							Err(OsManagedFeature)
						}

						#[cfg(feature="kernel_mode")]
						{
							/*
							Monitor co-processor: CR0[1]
							Emulation: CR0[2]

							*/

							let (cr0, cr4): (usize, usize);

							unsafe {
								llvm_asm!("mov $0, cr0" : "=r"(cr0) ::: "intel");
								llvm_asm!("mov $0, cr4" : "=r"(cr4) ::: "intel");
							}

							return Ok(if cr0 & 6 == 2 && cr4 >> 9 & 3 == 3 { Enabled } else { Disabled });
						}
					}

					return Err(Unavailable);
				},
				Ok(Disabled) => Err(FeatureDisabled),
				error => error,
			}
		}
	}
}

/// Checks whether the recommended 128-bit SIMD instructions are supported.
/// 
/// Returns `Err` with [`Error::FeatureDisabled`] when feature detection mechanism is disabled.
/// Returns error value returned by [`detection_mechanism_available`] when feature detection mechanism is unavailable.
/// 
/// [`Error::FeatureDisabled`]: ../../enum.Error.html#variant.FeatureDisabled
pub fn simd_256_available() -> Result<FeatureState> {
	use Error::*;
	use FeatureState::*;

	target_arch_else_unimplemented_error!{
		["x86", "x86_64"] {
			/* SSE2 */

			match detection_mechanism_available() {
				Ok(Enabled) => {
					/*
					MMX: D[23]
					SSE: D[25]
					SSE2: D[26]
					*/

					let result: u32;

					unsafe { llvm_asm!("cpuid" : "={edx}"(result) : "{eax}"(1)); }

					if result >> 23 & 13 == 13 { Ok(Enabled) } else { Err(Unavailable) }
				},
				Ok(Disabled) => Err(FeatureDisabled),
				error => error,
			}
		}
	}
}

/// Checks whether the recommended 128-bit SIMD instructions are supported.
/// 
/// Returns `Err` with [`Error::FeatureDisabled`] when feature detection mechanism is disabled.
/// Returns error value returned by [`detection_mechanism_available`] when feature detection mechanism is unavailable.
/// 
/// [`Error::FeatureDisabled`]: ../../enum.Error.html#variant.FeatureDisabled
pub fn simd_256_max_available() -> Result<FeatureState> {
	use Error::*;
	use FeatureState::*;

	target_arch_else_unimplemented_error!{
		["x86", "x86_64"] {
			match detection_mechanism_available() {
				Ok(Enabled) => {
					/*
					MMX: D[23]
					SSE: D[25]
					SSE2: D[26]
					SSE3: C[0]
					SSSE3: C[9]
					SSE4.1: C[19]
					SSE4.2: C[20]
					*/

					let (result_c, result_d): (u32, u32);

					unsafe { llvm_asm!("cpuid" : "={ecx}"(result_c), "={edx}"(result_d) : "{eax}"(1)); }

					if result_c & 0x180201 == 0x180201 && result_d >> 23 & 13 == 13 { Ok(Enabled) } else { Err(Unavailable) }
				},
				Ok(Disabled) => Err(FeatureDisabled),
				error => error,
			}
		}
	}
}
