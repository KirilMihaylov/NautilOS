macro_rules! supported_targets {
	($([$($($condition:tt)+),+]),+) => {
		#[cfg(not(any($(all($($($condition)+),+)),+)))]
		compile_error!("Compiling for unsupported target!");
	};
}

macro_rules! target_arch {
	($([$($targets:literal),+] $block:block),+) => {
		$(
			#[cfg(any($(target_arch=$targets),+))]
			$block
		)+
	};
}

macro_rules! global_target_arch {
	($([$($targets:literal),+] $item:item)+) => {
		$(
			#[cfg(any($(target_arch=$targets),+))]
			$item
		)+
	};
}

macro_rules! not_target_arch {
	($([$($targets:literal),+] $block:block),+) => {
		$(
			#[cfg(not(any($(target_arch=$targets),+)))]
			$block
		)+
	};
}

macro_rules! global_not_target_arch {
	($([$($targets:literal),+] $item:item)+) => {
		$(
			#[cfg(not(any($(target_arch=$targets),+)))]
			$item
		)+
	};
}

macro_rules! global_not_target_arch_generic {
	($([$($targets:literal),+] $($generic:tt)+)+) => {
		$(
			#[cfg(not(any($(target_arch=$targets),+)))]
			$($generic)+
		)+
	};
}

macro_rules! target_arch_else {
	($([$($targets:literal),+] $block:block),+ else $else_block:block) => {
		$(
			target_arch!{
				[$($targets),+]
				$block
			}
		)+
		not_target_arch!{
			[$($($targets),+),+]
			$else_block
		}
	};
}

macro_rules! global_target_arch_else {
	($([$($targets:literal),+] $item:item)+ else $($else:tt)+) => {
		$(
			global_target_arch!{
				[$($targets),+]
				$item
			}
		)+
		global_not_target_arch_generic!{
			[$($($targets),+),+]
			$($else)+
		}
	};
}

macro_rules! global_target_arch_else_error {
	($([$($targets:literal),+] $item:item)+ else $error:literal) => {
		$(
			global_target_arch!{
				[$($targets),+]
				$item
			}
		)+
		global_not_target_arch_generic!{
			[$($($targets),+),+]
			compile_error!($error);
		}
	};
}

macro_rules! target_arch_else_unimplemented {
	($([$($targets:literal),+] $block:block),+) => {
		target_arch_else! {
			$(
				[$($targets),+]
				$block
			),+
			else {
				unimplemented!();
			}
		}
	};
}

macro_rules! target_arch_else_error {
	($([$($targets:literal),+] $block:block),+ else $error:literal) => {
		target_arch_else! {
			$(
				[$($targets),+]
				$block
			),+
			else {
				compile_error!($error);
			}
		}
	};
}

macro_rules! target_arch_else_unimplemented_error {
	($([$($targets:literal),+] $block:block),+) => {
		target_arch_else_error! {
			$(
				[$($targets),+]
				$block
			),+
			else "Error: Unimplemented for current target!"
		}
	};
}

macro_rules! if_not_target_unimplemented {
	($($targets:literal),+) => {
		not_target_arch!{
			[$($targets),+]
			unimplemented!();
		}
	};
}
