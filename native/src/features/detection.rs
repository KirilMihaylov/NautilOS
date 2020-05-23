pub fn detection_mechanism_present() -> bool {
	target_arch_else_unimplemented_error!{
		["x86", "x86_64"] {
			use core::sync::atomic::{
				AtomicBool,
				Ordering::Relaxed,
			};

			static DETECTION_MECHANISM: AtomicBool = AtomicBool::new(false);

			if DETECTION_MECHANISM.load(Relaxed) { true }
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

				|value: bool| -> bool {
					DETECTION_MECHANISM.store(value, Relaxed);

					value
				}(((flags ^ updated_flags) >> 21) & 1 == 1)
			}
		}
	}
}
