#![no_std]
#![no_main]

// import panic handlers
// only use in debugging
#[cfg(debug_assertions)]
use panic_halt as _;

// only use in prod/release - i.e, end-user CoDi units
#[cfg(not(debug_assertions))]
use panic_abort as _;
