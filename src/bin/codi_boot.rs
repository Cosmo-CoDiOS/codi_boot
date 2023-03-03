#![cfg_attr(any(target_arch = "arm"), no_main, no_std)]

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
