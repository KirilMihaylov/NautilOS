#![no_std]
#![allow(dead_code)]

/* For platform-specific operations */
#![feature(llvm_asm)]

#[cfg(feature="memory_c")]
mod memory_c;
