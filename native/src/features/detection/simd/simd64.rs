//! Provides interface over platform's 64-bit SIMD features.

use crate::{
	result::{
		Result,
		Error::{
			self,
			FeatureDisabled,
		},
	},
	features::detection::{
		detection_mechanism_available,
		FeatureState,
	},
};

/// Checks whether the minimal 64-bit SIMD instructions are supported.
/// 
/// Returns `Err` with [`FeatureDisabled`] when feature detection mechanism is required but is disabled.
/// Returns error value returned by [`detection_mechanism_available`] when it returns an error.
#[cfg_attr(any(target_arch = "x86", target_arch = "x86_64"), deprecated = "The 64-bit SIMD (MMX) is obsolete on this platform, consider using 128-bit SIMD (SSE or SSE2).")]
pub fn min_available() -> Result<FeatureState> {
	use Error::*;
	use FeatureState::*;

	target_arch_else_unimplemented_error!{
		["x86", "x86_64"] {
			/* MMX */

			match detection_mechanism_available() {
				Ok(Enabled) => {
					/*
					MMX: CPUID[1].D[23]
					FXSAVE/FXRSTOR: CPUID[1].D[24]
					*/

					#[cfg(not(feature="kernel_mode"))]
					let c: u32;

					let d: u32;

					unsafe {
						#[cfg(not(feature="kernel_mode"))]
						asm!(
							"cpuid",
							inlateout("eax") 1 => _,
							out("ebx") _,
							out("ecx") c,
							out("edx") d,
							options(nomem, nostack)
						);

						#[cfg(feature="kernel_mode")]
						asm!(
							"cpuid",
							inout("eax") 1 => _,
							out("ebx") _,
							out("ecx") _,
							out("edx") d,
							options(nomem, nostack)
						);
					}

					if d >> 23 & 3 == 3 {
						#[cfg(not(feature="kernel_mode"))]
						{
							/*
							OSXSAVE: CPUID[1].C[27]
							FPU/MMX: XCR0[0]
							*/

							if c >> 27 & 1 == 1 {
								let result: u32;

								unsafe {
									asm!(
										"xgetbv",
										out("eax") result,
										in("ecx") 0,
										out("edx") _,
										options(nomem, nostack)
									);
								}

								if result & 1 == 1 {
									return Ok(Enabled);
								}
							}
							Err(OsInteractionRequired)
						}

						#[cfg(feature="kernel_mode")]
						{
							let cr0: usize;

							unsafe {
								asm!(
									"mov {cr0}, cr0",
									cr0 = out(reg) cr0,
									options(nomem, nostack)
								);
							}

							if cr0 & 6 == 6 {
								Ok(Enabled)
							} else {
								Ok(Disabled)
							}
						}
					} else { Err(Unavailable) }
				},
				Ok(Disabled) => Err(FeatureDisabled),
				error => error,
			}
		}
	}
}

/// Checks whether the recommended 64-bit SIMD instructions are supported.
/// 
/// Returns `Err` with [`FeatureDisabled`] when feature detection mechanism is required but is disabled.
/// Returns error value returned by [`detection_mechanism_available`] when it returns an error.
#[cfg_attr(any(target_arch = "x86", target_arch = "x86_64"), deprecated = "The 64-bit SIMD (MMX) is obsolete on this platform, consider using 128-bit SIMD (SSE or SSE2).")]
pub fn available() -> Result<FeatureState> {
	use Error::*;
	use FeatureState::*;

	target_arch_else_unimplemented_error!{
		["x86", "x86_64"] {
			/* MMX */

			#[allow(deprecated)]
			min_available()
		}
	}
}

/// Checks whether the maximal 64-bit SIMD instructions are supported.
/// 
/// Returns `Err` with [`FeatureDisabled`] when feature detection mechanism is required but is disabled.
/// Returns error value returned by [`detection_mechanism_available`] when it returns an error.
#[cfg_attr(any(target_arch = "x86", target_arch = "x86_64"), deprecated = "The 64-bit SIMD (MMX) is obsolete on this platform, consider using 128-bit SIMD (SSE or SSE2).")]
pub fn max_available() -> Result<FeatureState> {
	use Error::*;
	use FeatureState::*;

	target_arch_else_unimplemented_error!{
		["x86", "x86_64"] {
			/* MMX */

			#[allow(deprecated)]
			min_available()
		}
	}
}
