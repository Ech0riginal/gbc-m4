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

#[repr(packed)]
// packing so that we can
struct CPU {
    pub a: u8,
    pub f: u8,
    //  f: cpu flag register
    // 0 0 0 0 0 0 0 0
    // | | | carry
    // | | half carry
    // | subtraction
    // zero
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
}

impl CPU {
    pub fn new() -> Self {
        Self {
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

    pub fn get_register(&self, r: Register) -> *const u8 {
        match r {
            Register::A => &*self.a,
            Register::B => &*self.b,
            Register::C => &*self.c,
            Register::D => &*self.d,
            Register::E => &*self.e,
            Register::F => &*self.f,
            Register::H => &*self.h,
            Register::L => &*self.l,
            // if i'm correct, we should be able to overflow into the next register when
            // we assign a 16 bit value to these addresses
            Register::AF => &*self.a,
            Register::BC => &*self.b,
            Register::DE => &*self.d,
            Register::HL => &*self.h,
        }
    }

    pub fn set_register(&mut self, r: Register, v: u16) {
        match r {
            Register::A => self.a = ((v & 0xFF00) >> 8) as u8,
            Register::F => self.f = ((v & 0xFF00) >> 8) as u8,
            Register::B => self.b = ((v & 0xFF00) >> 8) as u8,
            Register::C => self.c = ((v & 0xFF00) >> 8) as u8,
            Register::D => self.d = ((v & 0xFF00) >> 8) as u8,
            Register::E => self.e = ((v & 0xFF00) >> 8) as u8,
            Register::H => self.h = ((v & 0xFF00) >> 8) as u8,
            Register::L => self.l = ((v & 0xFF00) >> 8) as u8,
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
        }
    }
}