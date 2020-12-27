pub type FlagRegister = u8;

pub trait Flagd {
    fn zero(&mut self, b: bool);
    fn subtract(&mut self, b: bool);
    fn half_carry(&mut self, b: bool);
    fn carry(&mut self, b: bool);
}

impl Flagd for FlagRegister {
    fn zero(&mut self, b: bool) {
        if b {
            *self |= 0b1000_0000
        } else {
            *self &= 0b0111_1111
        }
    }

    fn subtract(&mut self, b: bool) {
        if b {
            *self |= 0b0100_0000
        } else {
            *self &= 0b1011_1111
        }
    }

    fn half_carry(&mut self, b: bool) {
        if b {
            *self |= 0b0010_0000
        } else {
            *self &= 0b1101_1111
        }
    }

    fn carry(&mut self, b: bool) {
        if b {
            *self |= 0b0001_0000
        } else {
            *self &= 0b1110_1111
        }
    }
}