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

					let (a, c): (u32, u32);

					unsafe {
						llvm_asm!("cpuid" : "={eax}"(a) : "{eax}"(0), "{ecx}"(0), "{ebx}"(0), "{edx}"(0));

						llvm_asm!("cpuid" : "={ecx}"(c) : "{eax}"(1), "{ebx}"(0), "{edx}"(0));
					}

					if a >= 0xD && c >> 26 & 1 == 1 {
						Ok(Enabled)
					} else { Err(Unavailable) }
				},
				Ok(Disabled) => Err(FeatureDisabled),
				error => error,
			}
		}
	}
}
