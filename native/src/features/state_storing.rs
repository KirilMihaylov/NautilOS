//! Provides interface over platform's state storing features.

use crate::{
    features::detection::state_storing::available as state_storing_available, result::Result,
};

/// Contains information about the state storing mechanism.
#[derive(Clone, Copy)]
pub struct StateStoringInfo {
    required_bytes: usize,
    alignment: usize,
    simd64: bool,
    simd128: bool,
}

impl StateStoringInfo {
    /// Returns length in bytes required to store processor's architectural state with current configuration.
    pub fn required_bytes(&self) -> usize {
        self.required_bytes
    }

    /// Returns the minimal alignment required by the state storing mechanism.
    pub fn alignment(&self) -> usize {
        self.alignment
    }

    /// Returns true when 64-bit SIMD state can be stored by the mechanism.
    pub fn simd64(&self) -> bool {
        self.simd64
    }

    /// Returns true when 128-bit SIMD state can be stored by the mechanism.
    pub fn simd128(&self) -> bool {
        self.simd128
    }
}

/// Gathers information about which features are going to be stored and the size of the block required to store the extended (defined by specific features which may differ) state.
/// # Notes
/// Depending on the platform the architectural state storing and the extended state storing *may or may not* be separable.
/// In case they are not, this function will return `Err` with [`Unavailable`], even when [`state_storing_available`] returns value indicating the mechanism itself is.
pub fn extended_state_storing_info() -> Result<StateStoringInfo> {
    match state_storing_available() {
        Ok(_feature_state) => {
            target_arch_else_unimplemented_error! {
                ["x86", "x86_64"] {
                    /* state_storing::available depends on detection_mechanism_available so it is safe to use CPUID */

                    let (required_bytes, features_low/*, features_high*/): (u32, u32/*, u32*/);

                    unsafe {
                        asm!(
                            "cpuid",
                            inlateout("eax") 0xD => features_low,
                            lateout("ebx") required_bytes,
                            inlateout("ecx") 0 => _,
                            lateout("edx") _ /* features_high */,
                            options(nomem, nostack)
                        );
                    }

                    Ok(
                        StateStoringInfo {
                            required_bytes: /* Extended state */ (required_bytes as usize),
                            alignment: 64,
                            simd64: features_low & 1 == 1,
                            simd128: features_low & 2 == 2,
                        }
                    )
                }
            }
        }
        Err(error) => Err(error),
    }
}

/// Gathers information about which features are going to be stored and the size of the block required to store the architectural (common for the architecture) state & the extended (defined by specific features which may differ) state.
/// # Notes
/// This function is only available in kernel mode due to priviledge limitations because this function also stores supervisor/priviledged state (e.g.: Control registers on IA-32 (x86) and AMD64 (x86_64)).
///
/// This function does **not** store the current page map (when paging is used) nor pointer to the page map (when the platform uses such to perform paging).
#[cfg(any(feature = "kernel_mode", doc))]
pub fn state_storing_info() -> Result<StateStoringInfo> {
    match state_storing_available() {
        Ok(_feature_state) => {
            target_arch_else_unimplemented_error! {
                ["x86", "x86_64"] {
                    let (required_bytes, features_low/*, features_high*/): (u32, u32/*, u32*/);

                    unsafe {
                        asm!(
                            "cpuid",
                            inlateout("eax") 0xD => features_low,
                            lateout("ebx") required_bytes,
                            inlateout("ecx") 0 => _,
                            lateout("edx") _ /* features_high */,
                            options(nomem, nostack)
                        );
                    }

                    target_arch!{
                        ["x86"] {
                            Ok(
                                StateStoringInfo {
                                    required_bytes: /* Architectural state */ 68 + /* Padding */ 60 + /* Extended state */ (required_bytes as usize),
                                    alignment: 64,
                                    simd64: features_low & 1 == 1,
                                    simd128: features_low & 2 == 2,
                                }
                            )
                        },
                        ["x86_64"] {
                            Ok(
                                StateStoringInfo {
                                    required_bytes: /* Architectural state */ 200 + /* Padding */ 56 + /* Extended state */ (required_bytes as usize),
                                    alignment: 64,
                                    simd64: features_low & 1 == 1,
                                    simd128: features_low & 2 == 2,
                                }
                            )
                        }
                    }
                }
            }
        }
        Err(error) => Err(error),
    }
}
