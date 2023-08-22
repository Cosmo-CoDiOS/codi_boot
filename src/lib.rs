//! This crate houses a new and improved bootloader for `CoDi`.
#![cfg_attr(target_arch = "arm", no_main, no_std)]
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
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    variant_size_differences
)]
#![allow(unsafe_code, dead_code)]

#[cfg(not(target_arch = "arm"))]
compile_error!("We are not instructed to build for the STM32. Aborting build!");

use core2::io;
use thiserror_no_std::Error;
use anyhow::Result;

pub type CoDiBootError<T> = Result<T, Error>;

pub mod consts {
    //! This module defines constants for various areas of the bootloader.

    /// Where the bootloader starts on the STM32 chip.
    pub const BOOTLOADER_START: u32 = 0x0800_0000;

    /// Where the bootloader ends on the STM32 chip.
    pub const BOOTLOADER_END: u32 = 0x0808_0000;
}

// derived from https://github.com/karthickai/rustboot/blob/master/src/main.rs
// thanks

/// Errors defined in base crate for bootloader.
#[derive(Debug, Default, Error)]
#[allow(missing_docs)]
pub enum Error {
    InvalidAddr {
        mem_addr: i16,
    },
    PayloadLengthErr {
        size: usize,
        too_long: bool,
        too_short: bool,
    },
    IoErr(#[from] io::Error),
    FlashErr,
    InternalErr,
    #[default]
    UnknownErr,
}
