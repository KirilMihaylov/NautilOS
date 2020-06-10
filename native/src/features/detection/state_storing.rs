//! Provides interface over platform's state storing features.

use crate::{
	result::{
		Result,
		Error::{
			self,
			FeatureDisabled,
		},
	},
	features::detection::{
		FeatureState,
		detection_mechanism_available,
	},
};

/// Checks whether extended (feature defined) state storing mechanism is available.
/// 
/// Returns `Err` with [`FeatureDisabled`] when feature detection mechanism is required but is disabled.
/// Returns error value returned by [`detection_mechanism_available`] when it returns an error.
pub fn available() -> Result<FeatureState> {
	use Error::*;
	use FeatureState::*;

	target_arch_else_unimplemented_error!{
		["x86", "x86_64"] {
			/* XSAVE */

			match detection_mechanism_available() {
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
