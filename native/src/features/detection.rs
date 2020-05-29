/* For features and feature sets that are obsolete. */
#![allow(deprecated)]

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
		["x86", "x86_64"] {
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
		}
	}
}

/// This function attempts to enable the feature detection mechanism and returns the new state when no errors occured.
pub fn enable_detection_mechanism() -> Result<FeatureState> {
	use Error::*;
	use FeatureState::*;

	DETECTION_MECHANISM.store(false, Relaxed);

	target_arch_else_unimplemented_error! {
		["x86", "x86_64"] {
			if let Ok(_) = detection_mechanism_available() {
				DETECTION_MECHANISM.store(true, Relaxed);
				Ok(Enabled)
			} else {
				Err(Unavailable)
			}
		}
	}
}

/// This function attempts to disable the feature detection mechanism and returns the new state when no errors occured.
pub fn disable_detection_mechanism() -> Result<FeatureState> {
	use Error::*;
	use FeatureState::*;

	DETECTION_MECHANISM.store(false, Relaxed);

	target_arch_else_unimplemented_error! {
		["x86", "x86_64"] {
			if let Ok(_) = detection_mechanism_available() {
				DETECTION_MECHANISM.store(true, Relaxed);
				Ok(Enabled)
			} else {
				Err(Unavailable)
			}
		}
	}
}

/// Checks whether CPU vendor's identification is available.
pub fn cpu_vendor_id_available() -> Result<FeatureState> {
	use Error::*;
	use FeatureState::*;

	target_arch_else_unimplemented_error!{
		["x86", "x86_64"] {
			if let Ok(_) = detection_mechanism_available() {
				Ok(Enabled)
			} else {
				Err(Unavailable)
			}
		}
	}
}

pub use super::simd64::{
	simd_64_min_available,
	simd_64_available,
	simd_64_max_available,
};

