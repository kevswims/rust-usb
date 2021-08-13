#![no_main]
#![no_std]

use rust_usb as _; // global logger + panicking-behavior + memory layout

use stm32f4xx_hal::{prelude::*, stm32};
use cortex_m::asm::nop;

#[cortex_m_rt::entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();

    let _clocks = rcc
        .cfgr
        .use_hse(8.mhz())
        .sysclk(48.mhz())
        .pclk1(24.mhz())
        .pclk2(24.mhz())
        .freeze();

    // Configure the on-board RED LED (PB14)
    let gpiob = dp.GPIOB.split();
    let mut led = gpiob.pb14.into_push_pull_output();

    // Turn the LED off
    led.set_low().unwrap();

    defmt::info!("Hello, world!");

    loop {
        nop();
    }
}
