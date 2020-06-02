//! Provides interface over platform's 64-bit SIMD features.

#![allow(unused_imports,deprecated)]

use crate::result::{
	Result,
	Error,
};

use super::detection::{
	detection_mechanism_available,
	FeatureState,
};

/// Checks whether the minimal 64-bit SIMD instructions are supported.
/// 
/// Returns `Err` with [`Error::FeatureDisabled`] when feature detection mechanism is disabled.
/// Returns error value returned by [`detection_mechanism_available`] when feature detection mechanism is unavailable.
/// 
/// [`Error::FeatureDisabled`]: ../../enum.Error.html#variant.FeatureDisabled
#[cfg_attr(any(target_arch = "x86", target_arch = "x86_64"), deprecated = "The 64-bit SIMD (MMX) is obsolete on this platform, consider using 128-bit SIMD (SSE or SSE2).")]
pub fn simd_64_min_available() -> Result<FeatureState> {
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
						llvm_asm!("cpuid" : "={ecx}"(c), "={edx}"(d) : "{eax}"(1), "{ebx}"(0), "{ecx}"(0), "{edx}"(0));

						#[cfg(feature="kernel_mode")]
						llvm_asm!("cpuid" : "={edx}"(d) : "{eax}"(1), "{ebx}"(0), "{ecx}"(0), "{edx}"(0));
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

								unsafe { llvm_asm!("xgetbv" : "={eax}"(result) : "{ecx}"(0), "{edx}"(0)); }

								if result & 1 == 1 {
									return Ok(Enabled);
								}
							}
							Err(OsManagedFeature)
						}

						#[cfg(feature="kernel_mode")]
						{
							let cr0: usize;

							unsafe { llvm_asm!("mov $0, cr0" : "=r"(cr0) ::: "intel"); }

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
/// Returns `Err` with [`Error::FeatureDisabled`] when feature detection mechanism is disabled.
/// Returns error value returned by [`detection_mechanism_available`] when feature detection mechanism is unavailable.
/// 
/// [`Error::FeatureDisabled`]: ../../enum.Error.html#variant.FeatureDisabled
#[cfg_attr(any(target_arch = "x86", target_arch = "x86_64"), deprecated = "The 64-bit SIMD (MMX) is obsolete on this platform, consider using 128-bit SIMD (SSE or SSE2).")]
pub fn simd_64_available() -> Result<FeatureState> {
	use Error::*;
	use FeatureState::*;

	target_arch_else_unimplemented_error!{
		["x86", "x86_64"] {
			/* MMX */

			simd_64_min_available()
		}
	}
}

/// Checks whether the maximal 64-bit SIMD instructions are supported.
/// 
/// Returns `Err` with [`Error::FeatureDisabled`] when feature detection mechanism is disabled.
/// Returns error value returned by [`detection_mechanism_available`] when feature detection mechanism is unavailable.
/// 
/// [`Error::FeatureDisabled`]: ../../enum.Error.html#variant.FeatureDisabled
#[cfg_attr(any(target_arch = "x86", target_arch = "x86_64"), deprecated = "The 64-bit SIMD (MMX) is obsolete on this platform, consider using 128-bit SIMD (SSE or SSE2).")]
pub fn simd_64_max_available() -> Result<FeatureState> {
	use Error::*;
	use FeatureState::*;

	target_arch_else_unimplemented_error!{
		["x86", "x86_64"] {
			/* MMX */

			simd_64_min_available()
		}
	}
}
