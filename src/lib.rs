//! This crate houses a new and improved bootloader for `CoDi`.
#![deny(
    warnings,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::cargo,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    variant_size_differences
)]
#![allow(unsafe_code)]
#![allow(dead_code)]
#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use core2::io;

pub mod consts {
    //! This module defines constants for various areas of the bootloader.

    /// Where the bootloader starts on the STM32 chip.
    pub const BOOTLOADER_START: u32 = 0x0800_0000;

    /// Where the bootloader ends on the STM32 chip.
    pub const BOOTLOADER_END: u32 = 0x0808_0000;
}

// derived from https://github.com/karthickai/rustboot/blob/master/src/main.rs
// thanks

/// Errors defined in base crate for bootload.
#[derive(Debug)]
#[allow(missing_docs)]
pub enum Error {
    Success,
    InvalidAddr(u32),
    PayloadTooLong(usize),
    PayloadLengthErr(usize),
    EraseErr(Box<io::Error>),
    WriteErr(Box<io::Error>),
    FlashErr,
    InternalErr,
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::WriteErr(Box::new(e))
    }
}
