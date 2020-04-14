#![no_std]
#![allow(dead_code)]

/* Enables 'extern "efiapi"' */
#![feature(abi_efiapi)]

pub mod common;

pub mod types;
pub mod status;
pub mod guid;
mod table_header;

pub mod boot_services;
pub mod protocols;
