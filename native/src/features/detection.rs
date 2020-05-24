//! Provides methods for detecting, enabling and disabling specific features.

use crate::result::Result;

/// Checks whether there is available feature detection mechanism.
/// 
/// It returns `Ok(true)` when there is available mechanism. Returns `Ok(false)` when no error occured and there is no available mechanism. Returns `Err` when an error occured while checking.
/// 
/// # Notes
/// On IA-32 (x86) and AMD64 (x86_64) it should not return `Err`.
pub fn detection_mechanism_available() -> Result<bool> {
	target_arch_else_unimplemented_error!{
		["x86", "x86_64"] {
			use core::sync::atomic::{
				AtomicBool,
				Ordering::Relaxed,
			};

			static DETECTION_MECHANISM: AtomicBool = AtomicBool::new(false);

			if DETECTION_MECHANISM.load(Relaxed) { Ok(true) }
			else {
				let flags: usize;

				unsafe {
					llvm_asm!(
						"
						pushf
						pop $0
						" :
						"=r"(flags)
					);
				}

				let updated_flags: usize;

				unsafe {
					llvm_asm!(
						"
						push $1
						popf

						pushf
						pop $0
						" :
						"=r"(updated_flags) :
						"r"(flags ^ (1usize << 21))
					);
				}

				(|value: bool| -> Result<bool> {
					DETECTION_MECHANISM.store(value, Relaxed);

					Ok(value)
				})(((flags ^ updated_flags) >> 21) & 1 == 1)
			}
		}
	}
}
