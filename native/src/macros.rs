#![allow(unused_macros)]

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

macro_rules! not_target_arch {
	($([$($targets:literal),+] $block:block),+) => {
		$(
			#[cfg(not(any($(target_arch=$targets),+)))]
			$block
		)+
	};
}

macro_rules! target_arch_else_unimplemented {
	($([$($targets:literal),+] $block:block),+) => {
		$(
			target_arch!([$($targets),+] $block);
		)+
		if_not_target_unimplemented!($($($targets),+),+);
	};
}

macro_rules! if_not_target_unimplemented {
	($($targets:literal),+) => {
		not_target_arch!([$($targets),+] { unimplemented!(); });
	};
}
