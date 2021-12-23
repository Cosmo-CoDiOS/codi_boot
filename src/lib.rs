#![deny(
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
#![no_std]

pub mod consts {
    //! This module defines constants for various areas of the bootloader.
   
    /// Where the bootloader starts on the STM32 chip.
    pub const BOOTLOADER_START: u32 = 0x08000000;

    /// Where the bootloader ends on the STM32 chip.
    pub const BOOTLOADER_END: u32 = 0x08080000;
}

