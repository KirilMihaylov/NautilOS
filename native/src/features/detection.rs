pub fn detection_mechanism_present() -> bool {
	target_arch_else_unimplemented!{
		["x86", "x86_64"] {
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

			return ((flags ^ updated_flags) >> 21) & 1 == 1
		}
	}
}
