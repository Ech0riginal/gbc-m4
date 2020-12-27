// LR35902 ulator

mod registers;

// CPU flag positionsz
const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

enum Register {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
    AF,
    BC,
    DE,
    HL,
}
enum Instruction {
    ADD(Register, Register),

}


type MemoryBus = [u8; 65535];

trait Busd {
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

#[repr(packed)]
#[derive(Debug)]
struct CPU {
    // Program counter
    pc: u16,
    // Memory bus
    bus: MemoryBus,
    // Registers
    a: u8,
    f: u8,
    //  f: cpu flag register
    // 0 0 0 0 0 0 0 0
    // | | | carry
    // | | half carry
    // | subtraction
    // zero
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            pc: 0,
            bus: [0u8; 65535],
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0
        }
    }

    pub fn get_register(&mut self, r: Register) -> *mut u8 {
        match r {
            // Registers
            Register::A => &mut self.a,
            Register::B => &mut self.b,
            Register::C => &mut self.c,
            Register::D => &mut self.d,
            Register::E => &mut self.e,
            Register::F => &mut self.f,
            Register::H => &mut self.h,
            Register::L => &mut self.l,
            // Virtual registers
            Register::AF => &mut self.a,
            Register::BC => &mut self.b,
            Register::DE => &mut self.d,
            Register::HL => &mut self.h,
        }
    }

    pub fn execyt(
        &mut self,
        v: u16,
        reg: Register,
        ins: Instruction,
    ) {
        match reg {
            // if i'm correct, since m4's don't have mmu, we should be able
            // to overflow into the next byte when we assign a 16 bit value
            // to the first register's addresses, but lets get this working first
            Register::AF => {
                self.a = ((v & 0xFF00) >> 8) as u8;
                self.f = (v & 0xFF) as u8;
            }
            Register::BC => {
                self.b = ((v & 0xFF00) >> 8) as u8;
                self.c = (v & 0xFF) as u8;
            }
            Register::DE => {
                self.d = ((v & 0xFF00) >> 8) as u8;
                self.e = (v & 0xFF) as u8;
            }
            Register::HL => {
                self.h = ((v & 0xFF00) >> 8) as u8;
                self.l = (v & 0xFF) as u8;
            }
            _ => unsafe { *self.get_register(reg) = v as u8 },
        }
    }
}