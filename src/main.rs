#![no_std]
#![no_main]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt as rt;
// extern crate panic_itm;
extern crate panic_semihosting;
extern crate stm32f103xx_hal as hal;
extern crate stm32f103xx_usb;
extern crate usb_device;

mod cdc;

use hal::prelude::*;
use hal::stm32f103xx;
use rt::ExceptionFrame;

use stm32f103xx_usb::UsbBus;
use usb_device::prelude::*;

entry!(main);
fn main() -> ! {
    let dp = stm32f103xx::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr
        .hse(8.mhz())
        .sysclk(48.mhz())
        .pclk1(24.mhz())
        .freeze(&mut flash.acr);

    assert!(clocks.usbclk_valid());

    let usb_bus = UsbBus::usb(dp.USB, &mut rcc.apb1);

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    usb_bus
        .resetter(&clocks, &mut gpioa.crh, gpioa.pa12)
        .reset();

    let serial = cdc::SerialPort::new(&usb_bus.allocator());

    let usb_dev = UsbDevice::new(&usb_bus, UsbVidPid(0xeeee, 0xffff))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(cdc::USB_CLASS_CDC)
        .build(&[&serial]);

    loop {
        usb_dev.poll();

        if usb_dev.state() == UsbDeviceState::Configured {
            // let mut buf = [0u8; 8];

            // match serial.read(&mut buf) {
            //     Ok(count) if count > 0 => {
            //         // Echo back in upper case
            //         for c in buf[0..count].iter_mut() {
            //             if 0x61 <= *c && *c <= 0x7a {
            //                 *c &= !0x20;
            //             }
            //         }

            //         serial.write(&buf[0..count]).ok();
            //     }
            //     _ => {}
            // }

            serial.write(b"1001\n").ok();
        }
    }
}

exception!(HardFault, hard_fault);
fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

exception!(*, default_handler);
fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
