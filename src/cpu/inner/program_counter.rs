pub type ProgramCounter = u16;

trait Countd {
    fn d8(&self) -> u8;
    fn d16(&self) -> u16;

    fn push8(&self) -> u8;
    fn push16(&self) -> u16;

    fn pop8(&self) -> u8;
    fn pop16(&self) -> u16;
}

impl Countd for ProgramCounter {
    fn d8(&self) -> u8 {
        unimplemented!()
    }

    fn d16(&self) -> u16 {
        unimplemented!()
    }

    fn push8(&self) -> u8 {
        unimplemented!()
    }

    fn push16(&self) -> u16 {
        unimplemented!()
    }

    fn pop8(&self) -> u8 {
        unimplemented!()
    }

    fn pop16(&self) -> u16 {
        unimplemented!()
    }
}

