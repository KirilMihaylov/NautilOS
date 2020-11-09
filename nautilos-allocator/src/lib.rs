#![cfg_attr(not(test), no_std)]
#![allow(clippy::question_mark)]
#![forbid(warnings, clippy::pedantic)]
#![feature(const_alloc_layout, const_fn_fn_ptr_basics, const_mut_refs)]
#![cfg_attr(feature = "alloc_impl", feature(alloc_error_handler))]

pub(crate) mod heap;
pub(crate) mod heap_error;
pub(crate) mod list;
pub(crate) mod memory_range;
pub(crate) mod sorted_list;

pub use {
    heap::{init_in_place::initialization_error::InitializationError, Heap},
    heap_error::HeapError,
};

#[cfg(feature = "alloc_impl")]
mod alloc_impl;

#[cfg(feature = "alloc_impl")]
pub use alloc_impl::{initialize, AllocatorInitializationError};
