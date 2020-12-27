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
    /// Accumulator register
    a: u8,
    /// CPU flag register, see the flag_register module.
    flag: u8,
    // 0 0 0 0 - trailing nibble isn't used
    // | | | carry
    // | | half carry
    // | subtraction
    // zero
    /// Registers
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    /// Program counter
    pc: u16,
    /// Stack pointer
    sp: u16,
    /// Memory bus, see the memory_bus module.
    bus: [u8; 65535],
}

impl CPU {
    pub fn new() -> Self {
        Self {
            pc: 0,
            sp: 0,
            bus: [0u8; 65535],
            a: 0x11,
            flag: 0x00,
            b: 0x00,
            c: 0x00,
            d: 0xFF,
            e: 0x56,
            h: 0x00,
            l: 0x0D,
        }
    }

    pub unsafe fn cycle(&mut self) {
        let (inst, prefixed) = {
            let tmp = self.bus.read_byte(self.pc);
            if tmp == 0xCB {
                (self.bus.read_byte(self.pc + 1), true)
            } else {
                (tmp, false)
            }
        };



    }

    pub unsafe fn execute(&mut self, inst: Instruction) {
        match inst {
            // super handy - https://meganesulli.com/generate-gb-opcodes/
            Instruction::ADD8(reg) => {
                let regi = self.getreg(reg);
                let (nv, did_overflow) = self.a.overflowing_add(*regi);

                self.flag.zero(nv == 0);
                self.flag.subtract(false);
                self.flag.carry(did_overflow);
                self.flag.half_carry((*regi & 0xF) + (nv & 0xF) > 0xF);

                self.a = nv;
                self.pc.wrapping_add(1);
            }
            Instruction::ADD16(reg) => {
                let a = *self.getreg(reg) as u16;
                let hl = self.getreg(Register::HL) as *mut u16;
                // luckily, this will lock us to 16 bits
                let (nv, did_overflow) = (*hl).overflowing_add(a);

                self.flag.zero(nv == 0);
                self.flag.subtract(false);
                self.flag.carry(did_overflow);
                self.flag.half_carry(((*hl ^ a ^ (nv & 0xffff)) & 0x1000) != 0);

                *hl = nv;
                self.pc.wrapping_add(1);
            }
            Instruction::ADC => {}
            Instruction::SUB(reg) => {}
            Instruction::SBC => {}
            Instruction::AND(reg) => {
                let nv = self.a & *self.getreg(reg);

                self.flag.zero(nv == 0);
                self.flag.subtract(false);
                self.flag.half_carry(true);
                self.flag.carry(false);

                self.a = nv;
                self.pc.wrapping_add(1);
            }
            Instruction::OR(reg) => {}
            Instruction::XOR(reg) => {}
            Instruction::CP => {}
            Instruction::INC => {}
            Instruction::DEC => {}
            Instruction::CCF => {}
            Instruction::SCF => {
                self.flag.carry(true);
                self.pc.wrapping_add(1);
            },
            Instruction::RRA => {}
            Instruction::RLA => {}
            Instruction::RRCA => {}
            Instruction::RRLA => {}
            Instruction::CPL => {}
            Instruction::BIT(bit, register) => {
                // TODO flag factory; this's ridiculous, or is it? investigate once cpu's done
                self.flag.zero((*self.getreg(register) >> bit & 0x01) == 0);
                self.flag.subtract(false);
                self.flag.half_carry(true);
            }
            Instruction::RES(bit, reg) => {
                let r = self.getreg(reg);
                *r = *regi & !(0x01 << bit);
            }
            Instruction::SET(bit, reg) => {
                let r = self.getreg(reg);
                *r = *r | !(0x01 << bit);
            }
            Instruction::SRL => {}
            Instruction::RR => {}
            Instruction::RL => {}
            Instruction::RRC => {}
            Instruction::RLC => {}
            Instruction::SRA(reg) => {
                *self.getreg(reg) >>= 1;
                self.pc.wrapping_add(1);
            },
            Instruction::SLA(reg) => {
                *self.getreg(reg) <<= 1;
                self.pc.wrapping_add(1);
            },
            Instruction::SWAP8(reg) => {
                let r = self.getreg(reg);
                *r = ((*r & 0x0F) << 4 ) | (( *r & 0xF0) >> 4);

                self.flag.zero(*r == 0);
                self.flag.subtract(false);
                self.flag.half_carry(false);
                self.flag.carry(false);

                self.pc.wrapping_add(1);
            }
            Instruction::SWAP16(reg) => {
                let r = self.getreg(reg) as *mut u16;

                *r = ((*r & 0x00FF) << 8 ) | (( *r & 0xFF00) >> 8);

                self.flag.zero(*r == 0);
                self.flag.subtract(false);
                self.flag.half_carry(false);
                self.flag.carry(false);
            }
        }
    }

    /// Returns a pointer to the specified Register, mainly so that we can
    /// use our `Instruction`s to abstract away the virtual registers
    /// Remember to cast a virtual register's pointer to it's 'length', 16
    ///
    /// # Examples
    /// ```no_run
    /// use cpu::{CPU, Register};
    ///
    /// let mut cpu = CPU::new();
    /// let mut hl_regi = cpu.get(Register::HL) as *mut u16;
    ///
    /// *hl_regi = 0b0000_0001_1010_0100;
    ///
    /// assert_eq!(*cpu.getreg(Register::H), 0b0000_0001);
    /// assert_eq!(*cpu.getreg(Register::L), 0b1010_0100);
    /// ```
    pub fn getreg(&mut self, r: Register) -> *mut u8 {
        match r {
            // Registers
            Register::A => &mut self.a,
            Register::B => &mut self.b,
            Register::C => &mut self.c,
            Register::D => &mut self.d,
            Register::E => &mut self.e,
            Register::F => &mut self.flag,
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
                self.flag = (v & 0xFF) as u8;
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
            _ => unsafe { *self.getreg(reg) = v as u8 },
        }
    }
}