// LR35902 ulator

#[cfg(test)]
mod tests;

mod inner;

use core::ops::Sub;
use inner::*;
use core::ptr::read;

// CPU flag positionsz
const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

const BUS_SIZE: usize = 65535;
const RAM_SIZE: usize = 1024 * 32;
const VRAM_SIZE: usize = 0x7F;

// https://github.com/nekronos/gbc_rs/blob/master/src/gbc/interconnect.rs

#[repr(C)]
pub(crate) struct CPU {
    /// Accumulator register
    a: u8,
    /// CPU flag register, see inner/flag_register.rs
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
    halted: bool,
    /// Program counter
    pc: u16,
    /// Stack pointer, see inner/program_counter.rs
    sp: u16,
    svbk: u8,
    ppu_dma: u8,
    int_enable: u8,
    int_flags: u8,
    ram_offset: usize,
    /// Memory bus, see inner/memory_bus.rs
    bus: [u8; 65535],
    ram: [u8; RAM_SIZE],
    vram: [u8; VRAM_SIZE],
}

impl CPU {
    pub fn new() -> Self {
        Self {
            a: 0x11,
            flag: 0x00,
            b: 0x00,
            c: 0x00,
            d: 0xFF,
            e: 0x56,
            h: 0x00,
            l: 0x0D,
            halted: false,
            pc: 0,
            sp: 0,
            svbk: 0,
            ppu_dma: 0,
            int_enable: 0,
            int_flags: 0,
            ram_offset: 0,
            bus: [0u8; BUS_SIZE],
            ram: [0u8; RAM_SIZE],
            vram: [0u8; VRAM_SIZE],
        }
    }

    // Sue me
    pub unsafe fn cycle(&mut self) {
        let inst = self.step();

        self.execute(inst);
    }

    fn step(&mut self) -> Instruction {
        let (byte, prefixed) = {
            let tmp = self.bus.read_byte(self.pc);
            if tmp == 0xCB {
                (self.bus.read_byte(self.pc + 1), true)
            } else {
                (tmp, false)
            }
        };

        Instruction::from_memory(prefixed, byte)
    }

    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7fff => unimplemented!("{}", addr),
            0x8000..=0x9fff => unimplemented!("{}", addr),
            0xa000..=0xbfff => unimplemented!("{}", addr),
            0xc000..=0xcfff => unimplemented!("{}", addr),
            0xd000..=0xdfff => unimplemented!("{}", addr),
            0xe000..=0xfdff => self.read(addr - 0xE000 + 0xC000),

            0xff00 => unimplemented!("{}", addr),

            0xff01..=0xff02 => {
                // serial IO
                unimplemented!("{}", addr)
            }
            0xff04..=0xff07 => unimplemented!("{}", addr),

            0xff10..=0xff3f => unimplemented!("{}", addr),

            0xff0f => unimplemented!("{}", addr),

            0xff46 => unimplemented!("{}", addr),

            0xfe00..=0xfeff | 0xff40..=0xff45 | 0xff47..=0xff4b | 0xff68..=0xff69 | 0xff4f => {
                unimplemented!("{}", addr)
            }

            0xff4d => 0, // Speedswitch
            0xff70 => unimplemented!("{}", addr),
            0xff80..=0xfffe => self.vram[(addr - 0xFF80) as usize],
            0xffff => unimplemented!("{}", addr),
            _ => panic!("Read: addr not in range: 0x{:x}", addr),
        }
    }

    fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x7fff => unimplemented!("{}", addr),
            0x8000..=0x9fff => unimplemented!("{}", addr),
            0xa000..=0xbfff => unimplemented!("{}", addr),
            0xc000..=0xcfff => self.ram[(addr - 0xC000) as usize] = val,
            0xd000..=0xdfff => self.ram[(addr - 0xC000) as usize + self.ram_offset] = val,
            0xe000..=0xfdff => self.write(addr - 0xE000 + 0xC000, val),

            0xff00 => unimplemented!("{} {}", addr, val),

            0xff01..=0xff02 => unimplemented!("Serial"),
            0xff04..=0xff07 => unimplemented!("{}", addr),

            0xff10..=0xff3f => unimplemented!("{}", addr),

            0xff0f => self.int_flags = val,

            0xff46 => {
                self.ppu_dma = val;
                unimplemented!("{}", addr)
            }

            0xfe00..=0xfeff | 0xff40..=0xff45 | 0xff47..=0xff4b | 0xff68..=0xff69 | 0xff4f => {
                unimplemented!("{}", addr)
            }

            0xff4d => {} // Speedswitch
            0xff70 => {
                self.svbk = val & 0b111;
                unimplemented!("{}", addr)
                // self.update_ram_offset()
            }

            0xff7f => {} // TETRIS writes to this address for some reason

            0xff80..=0xfffe => self.vram[(addr - 0xFF80) as usize] = val,
            0xffff => self.int_enable = val,
            _ => panic!("Write: addr not in range: 0x{:x} - val: 0x{:x}", addr, val),
        }
    }

    pub unsafe fn execute(&mut self, inst: Instruction) {
        match inst {
            // super handy - https://meganesulli.com/generate-gb-opcodes/
            Instruction::ADD(reg) => {
                let v = self.read_reg(reg);
                let (nv, did_overflow) = self.a.overflowing_add(v);

                self.flag.zero(nv == 0);
                self.flag.subtract(false);
                self.flag.carry(did_overflow);
                self.flag.half_carry((v & 0xF) + (nv & 0xF) > 0xF);

                self.a = nv;
                let _ = self.pc.wrapping_add(1);
            }
            Instruction::ADC(flag, reg) => {
                let v = self.read_reg(reg);

                let c = match flag {
                    Flag::CY => if (self.flag >> 4) & 0x01 == 1 { 1 } else { 0 },
                    _ => 0,
                };

                self.a = (self.a + v + c);

                self.set_flags(
                    self.a == 0,
                    false,
                    ((self.a & 0x0F) + (v & 0x0F) + c) > 0x0F,
                    self.a > 0x0F,
                );
                let _ = self.pc.wrapping_add(1);
            }
            Instruction::SUB(reg) => {
                let v = self.read_reg(reg);
                let r  = self.a.wrapping_sub(v);
                let c = (self.a ^ v ^ r) as u16;

                self.a = r;

                self.set_flags(
                    self.a == 0,
                    true,
                    (c & 0x0010) != 0,
                    (c & 0x0010) != 0,

                )
            },
            Instruction::SBC(reg) => {
                let regi = self.getreg(reg);
                let c = if (self.flag >> 4) & 0x01 == 1 { 1 } else { 0 };

                self.a = self.a.wrapping_sub(*regi).wrapping_sub(c);

                self.set_flags(
                    self.a == 0,
                    true,
                    self.a < 0,
                    ((self.a & 0x0F) - (*regi & 0x0F) - c) < 0,
                );
                let _ = self.pc.wrapping_add(1);
            }
            Instruction::AND(reg) => {
                self.a = self.a & self.read_reg(reg);
                self.set_flags(self.a == 0, false, true, false);
                let _ = self.pc.wrapping_add(1);
            }
            Instruction::OR(reg) => {
                self.a = self.a | self.read_reg(reg);
                self.set_flags(self.a == 0, false, false, false);
                let _ = self.pc.wrapping_add(1);
            }
            Instruction::XOR(reg) => {
                self.a = self.a ^ self.read_reg(reg);
                self.set_flags(self.a == 0, false, false, false);
                let _ = self.pc.wrapping_add(1);
            }
            Instruction::CP(_reg) => { unimplemented!() }
            Instruction::JP(_flag, _reg) => { unimplemented!() }
            Instruction::JR(_flag, _reg) => { unimplemented!() }
            Instruction::INC(_reg) if !_reg.is_virtual() => { unimplemented!() }
            Instruction::INC(_reg) => { unimplemented!() }
            Instruction::DEC(_reg) if !_reg.is_virtual() => { unimplemented!() }
            Instruction::DEC(_reg) => { unimplemented!() }
            Instruction::CCF => { unimplemented!() }
            Instruction::SCF => {
                self.flag.carry(true);
                let _ = self.pc.wrapping_add(1);
            }
            Instruction::RRA => { unimplemented!() }
            Instruction::RLA => { unimplemented!() }
            Instruction::RRCA => { unimplemented!() }
            Instruction::RRLA => { unimplemented!() }
            Instruction::CPL => { unimplemented!() }
            Instruction::BIT(bit, reg) => {
                let v = self.read_reg(reg);

                self.flag.zero(((v >> bit) & 0x01) == 0);
                self.flag.subtract(false);
                self.flag.half_carry(true);
            }
            Instruction::RES(bit, reg) => {
                if reg.is_virtual() {
                    let r = self.getreg(reg) as *mut u16;
                    *r = *r & !(0x01 << bit);
                } else {
                    let r = self.getreg(reg);
                    *r = *r & !(0x01 << bit);
                }
            }
            Instruction::SET(bit, reg) => {
                if reg.is_virtual() {
                    let r = self.getreg(reg);
                    *r = *r | !(0x01 << bit);
                } else {
                    let r = self.getreg(reg) as *mut u16;
                    *r = *r | !(0x01 << bit);
                }
            }
            Instruction::NOP => { unimplemented!() }
            Instruction::SRL => { unimplemented!() }
            Instruction::RR => { unimplemented!() }
            Instruction::RL => { unimplemented!() }
            Instruction::RRC => { unimplemented!() }
            Instruction::RLC => { unimplemented!() }
            Instruction::SRA(reg) => {
                if reg.is_virtual() {
                    *(self.getreg(reg) as *mut u16) >>= 1;
                } else {
                    *self.getreg(reg) >>= 1
                }

                let _ = self.pc.wrapping_add(1);
            }
            Instruction::SLA(reg) => {
                if reg.is_virtual() {
                    *(self.getreg(reg) as *mut u16) <<= 1;
                } else {
                    *self.getreg(reg) <<= 1
                }
                let _ = self.pc.wrapping_add(1);
            }
            Instruction::SWAP(reg) => {
                let v = if reg.is_virtual() {
                    let r = self.getreg(reg) as *mut u16;
                    *r = ((*r & 0x00FF) << 8) | ((*r & 0xFF00) >> 8);
                    *r
                } else {
                    let r = self.getreg(reg);
                    *r = ((*r & 0x0F) << 4) | ((*r & 0xF0) >> 4);
                    *r as u16
                };

                self.set_flags(v == 0, false, false, false);
                let _ = self.pc.wrapping_add(1);
            },
            Instruction::CALL(_, _) => { unimplemented!() }
            Instruction::DAA => { unimplemented!() }
            Instruction::LD(_, _, _) => { unimplemented!() }
            Instruction::LDR(_, _) => { unimplemented!() }
            Instruction::LDSP => { unimplemented!() }
            Instruction::RLCA => { unimplemented!() }
            Instruction::HCF => { unimplemented!() }
            Instruction::POP(_) => { unimplemented!() }
            Instruction::PUSH(_) => { unimplemented!() }
            Instruction::STOP => { unimplemented!() }
            Instruction::RST(_) => { unimplemented!() }
            Instruction::RET(_) => { unimplemented!() }
            Instruction::RETI => { unimplemented!() }
            Instruction::EI => { unimplemented!() }
            Instruction::DI => { unimplemented!() }
            Instruction::CB_INSTRUCTION => { unimplemented!() }
        }
    }

    /// Just a helper function to get rid of some cruft, only applicable when
    /// we have to touch every flag.
    fn set_flags(&mut self, zero: bool, subtract: bool, half_carry: bool, carry: bool) {
        self.flag.zero(zero);
        self.flag.subtract(subtract);
        self.flag.half_carry(half_carry);
        self.flag.carry(carry);
    }

    /// Simple wrapper around `read` and `getreg` for reading values off Registers
    unsafe fn read_reg(&mut self, reg: Register) -> u8 {
        if reg.is_virtual() {
            let addr = *(self.getreg(reg) as *const u16);
            self.read(addr)
        } else {
            *self.getreg(reg)
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
    /// assert_eq!(*cpu.getreg(Register::H), 0b1010_0100);
    /// assert_eq!(*cpu.getreg(Register::L), 0b0000_0001);
    /// ```
    pub(crate) fn getreg(&mut self, r: Register) -> *mut u8 {
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
            Register::D8 => &mut self.pc.to_be_bytes()[0],
            // Virtual registers
            Register::AF => &mut self.a,
            Register::BC => &mut self.b,
            Register::DE => &mut self.d,
            Register::HL => &mut self.h,
            Register::HLi => &mut self.h,
            Register::HLd => &mut self.h,
            Register::SP => &mut self.sp.to_be_bytes()[0],
            Register::D16 => &mut self.pc.to_be_bytes()[0],
        }
    }
}