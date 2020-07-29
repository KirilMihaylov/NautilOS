#![no_std]
#![allow(dead_code)]
#![doc(html_no_source)]
/* Enables 'extern "efiapi"' */
#![feature(abi_efiapi)]
/* Disables warning for trait specialization */
#![allow(incomplete_features)]
/* Enables trait specialization */
#![feature(specialization)]

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
pub mod protocols;
pub mod runtime_services;
