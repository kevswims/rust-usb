#![no_std]
#![cfg_attr(test, no_main)]

use rust_usb as _; // memory layout + panic handler

#[defmt_test::tests]
mod tests {}
