// LR35902 ulator

mod inner;

use inner::*;

// CPU flag positionsz
const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

// https://github.com/nekronos/gbc_rs/blob/master/src/gbc/interconnect.rs

#[repr(packed(8))]
struct CPU {
    // Accumulator register
    a: u8,
    // CPU flag register
    f: FlagRegister,
    // 0 0 0 0 - trailing nibble isn't used
    // | | | carry
    // | | half carry
    // | subtraction
    // zero
    // Registers
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    // Program counter
    pc: u16,
    // Stack pointer
    sp: u16,
    // Memory bus
    bus: MemoryBus,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            pc: 0,
            sp: 0,
            bus: [0u8; 65535],
            a: 0x11,
            f: 0x00,
            b: 0x00,
            c: 0x00,
            d: 0xFF,
            e: 0x56,
            h: 0x00,
            l: 0x0D,
        }
    }

    pub unsafe fn execute(&mut self, inst: Instruction) {
        match inst {
            // super handy - https://meganesulli.com/generate-gb-opcodes/
            Instruction::ADD8(reg) => {
                let regi = self.get_register(reg);
                let (nv, did_overflow) = self.a.overflowing_add(*regi);

                self.f.zero(nv == 0);
                self.f.subtract(false);
                self.f.carry(did_overflow);
                self.f.half_carry((*regi & 0xF) + (nv & 0xF) > 0xF);

                self.a = nv;
            }
            Instruction::ADD16(reg) => {
                let a = *self.get_register(reg) as u16;
                let hl = self.get_register(Register::HL) as *mut u16;
                // luckily, this will lock us to 16 bits
                let (nv, did_overflow) = (*hl).overflowing_add(a);

                self.f.zero(*hl == 0);
                self.f.subtract(false);
                self.f.carry(did_overflow);
                self.f.half_carry(((*hl ^ a ^ (nv & 0xffff)) & 0x1000) != 0);

                *hl = nv;
            }

            Instruction::ADC => {}
            Instruction::SUB(reg) => {}
            Instruction::SBC => {}
            Instruction::AND(reg) => {
                let nv = self.a & *self.get_register(reg);

                self.a.zero(nv == 0);
                self.a.subtract(false);
                self.a.half_carry(true);
                self.a.carry(false);

                self.a = nv;
            }
            Instruction::OR(reg) => {}
            Instruction::XOR(reg) => {}
            Instruction::CP => {}
            Instruction::INC => {}
            Instruction::DEC => {}
            Instruction::CCF => {}
            Instruction::SCF => self.f.carry(true),
            Instruction::RRA => {}
            Instruction::RLA => {}
            Instruction::RRCA => {}
            Instruction::RRLA => {}
            Instruction::CPL => {}
            Instruction::BIT => {}
            Instruction::RES => {}
            Instruction::SET => {}
            Instruction::SRL => {}
            Instruction::RR => {}
            Instruction::RL => {}
            Instruction::RRC => {}
            Instruction::RLC => {}
            Instruction::SRA(reg) => {
                *self.get_register(reg) >>= 1
            },
            Instruction::SLA(reg) => {
                *self.get_register(reg) <<= 1
            },
            Instruction::SWAP(reg) => {
                let r = self.get_register(reg);
                *r = ((*r & 0x0f) << 4 ) | (( *r & 0xf0) >> 4)
            }
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

    pub fn set_register(
        &mut self,
        reg: Register,
        v: u16,
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