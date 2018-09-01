#![no_std]
#![no_main]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt as rt;
extern crate cortex_m_rtfm as rtfm;
extern crate cortex_m_rtfm_macros;
extern crate panic_semihosting;
extern crate stm32f103xx_hal as hal;
extern crate stm32f103xx_usb;
extern crate usb_device;

mod cdc;

use cortex_m_rtfm_macros::app;
use hal::prelude::*;
// use hal::stm32f103xx;
use cdc::SerialPort;
use hal::gpio::gpiob::PB9;
use hal::gpio::{Alternate, Floating, Input, Output, PushPull};
use rt::ExceptionFrame;
use rtfm::Threshold;
use stm32f103xx_usb::UsbBus;
use usb_device::prelude::*;

// Tasks and resources
app! {
    device: hal::stm32f103xx,

    resources: {
        // static USB: UsbDevice<'static, UsbBus>;
        static SERIAL: SerialPort<'static, UsbBus>;
        static INPUT_PIN: PB9<Input<Floating>>;
    },

    idle: {
        resources: [
            INPUT_PIN
        ],
    },

    tasks: {
        SYS_TICK: {
            path: tick,
            resources: [
                SERIAL
            ],
        },
    },
}

fn ent() -> ! {
    main();

    loop {}
}

entry!(ent);

fn init(p: init::Peripherals) -> init::LateResources {
    let mut flash = p.device.FLASH.constrain();
    let mut rcc = p.device.RCC.constrain();

    let clocks = rcc.cfgr
        .hse(8.mhz())
        .sysclk(48.mhz())
        .pclk1(24.mhz())
        .freeze(&mut flash.acr);

    assert!(clocks.usbclk_valid());

    let usb_bus = UsbBus::usb(p.device.USB, &mut rcc.apb1);

    let mut gpioa = p.device.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = p.device.GPIOB.split(&mut rcc.apb2);
    usb_bus
        .resetter(&clocks, &mut gpioa.crh, gpioa.pa12)
        .reset();

    let serial = SerialPort::new(&usb_bus.allocator());

    let usb_dev = UsbDevice::new(&usb_bus, UsbVidPid(0xeeee, 0xffff))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(cdc::USB_CLASS_CDC)
        .build(&[&serial]);

    let input_pin = gpiob.pb9.into_floating_input(&mut gpiob.crh);

    init::LateResources {
        // USB: usb_dev,
        SERIAL: serial,
        INPUT_PIN: input_pin,
    }
}

fn idle(_t: &mut Threshold, r: idle::Resources) -> ! {
    // let usb_dev: &'static mut UsbDevice = r.USB;
    // let serial: &'static mut SerialPort = r.SERIAL;
    // let input_pin: &'static mut u8 = r.INPUT_PIN;

    // loop {
    //     usb_dev.poll();

    //     if usb_dev.state() == UsbDeviceState::Configured {
    //         // let mut buf = [0u8; 8];

    //         // match serial.read(&mut buf) {
    //         //     Ok(count) if count > 0 => {
    //         //         // Echo back in upper case
    //         //         for c in buf[0..count].iter_mut() {
    //         //             if 0x61 <= *c && *c <= 0x7a {
    //         //                 *c &= !0x20;
    //         //             }
    //         //         }

    //         //         serial.write(&buf[0..count]).ok();
    //         //     }
    //         //     _ => {}
    //         // }
    //         if input_pin.is_high() {
    //             serial.write(b"255\n").ok();
    //         } else {
    //             serial.write(b"0\n").ok();
    //         }
    //     }

    //     rtfm::wfi();
    // }

    loop {}
}

fn tick(_t: &mut Threshold, mut r: SYS_TICK::Resources) {}

exception!(HardFault, hard_fault);
fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

exception!(*, default_handler);
fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
