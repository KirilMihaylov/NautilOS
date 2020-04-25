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
		
		#[cfg(not(target_arch=$target))]
		unimplemented!();
	};
	([$($targets:literal);+, $target:literal] $block:block) => {
		#[cfg(any(target_arch=$target,$(target_arch=$targets),+))]
		$block
		
		#[cfg(not(any(target_arch=$target,$(target_arch=$targets),+)))]
		unimplemented!();
	};
}
