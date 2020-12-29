use crate::cpu::CPU;

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

/// The GBC's opcodes often imply behavior we define herein, pulled from @meganesu's work.
pub enum Flag {
    /// NOP, but for Flags
    NF,
    /// Zero flag
    Z,
    /// Not-zero flag
    NZ,
    /// Carry flag
    CY,
    /// Not-carry flag
    NC,
}

impl Flag {
    pub fn status(&self, cpu: &CPU) -> bool {
        match self {
            Self::NF => true,
            Self::Z => cpu.flag >> 7 == 1,
            Self::NZ => cpu.flag >> 7 == 0,
            Self::CY => cpu.flag >> 5 == 1,
            Self::NC => cpu.flag >> 5 == 0,
            _ => panic!("Bad status call: {:016b}: {:016b}", cpu.pc, cpu.sp)
        }
    }
}