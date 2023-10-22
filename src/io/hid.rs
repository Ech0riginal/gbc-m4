use itsybitsy_m4::hal::prelude::*;
use itsybitsy_m4::hal::gpio::v2::*;

use itsybitsy_m4;
use itsybitsy_m4::Pins;
use itsybitsy_m4::hal::pac::Peripherals;
use itsybitsy_m4::pac::PORT;

pub enum Pressed {
    Up,
    Down,
    Left,
    Right,
    None,
    A,
    B,
    Menu,
}

pub struct Buttons {
    // pub(crate) a: Pin<PA22, atsamd_hal::gpio::v2::Input<atsamd_hal::gpio::v2::PullUp>>,
    pub a: Pin<PA13, Input<PullUp>>,
    pub b: Pin<PA18, Input<PullUp>>,
    pub up: Pin<PA20, Input<PullUp>>,
    pub down: Pin<PA23, Input<PullUp>>,
    pub left: Pin<PA21, Input<PullUp>>,
    pub right: Pin<PA19, Input<PullUp>>,
    pub menu: Pin<PA12, Input<PullUp>>,
}

impl Buttons {
    pub fn pressed(&self) -> Pressed {
        if self.up.is_low().unwrap_or(false) {
            Pressed::Up
        } else if self.down.is_low().unwrap_or(false) {
            Pressed::Down
        } else if self.right.is_low().unwrap_or(false) {
            Pressed::Right
        } else if self.left.is_low().unwrap_or(false) {
            Pressed::Left
        } else if self.a.is_low().unwrap_or(false) {
            Pressed::A
        } else if self.b.is_low().unwrap_or(false) {
            Pressed::B
        } else if self.menu.is_low().unwrap_or(false) {
            Pressed::Menu
        } else {
            Pressed::None
        }
    }
}
