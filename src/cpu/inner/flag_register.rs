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
    /// Half-carry flag
    HC,
    /// Subtract flag
    S,
}

pub trait Flagd {
    fn status(&self, f: Flag) -> bool;
    fn zero(&mut self, b: bool);
    fn subtract(&mut self, b: bool);
    fn half_carry(&mut self, b: bool);
    fn carry(&mut self, b: bool);
    fn set_flags(&mut self, zero: bool, subtract: bool, half_carry: bool, carry: bool);
}
