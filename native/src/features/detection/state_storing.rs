//! Provides interface over platform's state storing features.

use crate::{
    features::detection::{available as detection_available, FeatureState},
    result::{Error, Result},
};

/// Checks whether extended (feature defined) state storing mechanism is available.
///
/// Returns `Err` with [`FeatureDisabled`] when feature detection mechanism is required but is disabled.
/// Returns error value returned by [`available`] when it returns an error.
///
/// [`FeatureDisabled`]: variant@crate::features::detection::FeatureState::FeatureDisabled
/// [`available`]: fn@crate::features::detection::available
pub fn available() -> Result<FeatureState> {
    use Error::*;
    use FeatureState::*;

    target_arch_else_unimplemented_error! {
        ["x86", "x86_64"] {
            /* XSAVE */

            match detection_available() {
                Ok(Enabled) => {
                    /*
                    XSAVE: CPUID[1].C[26]
                    */

                    let mut result: u32;

                    unsafe {
                        asm!(
                            "cpuid",
                            inlateout("eax") 0 => result,
                            lateout("ebx") _,
                            lateout("ecx") _,
                            lateout("edx") _,
                            options(nomem, nostack)
                        );
                    }

                    if result >= 0xD {
                        unsafe {
                            asm!(
                                "cpuid",
                                inlateout("eax") 1 => _,
                                lateout("ebx") _,
                                lateout("ecx") result,
                                lateout("edx") _,
                                options(nomem, nostack)
                            );
                        }

                        if result >> 26 & 1 == 1 {
                            Ok(Enabled)
                        } else {
                            Err(Unavailable)
                        }
                    } else {
                        Err(Unavailable)
                    }
                },
                Ok(Disabled) => Err(FeatureDisabled),
                error => error,
            }
        }
    }
}

/// This function attempts to enable the state storing mechanism and returns the new state when no errors occured.
///
/// It returns `Ok` when mechanism is available.
/// Returns `Err` with [`Unavailable`] when mechanism is unavailable.
/// Returns `Err` with respective [`Error`] value when an error occured while checking.
/// 
/// [`Unavailable`]: variant@crate::result::Error::Unavailable
/// [`Error`]: enum@crate::result::Error
pub fn enable() -> Result<FeatureState> {
    use FeatureState::*;

    target_arch_else_unimplemented_error! {
        ["x86", "x86_64"] {
            match available() {
                Ok(Enabled) => Ok(Enabled),
                Ok(Disabled) => {
                    #[cfg(not(feature = "kernel_mode"))]
                    { Err(OsInteractionRequired) }

                    #[cfg(feature = "kernel_mode")]
                    {
                        unsafe {
                            asm!(
                                "mov {temp}, cr0
								or {temp}, {bit}
								mov cr0, {temp}",
                                temp = lateout(reg) _,
                                bit = const 1 << 18,
                                options(nomem, nostack)
                            );
                        }

                        Ok(Enabled)
                    }
                },
                Err(error) => Err(error),
            }
        }
    }
}

/// This function attempts to enable the state storing mechanism and returns the new state when no errors occured.
///
/// It returns `Ok` when mechanism is available.
/// Returns `Err` with [`Unavailable`] when mechanism is unavailable.
/// Returns `Err` with respective [`Error`] value when an error occured while checking.
/// 
/// [`Unavailable`]: variant@crate::result::Error::Unavailable
/// [`Error`]: enum@crate::result::Error
pub fn disable() -> Result<FeatureState> {
    use FeatureState::*;

    target_arch_else_unimplemented_error! {
        ["x86", "x86_64"] {
            match available() {
                Ok(Enabled) => {
                    #[cfg(not(feature = "kernel_mode"))]
                    { Err(OsInteractionRequired) }

                    #[cfg(feature = "kernel_mode")]
                    {
                        unsafe {
                            asm!(
                                "mov {temp}, cr0
								and {temp}, {bit}
								mov cr0, {temp}",
                                temp = lateout(reg) _,
                                bit = const !(1 << 18),
                                options(nomem, nostack)
                            );
                        }

                        Ok(Disabled)
                    }
                },
                Ok(Disabled) => Ok(Disabled),
                Err(error) => Err(error),
            }
        }
    }
}
