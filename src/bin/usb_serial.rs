#![no_main]
#![no_std]

use rust_usb as _; // global logger + panicking-behavior + memory layout

use stm32f4xx_hal::{prelude::*, stm32};

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

    

    defmt::info!("Hello, world!");

    rust_usb::exit()
}
