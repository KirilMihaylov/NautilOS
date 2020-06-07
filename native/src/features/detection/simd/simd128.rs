//! Provides interface over platform's 128-bit SIMD features.

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
	}
};

/// Checks whether the minimal 128-bit SIMD instructions are supported.
/// 
/// Returns `Err` with [`FeatureDisabled`] when feature detection mechanism is required but is disabled.
/// Returns error value returned by [`detection_mechanism_available`] when it returns an error.
pub fn min_available() -> Result<FeatureState> {
	use Error::*;
	use FeatureState::*;

	target_arch_else_unimplemented_error!{
		["x86"] {
			/* SSE */

			match detection_mechanism_available() {
				Ok(Enabled) => {
					/*
					SSE: CPUID[1].D[25]
					*/

					#[cfg(not(feature="kernel_mode"))]
					let c: u32;

					let d: u32;

					unsafe {
						#[cfg(not(feature="kernel_mode"))]
						llvm_asm!("cpuid" : "={ecx}"(c), "={edx}"(d) : "{eax}"(1), "{ebx}"(0));

						#[cfg(feature="kernel_mode")]
						llvm_asm!("cpuid" : "={edx}"(d) : "{eax}"(1), "{ebx}"(0), "{ecx}"(0));
					}

					if d >> 25 & 1 == 1 {
						#[cfg(not(feature="kernel_mode"))]
						{
							/*
							OSXSAVE: C[27]
							FPU/MMX: XCR0[0]
							SSE: XCR0[1]
							*/

							if c >> 27 & 1 == 1 {
								let result: u32;

								unsafe { llvm_asm!("xgetbv" : "={eax}"(result) : "{ecx}"(0), "{edx}"(0)); }

								if result & 3 == 3 {
									return Ok(Enabled);
								}
							}
							Err(OsInteractionRequired)
						}

						#[cfg(feature="kernel_mode")]
						{
							let (cr0, cr4): (usize, usize);

							unsafe { llvm_asm!("mov $0, cr0 \n mov $1, cr4" : "=r"(cr0), "=r"(cr4) ::: "intel"); }

							Ok(if cr0 & 6 == 6 && cr4 >> 9 & 3 == 3 { Enabled } else { Disabled })
						}
					} else { Err(Unavailable) }
				},
				Ok(Disabled) => Err(FeatureDisabled),
				error => error,
			}
		},
		["x86_64"] {
			/* SSE */

			#[cfg(not(feature="kernel_mode"))]
			{
				/*
				OSXSAVE: C[27]
				FPU/MMX: XCR0[0]
				SSE: XCR0[1]
				*/
				
				let c: u32;
				
				unsafe { llvm_asm!("cpuid" : "={ecx}"(c) : "{eax}"(1), "{ebx}"(0), "{edx}"(0)); }

				if c >> 27 & 1 == 1 {
					let result: u32;

					unsafe { llvm_asm!("xgetbv" : "={eax}"(result) : "{ecx}"(0), "{edx}"(0)); }

					if result & 3 == 3 {
						return Ok(Enabled);
					}
				}
				Err(OsInteractionRequired)
			}

			#[cfg(feature="kernel_mode")]
			{
				let (cr0, cr4): (usize, usize);

				unsafe { llvm_asm!("mov $0, cr0 \n mov $1, cr4" : "=r"(cr0), "=r"(cr4) ::: "intel"); }

				if cr0 & 6 == 6 && cr4 >> 9 & 3 == 3 {
					Ok(Enabled)
				} else {
					Ok(Disabled)
				}
			}
		}
	}
}

/// Checks whether the recommended 128-bit SIMD instructions are supported.
/// 
/// Returns `Err` with [`FeatureDisabled`] when feature detection mechanism is required but is disabled.
/// Returns error value returned by [`detection_mechanism_available`] when it returns an error.
pub fn available() -> Result<FeatureState> {
	use Error::*;
	use FeatureState::*;

	target_arch_else_unimplemented_error!{
		["x86"] {
			/* SSE2 */

			match detection_mechanism_available() {
				Ok(Enabled) => {
					/*
					SSE: CPUID[1].D[25]
					SSE2: CPUID[1].D[26]
					*/

					#[cfg(not(feature="kernel_mode"))]
					let c: u32;

					let d: u32;

					unsafe {
						#[cfg(not(feature="kernel_mode"))]
						llvm_asm!("cpuid" : "={ecx}"(c), "={edx}"(d) : "{eax}"(1), "{ebx}"(0));

						#[cfg(feature="kernel_mode")]
						llvm_asm!("cpuid" : "={edx}"(d) : "{eax}"(1), "{ebx}"(0), "{ecx}"(0));
					}

					if d >> 25 & 3 == 3 {
						#[cfg(not(feature="kernel_mode"))]
						{
							/*
							OSXSAVE: C[27]
							FPU/MMX: XCR0[0]
							SSE: XCR0[1]
							*/

							if c >> 27 & 1 == 1 {
								let result: u32;

								unsafe { llvm_asm!("xgetbv" : "={eax}"(result) : "{ecx}"(0), "{edx}"(0)); }

								if result & 3 == 3 {
									return Ok(Enabled);
								}
							}
							Err(OsInteractionRequired)
						}

						#[cfg(feature="kernel_mode")]
						{
							let (cr0, cr4): (usize, usize);

							unsafe { llvm_asm!("mov $0, cr0 \n mov $1, cr4" : "=r"(cr0), "=r"(cr4) ::: "intel"); }

							Ok(if cr0 & 6 == 6 && cr4 >> 9 & 3 == 3 { Enabled } else { Disabled })
						}
					} else { Err(Unavailable) }
				},
				Ok(Disabled) => Err(FeatureDisabled),
				error => error,
			}
		},
		["x86_64"] {
			/* SSE2 */

			#[cfg(not(feature="kernel_mode"))]
			{
				/*
				OSXSAVE: C[27]
				FPU/MMX: XCR0[0]
				SSE/SSE2: XCR0[1]
				*/
				
				let c: u32;
				
				unsafe { llvm_asm!("cpuid" : "={ecx}"(c), "={edx}"(d) : "{eax}"(1), "{ebx}"(0)); }

				if c >> 27 & 1 == 1 {
					let result: u32;

					unsafe { llvm_asm!("xgetbv" : "={eax}"(result) : "{ecx}"(0), "{edx}"(0)); }

					if result & 3 == 3 {
						return Ok(Enabled);
					}
				}
				Err(OsInteractionRequired)
			}

			#[cfg(feature="kernel_mode")]
			{
				let (cr0, cr4): (usize, usize);

				unsafe { llvm_asm!("mov $0, cr0 \n mov $1, cr4" : "=r"(cr0), "=r"(cr4) ::: "intel"); }

				if cr0 & 6 == 6 && cr4 >> 9 & 3 == 3 {
					Ok(Enabled)
				} else {
					Ok(Disabled)
				}
			}
		}
	}
}

/// Checks whether the maximal 128-bit SIMD instructions are supported.
/// 
/// Returns `Err` with [`FeatureDisabled`] when feature detection mechanism is required but is disabled.
/// Returns error value returned by [`detection_mechanism_available`] when it returns an error.
pub fn max_available() -> Result<FeatureState> {
	use Error::*;
	use FeatureState::*;

	target_arch_else_unimplemented_error!{
		["x86"] {
			/* SSE4.2 */

			match detection_mechanism_available() {
				Ok(Enabled) => {
					/*
					SSE: CPUID[1].D[25]
					SSE2: CPUID[1].D[26]
					SSE3: CPUID[1].C[0]
					SSSE3: CPUID[1].C[9]
					SSE4.1: CPUID[1].C[19]
					SSE4.2: CPUID[1].C[20]
					*/

					let (c, d): (u32, u32);

					unsafe { llvm_asm!("cpuid" : "={ecx}"(c), "={edx}"(d) : "{eax}"(1), "{ebx}"(0)); }

					if c & 0x180201 == 0x180201 && d >> 25 & 3 == 3 {
						#[cfg(not(feature="kernel_mode"))]
						{
							/*
							OSXSAVE: C[27]
							FPU/MMX: XCR0[0]
							SSE: XCR0[1]
							*/

							if c >> 27 & 1 == 1 {
								let result: u32;

								unsafe { llvm_asm!("xgetbv" : "={eax}"(result) : "{ecx}"(0), "{edx}"(0)); }

								if result & 3 == 3 {
									return Ok(Enabled);
								}
							}
							Err(OsInteractionRequired)
						}

						#[cfg(feature="kernel_mode")]
						{
							let (cr0, cr4): (usize, usize);

							unsafe { llvm_asm!("mov $0, cr0 \n mov $1, cr4" : "=r"(cr0), "=r"(cr4) ::: "intel"); }

							Ok(if cr0 & 6 == 6 && cr4 >> 9 & 3 == 3 { Enabled } else { Disabled })
						}
					} else { Err(Unavailable) }
				},
				Ok(Disabled) => Err(FeatureDisabled),
				error => error,
			}
		},
		["x86_64"] {
			/* SSE4.2 */

			/*
			SSE: CPUID[1].D[25]
			SSE2: CPUID[1].D[26]
			SSE3: CPUID[1].C[0]
			SSSE3: CPUID[1].C[9]
			SSE4.1: CPUID[1].C[19]
			SSE4.2: CPUID[1].C[20]
			*/

			let (c, d): (u32, u32);

			unsafe { llvm_asm!("cpuid" : "={ecx}"(c), "={edx}"(d) : "{eax}"(1), "{ebx}"(0)); }

			if c & 0x180201 == 0x180201 && d >> 25 & 3 == 3 {
				#[cfg(not(feature="kernel_mode"))]
				{
					/*
					OSXSAVE: C[27]
					FPU/MMX: XCR0[0]
					SSE: XCR0[1]
					*/

					if c >> 27 & 1 == 1 {
						let result: u32;

						unsafe { llvm_asm!("xgetbv" : "={eax}"(result) : "{ecx}"(0), "{edx}"(0)); }

						if result & 3 == 3 {
							return Ok(Enabled);
						}
					}
					Err(OsInteractionRequired)
				}

				#[cfg(feature="kernel_mode")]
				{
					let (cr0, cr4): (usize, usize);

					unsafe { llvm_asm!("mov $0, cr0 \n mov $1, cr4" : "=r"(cr0), "=r"(cr4) ::: "intel"); }

					Ok(if cr0 & 6 == 6 && cr4 >> 9 & 3 == 3 { Enabled } else { Disabled })
				}
			} else { Err(Unavailable) }
		}
	}
}
