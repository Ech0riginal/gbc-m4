pub type ProgramCounter = u16;

trait Countd {
    fn d8(&mut self) -> u8;
    fn d16(&mut self) -> u16;
}

impl Countd for ProgramCounter {
    // TODO rename these to make sense once i figure out megan's docs
    /// Loads the first byte of immediate data in the program counter and increments
    /// the program counter before returning the value.
    fn d8(&mut self) -> u8 {
        let v = (*self >> 8 & 0x00FF) as u8;
        let _ = self.wrapping_add(1);
        v
    }

    /// Loads the first two bytes of immediate data in the program counter
    /// and increments the counter before returning the value.
    fn d16(&mut self) -> u16 {
        let low = self.d8() as u16;
        let high = self.d8() as u16;
        (high << 8) | low
    }
}

