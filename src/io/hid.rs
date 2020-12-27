use atsamd_hal::prelude::*;
use atsamd_hal::gpio::*;

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
    pub(crate) a: Pa13<Input<PullUp>>,
    pub(crate) b: Pa18<Input<PullUp>>,
    pub(crate) up: Pa20<Input<PullUp>>,
    pub(crate) down: Pa23<Input<PullUp>>,
    pub(crate) left: Pa21<Input<PullUp>>,
    pub(crate) right: Pa19<Input<PullUp>>,
    pub(crate) menu: Pa12<Input<PullUp>>,
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