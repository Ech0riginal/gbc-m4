#![no_std]
#![no_main]
#![recursion_limit = "1024"]
#![cfg_attr(debug_assertions, allow(unused_imports))]
#[allow(arithmetic_overflow)]
#[allow(overflowing_literals)]

use panic_halt as _;

// mod cpu;
// mod cart;
mod io;
// mod mmu;


use itsybitsy_m4 as bsp;
use bsp::hal;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::watchdog::{Watchdog, WatchdogTimeout};
// use hal::gpio::v2::Pins;

use io::hid;
use crate::io::hid::{Buttons, Pressed};

#[entry]
fn main() -> ! {
    let core = CorePeripherals::take().unwrap();
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut delay = Delay::new(core.SYST, &mut clocks);
    delay.delay_ms(400u16);

    let mut pins = bsp::Pins::new(peripherals.PORT);
    // let mut red_led = pins.d13.into_push_pull_output();
    let mut wdt = Watchdog::new(peripherals.WDT);
    wdt.start(WatchdogTimeout::Cycles256 as u8);

    // Turns off the indicator until we need it
    let mut _indicator = pins.d13.into_push_pull_output();

    let btns = Buttons {
        a: pins.scl.into_pull_up_input(),
        b: pins.d7.into_pull_up_input(),
        up: pins.d10.into_pull_up_input(),
        down: pins.d12.into_pull_up_input(),
        left: pins.d11.into_pull_up_input(),
        right: pins.d9.into_pull_up_input(),
        menu: pins.sda.into_pull_up_input(),
    };

    loop {
        delay.delay_ms(7u8);
        _ = match btns.pressed() {
            Pressed::None => _indicator.set_low(),
            _ => _indicator.set_high()
        };
        wdt.feed();
    }
}