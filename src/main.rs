#![no_std]
#![no_main]

mod cpu;
mod io;

// This import is very much used to indicate a panic handler
#[allow(unused_imports)]
use panic_halt;

use itsybitsy_m4::clock::GenericClockController;
use itsybitsy_m4::delay::Delay;
use itsybitsy_m4::entry;
use itsybitsy_m4::pac::{CorePeripherals, Peripherals};
use itsybitsy_m4::prelude::*;
use itsybitsy_m4::watchdog::{Watchdog, WatchdogTimeout};

use io::*;

#[entry]
fn main() -> ! {
    // Example blinky_led
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

    let wdt = Watchdog::new(peripherals.WDT);
    let pins = itsybitsy_m4::Pins::new(peripherals.PORT);

    event_loop(pins, wdt, delay)
}

fn event_loop(mut pins: itsybitsy_m4::Pins, mut wdt: Watchdog, _delay: Delay) -> ! {
    wdt.start(WatchdogTimeout::Cycles256 as u8);
    // Turns off the indicator until we need it
    let mut _indicator = pins.d13.into_open_drain_output(&mut pins.port);

    let buttons = hid::Buttons {
        a: pins.i2c_scl.into_pull_up_input(&mut pins.port),
        b: pins.d7.into_pull_up_input(&mut pins.port),
        up: pins.d10.into_pull_up_input(&mut pins.port),
        down: pins.d12.into_pull_up_input(&mut pins.port),
        left: pins.d11.into_pull_up_input(&mut pins.port),
        right: pins.d9.into_pull_up_input(&mut pins.port),
        menu: pins.i2c_sda.into_pull_up_input(&mut pins.port),
    };

    loop {
        // You can verify this works by doing something akin to
        // hid::Pressed::Up => _indicator.set_high().unwrap(),
        match buttons.pressed() {
            hid::Pressed::Up => {}
            hid::Pressed::Down => {}
            hid::Pressed::Left => {}
            hid::Pressed::Right => {}
            hid::Pressed::None => {}
            hid::Pressed::A => {}
            hid::Pressed::B => {}
            hid::Pressed::Menu => {}
        }
        wdt.feed();
    }
}
