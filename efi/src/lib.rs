#![no_std]
#![allow(dead_code)]
#![doc(html_no_source)]
#![forbid(warnings)]
/* Enables 'extern "efiapi"' */
#![feature(abi_efiapi)]
/* Enables trait specialization */
#![feature(min_specialization)]
/* Enables usage of "core::mem::transmute" in "const fn"s */
#![feature(const_fn_transmute)]
/* Enables "!" (never) type */
#![feature(never_type)]

pub mod utilities;

mod types;
pub use types::*;

mod status;
pub use status::*;

mod guid;
pub use guid::*;

mod table_header;
pub use table_header::*;

mod configuration_table;
pub use configuration_table::*;

mod system_table;
pub use system_table::*;

pub mod boot_services;
pub mod guids;
pub mod protocols;
pub mod runtime_services;
pub mod structures;
