#![allow(unused_macros)]

macro_rules! supported_targets {
	($([$($($condition:tt)+),+]),+) => {
		#[cfg(not(any($(all($($($condition)+),+)),+)))]
		compile_error!("Compiling for unsupported target!");
	};
}

macro_rules! any_target_arch {
	([$target:literal] $block:block) => {
		#[cfg(target_arch=$target)]
		$block
	};
	([$($targets:literal);+, $target:literal] $block:block) => {
		#[cfg(any(target_arch=$target,$(target_arch=$targets),+))]
		$block
	};
}

macro_rules! any_target_arch_else_unimplemented {
	([$target:literal] $block:block) => {
		#[cfg(target_arch=$target)]
		$block
		
		if_not_target_unimplemented!($target);
	};
	([$($targets:literal);+, $target:literal] $block:block) => {
		#[cfg(any(target_arch=$target,$(target_arch=$targets),+))]
		$block
		
		if_not_target_unimplemented!($target,$($targets),+);
	};
}

macro_rules! if_not_target_unimplemented {
	(target:literal) => {
		#[cfg(target_arch=$target)]
		unimplemented!();
	};
	($($targets:literal);+, $target:literal) => {
		#[cfg(not(any(target_arch=$target,$(target_arch=$targets),+)))]
		unimplemented!();
	};
}
