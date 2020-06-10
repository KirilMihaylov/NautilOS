//! Provides interface over platform's state storing features.

use crate::{
	result::{
		Result,
		Error::{
			self,
			FeatureDisabled,
			Unavailable,
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
pub fn state_storing_available() -> Result<FeatureState> {
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

/// This function attempts to enable the state storing mechanism and returns the new state when no errors occured.
/// 
/// It returns `Ok` when mechanism is available.
/// Returns `Err` with [`Unavailable`] when mechanism is unavailable.
/// Returns `Err` with respective [`Error`] value when an error occured while checking.
pub fn enable_state_storing() -> Result<FeatureState> {
	use Error::*;
	use FeatureState::*;

	target_arch_else_unimplemented_error!{
		["x86", "x86_64"] {
			match state_storing_available() {
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
pub fn disable_state_storing() -> Result<FeatureState> {
	use Error::*;
	use FeatureState::*;

	target_arch_else_unimplemented_error!{
		["x86", "x86_64"] {
			match state_storing_available() {
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
