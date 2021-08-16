#![no_main]
#![no_std]

use rust_usb as _; // global logger + panicking-behavior + memory layout

use stm32f4xx_hal::{
    otg_fs::{UsbBus, USB},
    prelude::*,
    stm32,
};
use usb_device::{class::ControlOut, prelude::*};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

static mut EP_MEMORY: [u32; 1024] = [0; 1024];

#[cortex_m_rt::entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();

    let clocks = rcc
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

    let gpioa = dp.GPIOA.split();

    let usb_dm = gpioa.pa11.into_alternate_af10();
    let usb_dp = gpioa.pa12.into_alternate_af10();

    let usb = USB {
        usb_global: dp.OTG_FS_GLOBAL,
        usb_device: dp.OTG_FS_DEVICE,
        usb_pwrclk: dp.OTG_FS_PWRCLK,
        pin_dm: usb_dm,
        pin_dp: usb_dp,
        hclk: clocks.hclk(),
    };

    let usb_bus = UsbBus::new(usb, unsafe { &mut EP_MEMORY });

    // let mut serial = SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("Fake Company")
        .product("Serial Port")
        .serial_number("TEST")
        .device_class(0xFF)
        .build();

    let mut custom_class = CustomClass {};

    loop {
        if !usb_dev.poll(&mut [&mut custom_class]) {
            continue;
        }

        // let mut buffer = [0u8; 64];

        // match serial.read(&mut buffer) {
        //     Ok(count) if count > 0 => {
        //         // Turn on the LED
        //         led.set_high().unwrap();

        //         // Echo back in upper case
        //         for c in buffer[0..count].iter_mut() {
        //             if 0x61 <= *c && *c <= 0x7a {
        //                 *c &= !0x20;
        //             }
        //         }

        //         let mut write_offset = 0;
        //         while write_offset < count {
        //             match serial.write(&buffer[write_offset..count]) {
        //                 Ok(len) if len > 0 => {
        //                     write_offset += len;
        //                 }
        //                 _ => {}
        //             }
        //         }
        //     }
        //     _ => {}
        // }

        // Turn off the LED
        led.set_low().unwrap();
    }
}

struct CustomClass {

}

use usb_device::class_prelude::*;

impl<B: usb_device::bus::UsbBus> usb_device::class::UsbClass<B> for CustomClass {

    fn control_out(&mut self, xfer: ControlOut<B>) {
        let req = *xfer.request();

        defmt::info!("Hello from control transfer");
        if !(req.request_type == control::RequestType::Vendor && req.recipient == control::Recipient::Device)
        {
            return;
        }
    }

    // fn get_configuration_descriptors(&self, writer: &mut DescriptorWriter) -> Result<()> {
    //     writer.interface()
    // }
}