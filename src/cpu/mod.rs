// LR35902 ulator

#[cfg(test)]
mod tests;
mod inner;

use inner::*;
use inner::Register::{A, HL};
use atsamd_hal::hal::blocking::spi::Write;
use core::marker::PhantomData;

// CPU flag positions
const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

const BUS_SIZE: usize = 65535;
const RAM_SIZE: usize = 1024 * 32;
const VRAM_SIZE: usize = 0x7F;

// https://github.com/nekronos/gbc_rs/blob/master/src/gbc/interconnect.rs

#[repr(C)]
// The order of these fields does matter, and tbh this may not work on any other chip or compiler
pub(crate) struct CPU {
    /// CPU flag register, see inner/flag_register.rs
    flag: u8,
    /// Accumulator register
    a: u8,
    /// Registers
    c: u8,
    b: u8,
    e: u8,
    d: u8,
    l: u8,
    h: u8,
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

    fn step<T, D: Dst<T>, S: Src<T>>(&mut self) -> Instruction<T,D,S> {
        let (byte, prefixed) = {
            let tmp = self.bus.read_byte(self.pc);
            if tmp == 0xCB {
                (self.bus.read_byte(self.pc + 1), true)
            } else {
                (tmp, false)
            }
        };

        // self.pc.wrapping_add(1);

        Instruction::from_memory(prefixed, byte)
    }

    /*
        0xFF00-0xFF7F: Port/Mode registers, control register, sound register
        0xFF80-0xFFFE: Working & Stack RAM (127 bytes)
        0xFFFF: Interrupt Enable Register
     */

    fn read_mem(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7fff => unimplemented!("{}", addr),
            0x8000..=0x9fff => unimplemented!("{}", addr),
            0xa000..=0xbfff => unimplemented!("{}", addr),
            0xc000..=0xcfff => unimplemented!("{}", addr),
            0xd000..=0xdfff => unimplemented!("{}", addr),
            0xe000..=0xfdff => self.read_mem(addr - 0xE000 + 0xC000),

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

    fn write_mem(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x7fff => unimplemented!("{}", addr),
            0x8000..=0x9fff => unimplemented!("{}", addr),
            0xa000..=0xbfff => unimplemented!("{}", addr),
            0xc000..=0xcfff => self.ram[(addr - 0xC000) as usize] = val,
            0xd000..=0xdfff => self.ram[(addr - 0xC000) as usize + self.ram_offset] = val,
            0xe000..=0xfdff => self.write_mem(addr - 0xE000 + 0xC000, val),

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

    pub unsafe fn execute<T, D: Dst<T>, S: Src<T>>(&mut self, inst: Instruction<T,D,S>) {
        match inst {
            // super handy - https://meganesulli.com/generate-gb-opcodes/
            Instruction::ADD(reg) => {
                let v = reg.read(self);
                let av: u8 = A.read(self);
                let (nv, did_overflow) = av.overflowing_add(v);

                self.flag.zero(nv == 0);
                self.flag.subtract(false);
                self.flag.carry(did_overflow);
                self.flag.half_carry((v & 0xF) + (nv & 0xF) > 0xF);

                A.write(self, nv);
                let _ = self.pc.wrapping_add(1);
            }
            Instruction::ADC(reg) => {
                let v = reg.read(self);
                let c = if (self.flag >> 4) & 0x01 == 1 { 1 } else { 0 };

                A.write(self, self.a + v + c);

                self.set_flags(
                    self.a == 0,
                    false,
                    ((self.a & 0x0F) + (v & 0x0F) + c) > 0x0F,
                    self.a > 0x0F,
                );
                let _ = self.pc.wrapping_add(1);
            }
            Instruction::SUB(reg) => {
                let v = reg.read(self);
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
                let v = reg.read(self);
                let c = if (self.flag >> 4) & 0x01 == 1 { 1 } else { 0 };

                self.a = self.a.wrapping_sub(v).wrapping_sub(c);

                self.set_flags(
                    self.a == 0,
                    true,
                    self.a < 0,
                    ((self.a & 0x0F) - (v & 0x0F) - c) < 0,
                );
                let _ = self.pc.wrapping_add(1);
            }
            Instruction::AND(reg) => {
                self.a = self.a & reg.read(self);
                self.set_flags(self.a == 0, false, true, false);
                let _ = self.pc.wrapping_add(1);
            }
            Instruction::OR(reg) => {
                self.a = self.a | reg.read(self);
                self.set_flags(self.a == 0, false, false, false);
                let _ = self.pc.wrapping_add(1);
            }
            Instruction::XOR(reg) => {
                self.a = self.a ^ reg.read(self);
                self.set_flags(self.a == 0, false, false, false);
                let _ = self.pc.wrapping_add(1);
            }
            Instruction::CP(reg) => {
                let a = A.read(self);
                let v = reg.read(self);
                self.set_flags(
                    a == v,
                    true,
                    (a.wrapping_sub(v) & 0xf) > (a & 0xf),
                    a < v
                );
            }
            Instruction::BIT(bit, reg) => {
                let v = reg.read(self) >> bit;
                self.flag.zero((v & 0x01) == 0);
                self.flag.subtract(false);
                self.flag.half_carry(true);
            }
            Instruction::RES(bit, reg) => {
                if reg.is_virtual() {
                    let r = self.addr(&reg) as *mut u16;
                    *r = *r & !(0x01 << bit);
                } else {
                    let r = self.addr(&reg);
                    *r = *r & !(0x01 << bit);
                }
            }
            Instruction::SET(bit, reg) => {
                if reg.is_virtual() {
                    let r = self.addr(&reg);
                    *r = *r | !(0x01 << bit);
                } else {
                    let r = self.addr(&reg) as *mut u16;
                    *r = *r | !(0x01 << bit);
                }
            }

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
            Instruction::NOP => { unimplemented!() }
            Instruction::SRL => { unimplemented!() }
            Instruction::RR => { unimplemented!() }
            Instruction::RL => { unimplemented!() }
            Instruction::RRC => { unimplemented!() }
            Instruction::RLC(mut reg) => {
                let v = reg.read(self);
                let r = v.rotate_left(1);
                self.set_flags(r == 0, false, false, (v & 0x80) != 0);
                reg.write(self, r);
            }
            Instruction::SRA(mut reg) => {
                let v = reg.read(self) >> 1;
                reg.write(self, v);
                let _ = self.pc.wrapping_add(1);
            }
            Instruction::SLA(reg) => {
                if reg.is_virtual() {
                    *(self.vaddr(&reg)) <<= 1;
                } else {
                    *self.addr(&reg) <<= 1
                }
                let _ = self.pc.wrapping_add(1);
            }
            Instruction::SWAP(reg) => {
                let v = if reg.is_virtual() {
                    let r = self.addr(&reg) as *mut u16;
                    *r = ((*r & 0x00FF) << 8) | ((*r & 0xFF00) >> 8);
                    *r
                } else {
                    let r = self.addr(&reg);
                    *r = ((*r & 0x0F) << 4) | ((*r & 0xF0) >> 4);
                    *r as u16
                };

                self.set_flags(v == 0, false, false, false);
                let _ = self.pc.wrapping_add(1);
            },
            Instruction::CALL(_, _) => { unimplemented!() }
            Instruction::DAA => { unimplemented!() }
            Instruction::LD(dst, src) => {

            }
            Instruction::LDR(_, _) => { unimplemented!() }
            Instruction::LDSP => {


            }
            Instruction::RLCA => {
                self.execute(Instruction::RLC(A));
                self.flag.zero(false);

            }
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

    /// Simple wrapper around `read_mem` and `addr` for reading values off Registers
    unsafe fn read<R: Dst<u8> + Src<u8>>(&mut self, reg: &R) -> u8 {
        if reg.is_virtual() {
            let addr = *(self.addr(&reg) as *const u16);
            self.read_mem(addr)
        } else {
            *self.addr(&reg)
        }
    }

    /// Returns a pointer to the specified Register, mainly so that we can
    /// use our `Instruction`s to abstract away the virtual registers
    /// Remember to cast a virtual register's pointer to it's 'length', 16
    ///
    /// # Examples
    /// ```no_run
    /// use cpu::{CPU, Register8};
    ///
    /// let mut cpu = CPU::new();
    /// let mut regi = cpu.addr(Register8::H);
    ///
    /// *regi = 0b1010_0100;
    ///
    /// assert_eq!(*cpu.addr(Register8::H), 0b1010_0100);
    /// ```
    fn addr(&mut self, r: &Register) -> *mut u8 {
        match r {
            Register::D8 => &mut self.pc.to_be_bytes()[0],
            Register::A => &mut self.a,
            Register::F => &mut self.flag,
            Register::B => &mut self.b,
            Register::C => &mut self.c,
            Register::D => &mut self.d,
            Register::E => &mut self.e,
            Register::H => &mut self.h,
            Register::L => &mut self.l,
            _ => {
                panic!("Invalid call to addr");
            }
        }
    }

    fn vaddr(&mut self, r: &Register) -> *mut u16 {
        match r {
            Register::AF => (&mut self.flag as *mut u8) as *mut u16,
            Register::BC => (&mut self.c as *mut u8) as *mut u16,
            Register::DE => (&mut self.e as *mut u8) as *mut u16,
            Register::HL => (&mut self.l as *mut u8) as *mut u16,
            Register::HLi => (&mut self.l as *mut u8) as *mut u16,
            Register::HLd => (&mut self.l as *mut u8) as *mut u16,
            Register::SP => &mut self.sp,
            Register::D16 => &mut self.pc,
            _ => {
                panic!("Invalid call to vaddr");
            }
        }
    }

}