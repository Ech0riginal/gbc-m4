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
    /// When we need to store a value at a memory location specified by a 16-bit register
    STR,
    /// When we need to grab the value specified by a 16-bit register into the 8-bit accumulator
    GRB,
    /// Zero flag
    Z,
    /// Not-zero flag
    NZ,
    /// Subtract flag
    S,
    /// Carry flag
    CY,
    /// Not-carry flag
    NC,
    /// Half-carry flag
    HCY,
    /// Tells the cpu that the registers given contain the value and the address to store to RAM
    RAM,
    /// Tells the cpu that the registers given contain the value and the address to store to VRAM
    VRAM,
}