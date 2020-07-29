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
//!                 * Return `Err` with [`Unavailable`].
//!     * Detection mechanism is unavailable.
//!         * Return `Err` with [`Unavailable`].
//! * Detection mechanism is not required (feature is built-in of the platform).
//!     * Check whether feature is available.
//!         * Feature is available.
//!             * Check feature's state (enabled or disabled).
//!             * Return `Ok` with respective [`FeatureState`] value.
//!         * Feature is unavailable.
//!             * Return `Err` with [`Unavailable`].

#![allow(unused_imports)]

use {
    crate::result::{
        Error::{self, Unavailable},
        Result,
    },
    core::sync::atomic::{AtomicBool, Ordering::Relaxed},
};

/// Defines feature states.
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
/// Returns `Err` with [`Unavailable`] when mechanism is unavailable.
/// Returns `Err` with respective [`Error`] value when an error occured while checking.
pub fn detection_mechanism_available() -> Result<FeatureState> {
    use Error::*;
    use FeatureState::*;

    target_arch_else_unimplemented_error! {
        ["x86"] {
            if DETECTION_MECHANISM.load(Relaxed) { Ok(Enabled) }
            else {
                let flags: usize;

                unsafe {
                    asm!(
                        "pushf
                        pop {flags}",
                        flags = lateout(reg) flags,
                        options(pure, nomem)
                    );
                }

                let updated_flags: usize;

                unsafe {
                    asm!(
                        "push {flags}
						popf
						pushf
						pop {new_flags}",
                        new_flags = lateout(reg) updated_flags,
                        flags = in(reg) (flags ^ (1usize << 21)),
                        options(pure, nomem)
                    );
                }

                let value: bool = (flags ^ updated_flags) >> 21 & 1 == 1;

                DETECTION_MECHANISM.store(value, Relaxed);

                if value {
                    Ok(Enabled)
                } else {
                    Err(Unavailable)
                }
            }
        },
        ["x86_64"] {
            Ok(Enabled)
        }
    }
}

/// This function attempts to enable the feature detection mechanism and returns the new state when no errors occured.
///
/// It returns `Ok` when mechanism is available.
/// Returns `Err` with [`Unavailable`] when mechanism is unavailable.
/// Returns `Err` with respective [`Error`] value when an error occured while checking.
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
            if detection_mechanism_available().is_ok() {
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
///
/// It returns `Ok` when mechanism is available.
/// Returns `Err` with [`Unavailable`] when mechanism is unavailable.
/// Returns `Err` with respective [`Error`] value when an error occured while checking.
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
            if detection_mechanism_available().is_ok() {
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
///
/// It returns `Ok` when mechanism is available.
/// Returns `Err` with [`Unavailable`] when mechanism is unavailable.
/// Returns `Err` with respective [`Error`] value when an error occured while checking.
pub fn cpu_vendor_id_available() -> Result<FeatureState> {
    use Error::*;
    use FeatureState::*;

    target_arch_else_unimplemented_error! {
        ["x86"] {
            detection_mechanism_available()
                .map(
                    |_| Enabled
                )
                .map_err(
                    |_| Unavailable
                )
        },
        ["x86_64"] {
            Ok(Enabled)
        }
    }
}

/// Provides methods for detecting whether SIMD instruction sets are available.
pub mod simd {
    pub mod simd128;
    pub mod simd64;
}
pub mod state_storing;
