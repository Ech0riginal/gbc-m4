pub type MemoryBus = [u8; 65535];

pub trait Busd {
    fn read_byte(&self, address: u16) -> u8;
    fn new() -> Self;
}

impl Busd for MemoryBus {
    fn read_byte(&self, address: u16) -> u8 {
        self[address as usize]
    }

    fn new() -> Self {
        [0u8; 65535]
    }
}