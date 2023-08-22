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

// import panic handlers
// only use in debugging
#[cfg(debug_assertions)]
use panic_halt as _;

// only use in prod/release - i.e, end-user CoDi units
// we should handle bootloader failures better
#[cfg(not(debug_assertions))]
use panic_abort as _;

#[cfg_attr(target_arch = "arm", cortex_m_rt::entry)]
#[cfg(target_arch = "arm")]
fn main() -> ! {
    loop {}
}
