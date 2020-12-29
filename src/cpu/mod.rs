// LR35902 ulator

#[cfg(test)]
mod tests;
mod inner;

use inner::*;
use inner::Flag::*;
use inner::Register::*;
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
    ime: bool,
    int_enable: u8,
    int_flags: u8,
    ram_offset: usize,
    /// Memory bus, see inner/memory_bus.rs
    bus: [u8; 65535],
    ram: [u8; RAM_SIZE],
    vram: [u8; VRAM_SIZE],
}

impl CPU {
    pub unsafe fn new() -> Self {
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
            ime: false,
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
        let (prefixed, byte) = self.step();

        let timing = if prefixed {
            self.execute_cb_instruction(byte)
        } else {
            self.execute_instruction(byte)
        };

    }

    unsafe fn step(&mut self) -> (prefixed, byte) {
        let (byte, prefixed) = {
            let tmp = self.bus.read_byte(self.pc);
            if tmp == 0xCB {
                (self.bus.read_byte(self.pc + 1), true)
            } else {
                (tmp, false)
            }
        };

    }

    /*
        0xFF00-0xFF7F: Port/Mode registers, control register, sound register
        0xFF80-0xFFFE: Working & Stack RAM (127 bytes)
        0xFFFF: Interrupt Enable Register
     */

    unsafe fn read_mem(&self, addr: u16) -> u8 {
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

    unsafe fn write_mem(&mut self, addr: u16, val: u8) {
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

    #[inline]
    unsafe fn handle_interrupt(&mut self) -> u32 {
        let ints = self.interconnect.int_flags & self.interconnect.int_enable;

        if self.halted {
            self.halted = ints == 0;
        }

        if !self.ime {
            return 0;
        }

        if ints == 0 {
            return 0;
        }

        self.ime = false;

        let int = ints.trailing_zeros();
        let int_handler = {
            match int {
                0 => 0x40,// VBLANK
                1 => 0x48,// LCDC STATUS
                2 => 0x50,// TIMER OVERFLOW
                3 => 0x58,// SERIAL TRANSFER COMPLETE
                4 => 0x60,// P10-P13 INPUT SIGNAL
                _ => panic!("Invalid interrupt {:x}", int),
            }
        };

        self.interconnect.int_flags &= 0xff << (int + 1);

        let pc = self.reg.pc;
        self.push_u16(pc);
        self.flag.pc(int_handler);

        20
    }

    #[inline]
    unsafe fn execute_instruction(&mut self, opcode: u8) -> u32 {
        use super::Register::*;
        use super::Flag::*;

        let timing = {
            match opcode {
                0x00 => Timing::Default,
                0x01 => instructions::ld(self, BC, D16),
                0x02 => instructions::ld(self, Mem(BC), A),
                0x03 => instructions::inc_16(self, BC),
                0x04 => instructions::inc_8(self, B),
                0x05 => instructions::dec_8(self, B),
                0x06 => instructions::ld(self, B, D8),
                0x07 => instructions::rlca(self, ),
                0x08 => instructions::ld(self, Mem(D16), SP),
                0x09 => instructions::add_16(self, HL, BC),
                0x0a => instructions::ld(self, A, Mem(BC)),
                0x0b => instructions::dec_16(self, BC),
                0x0c => instructions::inc_8(self, C),
                0x0d => instructions::dec_8(self, C),
                0x0e => instructions::ld(self, C, D8),
                0x0f => instructions::rrca(self, ),
                0x10 => instructions::stop(),
                0x11 => instructions::ld(self, DE, D16),
                0x12 => instructions::ld(self, Mem(DE), A),
                0x13 => instructions::inc_16(self, DE),
                0x14 => instructions::inc_8(self, D),
                0x15 => instructions::dec_8(self, D),
                0x16 => instructions::ld(self, D, D8),
                0x17 => instructions::rla(self, ),
                0x18 => instructions::jr(self, NF, D8),
                0x19 => instructions::add_16(self, HL, DE),
                0x1a => instructions::ld(self, A, Mem(DE)),
                0x1b => instructions::dec_16(self, DE),
                0x1c => instructions::inc_8(self, E),
                0x1d => instructions::dec_8(self, E),
                0x1e => instructions::ld(self, E, D8),
                0x1f => instructions::rra(self, ),
                0x20 => instructions::jr(self, NZ, D8),
                0x21 => instructions::ld(self, HL, D16),
                0x22 => instructions::ldi(self, Mem(HL), A, HL),
                0x23 => instructions::inc_16(self, HL),
                0x24 => instructions::inc_8(self, H),
                0x25 => instructions::dec_8(self, H),
                0x26 => instructions::ld(self, H, D8),
                0x27 => instructions::daa(self, ),
                0x28 => instructions::jr(self, Z, D8),
                0x29 => instructions::add_16(self, HL, HL),
                0x2a => instructions::ldi(self, A, Mem(HL), HL),
                0x2b => instructions::dec_16(self, HL),
                0x2c => instructions::inc_8(self, L),
                0x2d => instructions::dec_8(self, L),
                0x2e => instructions::ld(self, L, D8),
                0x2f => instructions::cpl(self, ),
                0x30 => instructions::jr(self, NC, D8),
                0x31 => instructions::ld(self, SP, D16),
                0x32 => instructions::ldd(self, Mem(HL), A, HL),
                0x33 => instructions::inc_16(self, SP),
                0x34 => instructions::inc_8(self, Mem(HL)),
                0x35 => instructions::dec_8(self, Mem(HL)),
                0x36 => instructions::ld(self, Mem(HL), D8),
                0x37 => instructions::scf(self, ),
                0x38 => instructions::jr(self, CY, D8),
                0x39 => instructions::add_16(self, HL, SP),
                0x3a => instructions::ldd(self, A, Mem(HL), HL),
                0x3b => instructions::dec_16(self, SP),
                0x3c => instructions::inc_8(self, A),
                0x3d => instructions::dec_8(self, A),
                0x3e => instructions::ld(self, A, D8),
                0x3f => instructions::ccf(self, ),
                0x40 => instructions::ld(self, B, B),
                0x41 => instructions::ld(self, B, C),
                0x42 => instructions::ld(self, B, D),
                0x43 => instructions::ld(self, B, E),
                0x44 => instructions::ld(self, B, H),
                0x45 => instructions::ld(self, B, L),
                0x46 => instructions::ld(self, B, Mem(HL)),
                0x47 => instructions::ld(self, B, A),
                0x48 => instructions::ld(self, C, B),
                0x49 => instructions::ld(self, C, C),
                0x4a => instructions::ld(self, C, D),
                0x4b => instructions::ld(self, C, E),
                0x4c => instructions::ld(self, C, H),
                0x4d => instructions::ld(self, C, L),
                0x4e => instructions::ld(self, C, Mem(HL)),
                0x4f => instructions::ld(self, C, A),
                0x50 => instructions::ld(self, D, B),
                0x51 => instructions::ld(self, D, C),
                0x52 => instructions::ld(self, D, D),
                0x53 => instructions::ld(self, D, E),
                0x54 => instructions::ld(self, D, H),
                0x55 => instructions::ld(self, D, L),
                0x56 => instructions::ld(self, D, Mem(HL)),
                0x57 => instructions::ld(self, D, A),
                0x58 => instructions::ld(self, E, B),
                0x59 => instructions::ld(self, E, C),
                0x5a => instructions::ld(self, E, D),
                0x5b => instructions::ld(self, E, E),
                0x5c => instructions::ld(self, E, H),
                0x5d => instructions::ld(self, E, L),
                0x5e => instructions::ld(self, E, Mem(HL)),
                0x5f => instructions::ld(self, E, A),
                0x60 => instructions::ld(self, H, B),
                0x61 => instructions::ld(self, H, C),
                0x62 => instructions::ld(self, H, D),
                0x63 => instructions::ld(self, H, E),
                0x64 => instructions::ld(self, H, H),
                0x65 => instructions::ld(self, H, L),
                0x66 => instructions::ld(self, H, Mem(HL)),
                0x67 => instructions::ld(self, H, A),
                0x68 => instructions::ld(self, L, B),
                0x69 => instructions::ld(self, L, C),
                0x6a => instructions::ld(self, L, D),
                0x6b => instructions::ld(self, L, E),
                0x6c => instructions::ld(self, L, H),
                0x6d => instructions::ld(self, L, L),
                0x6e => instructions::ld(self, L, Mem(HL)),
                0x6f => instructions::ld(self, L, A),
                0x70 => instructions::ld(self, Mem(HL), B),
                0x71 => instructions::ld(self, Mem(HL), C),
                0x72 => instructions::ld(self, Mem(HL), D),
                0x73 => instructions::ld(self, Mem(HL), E),
                0x74 => instructions::ld(self, Mem(HL), H),
                0x75 => instructions::ld(self, Mem(HL), L),
                0x76 => instructions::halt(self, ),
                0x77 => instructions::ld(self, Mem(HL), A),
                0x78 => instructions::ld(self, A, B),
                0x79 => instructions::ld(self, A, C),
                0x7a => instructions::ld(self, A, D),
                0x7b => instructions::ld(self, A, E),
                0x7c => instructions::ld(self, A, H),
                0x7d => instructions::ld(self, A, L),
                0x7e => instructions::ld(self, A, Mem(HL)),
                0x7f => instructions::ld(self, A, A),
                0x80 => instructions::add_8(self, A, B),
                0x81 => instructions::add_8(self, A, C),
                0x82 => instructions::add_8(self, A, D),
                0x83 => instructions::add_8(self, A, E),
                0x84 => instructions::add_8(self, A, H),
                0x85 => instructions::add_8(self, A, L),
                0x86 => instructions::add_8(self, A, Mem(HL)),
                0x87 => instructions::add_8(self, A, A),
                0x88 => instructions::adc(self, A, B),
                0x89 => instructions::adc(self, A, C),
                0x8a => instructions::adc(self, A, D),
                0x8b => instructions::adc(self, A, E),
                0x8c => instructions::adc(self, A, H),
                0x8d => instructions::adc(self, A, L),
                0x8e => instructions::adc(self, A, Mem(HL)),
                0x8f => instructions::adc(self, A, A),
                0x90 => instructions::sub_8(self, A, B),
                0x91 => instructions::sub_8(self, A, C),
                0x92 => instructions::sub_8(self, A, D),
                0x93 => instructions::sub_8(self, A, E),
                0x94 => instructions::sub_8(self, A, H),
                0x95 => instructions::sub_8(self, A, L),
                0x96 => instructions::sub_8(self, A, Mem(HL)),
                0x97 => instructions::sub_8(self, A, A),
                0x98 => instructions::sbc(self, A, B),
                0x99 => instructions::sbc(self, A, C),
                0x9a => instructions::sbc(self, A, D),
                0x9b => instructions::sbc(self, A, E),
                0x9c => instructions::sbc(self, A, H),
                0x9d => instructions::sbc(self, A, L),
                0x9e => instructions::sbc(self, A, Mem(HL)),
                0x9f => instructions::sbc(self, A, A),
                0xa0 => instructions::and(self, B),
                0xa1 => instructions::and(self, C),
                0xa2 => instructions::and(self, D),
                0xa3 => instructions::and(self, E),
                0xa4 => instructions::and(self, H),
                0xa5 => instructions::and(self, L),
                0xa6 => instructions::and(self, Mem(HL)),
                0xa7 => instructions::and(self, A),
                0xa8 => instructions::xor(self, B),
                0xa9 => instructions::xor(self, C),
                0xaa => instructions::xor(self, D),
                0xab => instructions::xor(self, E),
                0xac => instructions::xor(self, H),
                0xad => instructions::xor(self, L),
                0xae => instructions::xor(self, Mem(HL)),
                0xaf => instructions::xor(self, A),
                0xb0 => instructions::or(self, B),
                0xb1 => instructions::or(self, C),
                0xb2 => instructions::or(self, D),
                0xb3 => instructions::or(self, E),
                0xb4 => instructions::or(self, H),
                0xb5 => instructions::or(self, L),
                0xb6 => instructions::or(self, Mem(HL)),
                0xb7 => instructions::or(self, A),
                0xb8 => instructions::cp(self, B),
                0xb9 => instructions::cp(self, C),
                0xba => instructions::cp(self, D),
                0xbb => instructions::cp(self, E),
                0xbc => instructions::cp(self, H),
                0xbd => instructions::cp(self, L),
                0xbe => instructions::cp(self, Mem(HL)),
                0xbf => instructions::cp(self, A),
                0xc0 => instructions::ret(self, NZ),
                0xc1 => instructions::pop(self, BC),
                0xc2 => instructions::jp(self, NZ, D16),
                0xc3 => instructions::jp(self, NF, D16),
                0xc4 => instructions::call(self, NZ, D16),
                0xc5 => instructions::push(self, BC),
                0xc6 => instructions::add_8(self, A, D8),
                0xc7 => instructions::rst(self, 00),
                0xc8 => instructions::ret(self, Z),
                0xc9 => instructions::ret(self, NF),
                0xca => instructions::jp(self, Z, D16),
                0xcb => instructions::execute_cb_instruction(self, ),
                0xcc => instructions::call(self, Z, D16),
                0xcd => instructions::call(self, NF, D16),
                0xce => instructions::adc(self, A, D8),
                0xcf => instructions::rst(self, 0x08),
                0xd0 => instructions::ret(self, NC),
                0xd1 => instructions::pop(self, DE),
                0xd2 => instructions::jp(self, NC, D16),
                0xd4 => instructions::call(self, NC, D16),
                0xd5 => instructions::push(self, DE),
                0xd6 => instructions::sub_8(self, A, D8),
                0xd7 => instructions::rst(self, 0x10),
                0xd8 => instructions::ret(self, CY),
                0xd9 => instructions::reti(self, ),
                0xda => instructions::jp(self, CY, D16),
                0xdc => instructions::call(self, CY, D16),
                0xde => instructions::sbc(self, A, D8),
                0xdf => instructions::rst(self, 0x18),
                0xe0 => instructions::ld(self, ZMem(D8), A),
                0xe1 => instructions::pop(self, HL),
                0xe2 => instructions::ld(self, ZMem(C), A),
                0xe5 => instructions::push(self, HL),
                0xe6 => instructions::and(self, D8),
                0xe7 => instructions::rst(self, 0x20),
                0xe8 => instructions::add_sp(self, ),
                0xe9 => instructions::jp(self, NF, HL),
                0xea => instructions::ld(self, Mem(D16), A),
                0xee => instructions::xor(self, D8),
                0xef => instructions::rst(self, 0x28),
                0xf0 => instructions::ld(self, A, ZMem(D8)),
                0xf1 => instructions::pop(self, AF),
                0xf2 => instructions::ld(self, A, ZMem(C)),
                0xf3 => instructions::di(self, ),
                0xf5 => instructions::push(self, AF),
                0xf6 => instructions::or(self, D8),
                0xf7 => instructions::rst(self, 0x30),
                0xf8 => instructions::ld_hl_sp(self, ),
                0xf9 => instructions::ld(self, SP, HL),
                0xfa => instructions::ld(self, A, Mem(D16)),
                0xfb => instructions::ei(self, ),
                0xfe => instructions::cp(self, D8),
                0xff => instructions::rst(self, 0x38),

                _ => panic!("Invalid opcode: 0x{:x}\n{:#?}", opcode, self.flag),
            }
        };

        let cycles = match timing {
            Timing::Default => OPCODE_TIMES[opcode as usize] as u32,
            Timing::Flag => OPCODE_COND_TIMES[opcode as usize] as u32,
            Timing::Cb(x) => x,
        };
        cycles * 4
    }

    #[inline]
    unsafe fn execute_cb_instruction(&mut self, opcode: u8) -> Timing {

        use Register::*;

        match opcode {

            0x00 => instructions::rlc(self, B),
            0x01 => instructions::rlc(self, C),
            0x02 => instructions::rlc(self, D),
            0x03 => instructions::rlc(self, E),
            0x04 => instructions::rlc(self, H),
            0x05 => instructions::rlc(self, L),
            0x06 => instructions::rlc(self, Mem(HL)),
            0x07 => instructions::rlc(self, A),
            0x08 => instructions::rrc(self, B),
            0x09 => instructions::rrc(self, C),
            0x0a => instructions::rrc(self, D),
            0x0b => instructions::rrc(self, E),
            0x0c => instructions::rrc(self, H),
            0x0d => instructions::rrc(self, L),
            0x0e => instructions::rrc(self, Mem(HL)),
            0x0f => instructions::rrc(self, A),
            0x10 => instructions::rl(self, B),
            0x11 => instructions::rl(self, C),
            0x12 => instructions::rl(self, D),
            0x13 => instructions::rl(self, E),
            0x14 => instructions::rl(self, H),
            0x15 => instructions::rl(self, L),
            0x16 => instructions::rl(self, Mem(HL)),
            0x17 => instructions::rl(self, A),
            0x18 => instructions::rr(self, B),
            0x19 => instructions::rr(self, C),
            0x1a => instructions::rr(self, D),
            0x1b => instructions::rr(self, E),
            0x1c => instructions::rr(self, H),
            0x1d => instructions::rr(self, L),
            0x1e => instructions::rr(self, Mem(HL)),
            0x1f => instructions::rr(self, A),
            0x20 => instructions::sla(self, B),
            0x21 => instructions::sla(self, C),
            0x22 => instructions::sla(self, D),
            0x23 => instructions::sla(self, E),
            0x24 => instructions::sla(self, H),
            0x25 => instructions::sla(self, L),
            0x26 => instructions::sla(self, Mem(HL)),
            0x27 => instructions::sla(self, A),
            0x28 => instructions::sra(self, B),
            0x29 => instructions::sra(self, C),
            0x2a => instructions::sra(self, D),
            0x2b => instructions::sra(self, E),
            0x2c => instructions::sra(self, H),
            0x2d => instructions::sra(self, L),
            0x2e => instructions::sra(self, Mem(HL)),
            0x2f => instructions::sra(self, A),
            0x30 => instructions::swap_8(self, B),
            0x31 => instructions::swap_8(self, C),
            0x32 => instructions::swap_8(self, D),
            0x33 => instructions::swap_8(self, E),
            0x34 => instructions::swap_8(self, H),
            0x35 => instructions::swap_8(self, L),
            0x36 => instructions::swap_8(self, Mem(HL)),
            0x37 => instructions::swap_8(self, A),
            0x38 => instructions::srl(self, B),
            0x39 => instructions::srl(self, C),
            0x3a => instructions::srl(self, D),
            0x3b => instructions::srl(self, E),
            0x3c => instructions::srl(self, H),
            0x3d => instructions::srl(self, L),
            0x3e => instructions::srl(self, Mem(HL)),
            0x3f => instructions::srl(self, A),
            0x40 => instructions::bit(self, 0, B),
            0x41 => instructions::bit(self, 0, C),
            0x42 => instructions::bit(self, 0, D),
            0x43 => instructions::bit(self, 0, E),
            0x44 => instructions::bit(self, 0, H),
            0x45 => instructions::bit(self, 0, L),
            0x46 => instructions::bit(self, 0, Mem(HL)),
            0x47 => instructions::bit(self, 0, A),
            0x48 => instructions::bit(self, 1, B),
            0x49 => instructions::bit(self, 1, C),
            0x4a => instructions::bit(self, 1, D),
            0x4b => instructions::bit(self, 1, E),
            0x4c => instructions::bit(self, 1, H),
            0x4d => instructions::bit(self, 1, L),
            0x4e => instructions::bit(self, 1, Mem(HL)),
            0x4f => instructions::bit(self, 1, A),
            0x50 => instructions::bit(self, 2, B),
            0x51 => instructions::bit(self, 2, C),
            0x52 => instructions::bit(self, 2, D),
            0x53 => instructions::bit(self, 2, E),
            0x54 => instructions::bit(self, 2, H),
            0x55 => instructions::bit(self, 2, L),
            0x56 => instructions::bit(self, 2, Mem(HL)),
            0x57 => instructions::bit(self, 2, A),
            0x58 => instructions::bit(self, 3, B),
            0x59 => instructions::bit(self, 3, C),
            0x5a => instructions::bit(self, 3, D),
            0x5b => instructions::bit(self, 3, E),
            0x5c => instructions::bit(self, 3, H),
            0x5d => instructions::bit(self, 3, L),
            0x5e => instructions::bit(self, 3, Mem(HL)),
            0x5f => instructions::bit(self, 3, A),
            0x60 => instructions::bit(self, 4, B),
            0x61 => instructions::bit(self, 4, C),
            0x62 => instructions::bit(self, 4, D),
            0x63 => instructions::bit(self, 4, E),
            0x64 => instructions::bit(self, 4, H),
            0x65 => instructions::bit(self, 4, L),
            0x66 => instructions::bit(self, 4, Mem(HL)),
            0x67 => instructions::bit(self, 4, A),
            0x68 => instructions::bit(self, 5, B),
            0x69 => instructions::bit(self, 5, C),
            0x6a => instructions::bit(self, 5, D),
            0x6b => instructions::bit(self, 5, E),
            0x6c => instructions::bit(self, 5, H),
            0x6d => instructions::bit(self, 5, L),
            0x6e => instructions::bit(self, 5, Mem(HL)),
            0x6f => instructions::bit(self, 5, A),
            0x70 => instructions::bit(self, 6, B),
            0x71 => instructions::bit(self, 6, C),
            0x72 => instructions::bit(self, 6, D),
            0x73 => instructions::bit(self, 6, E),
            0x74 => instructions::bit(self, 6, H),
            0x75 => instructions::bit(self, 6, L),
            0x76 => instructions::bit(self, 6, Mem(HL)),
            0x77 => instructions::bit(self, 6, A),
            0x78 => instructions::bit(self, 7, B),
            0x79 => instructions::bit(self, 7, C),
            0x7a => instructions::bit(self, 7, D),
            0x7b => instructions::bit(self, 7, E),
            0x7c => instructions::bit(self, 7, H),
            0x7d => instructions::bit(self, 7, L),
            0x7e => instructions::bit(self, 7, Mem(HL)),
            0x7f => instructions::bit(self, 7, A),
            0x80 => instructions::res(self, 0, B),
            0x81 => instructions::res(self, 0, C),
            0x82 => instructions::res(self, 0, D),
            0x83 => instructions::res(self, 0, E),
            0x84 => instructions::res(self, 0, H),
            0x85 => instructions::res(self, 0, L),
            0x86 => instructions::res(self, 0, Mem(HL)),
            0x87 => instructions::res(self, 0, A),
            0x88 => instructions::res(self, 1, B),
            0x89 => instructions::res(self, 1, C),
            0x8a => instructions::res(self, 1, D),
            0x8b => instructions::res(self, 1, E),
            0x8c => instructions::res(self, 1, H),
            0x8d => instructions::res(self, 1, L),
            0x8e => instructions::res(self, 1, Mem(HL)),
            0x8f => instructions::res(self, 1, A),
            0x90 => instructions::res(self, 2, B),
            0x91 => instructions::res(self, 2, C),
            0x92 => instructions::res(self, 2, D),
            0x93 => instructions::res(self, 2, E),
            0x94 => instructions::res(self, 2, H),
            0x95 => instructions::res(self, 2, L),
            0x96 => instructions::res(self, 2, Mem(HL)),
            0x97 => instructions::res(self, 2, A),
            0x98 => instructions::res(self, 3, B),
            0x99 => instructions::res(self, 3, C),
            0x9a => instructions::res(self, 3, D),
            0x9b => instructions::res(self, 3, E),
            0x9c => instructions::res(self, 3, H),
            0x9d => instructions::res(self, 3, L),
            0x9e => instructions::res(self, 3, Mem(HL)),
            0x9f => instructions::res(self, 3, A),
            0xa0 => instructions::res(self, 4, B),
            0xa1 => instructions::res(self, 4, C),
            0xa2 => instructions::res(self, 4, D),
            0xa3 => instructions::res(self, 4, E),
            0xa4 => instructions::res(self, 4, H),
            0xa5 => instructions::res(self, 4, L),
            0xa6 => instructions::res(self, 4, Mem(HL)),
            0xa7 => instructions::res(self, 4, A),
            0xa8 => instructions::res(self, 5, B),
            0xa9 => instructions::res(self, 5, C),
            0xaa => instructions::res(self, 5, D),
            0xab => instructions::res(self, 5, E),
            0xac => instructions::res(self, 5, H),
            0xad => instructions::res(self, 5, L),
            0xae => instructions::res(self, 5, Mem(HL)),
            0xaf => instructions::res(self, 5, A),
            0xb0 => instructions::res(self, 6, B),
            0xb1 => instructions::res(self, 6, C),
            0xb2 => instructions::res(self, 6, D),
            0xb3 => instructions::res(self, 6, E),
            0xb4 => instructions::res(self, 6, H),
            0xb5 => instructions::res(self, 6, L),
            0xb6 => instructions::res(self, 6, Mem(HL)),
            0xb7 => instructions::res(self, 6, A),
            0xb8 => instructions::res(self, 7, B),
            0xb9 => instructions::res(self, 7, C),
            0xba => instructions::res(self, 7, D),
            0xbb => instructions::res(self, 7, E),
            0xbc => instructions::res(self, 7, H),
            0xbd => instructions::res(self, 7, L),
            0xbe => instructions::res(self, 7, Mem(HL)),
            0xbf => instructions::res(self, 7, A),
            0xc0 => instructions::set(self, 0, B),
            0xc1 => instructions::set(self, 0, C),
            0xc2 => instructions::set(self, 0, D),
            0xc3 => instructions::set(self, 0, E),
            0xc4 => instructions::set(self, 0, H),
            0xc5 => instructions::set(self, 0, L),
            0xc6 => instructions::set(self, 0, Mem(HL)),
            0xc7 => instructions::set(self, 0, A),
            0xc8 => instructions::set(self, 1, B),
            0xc9 => instructions::set(self, 1, C),
            0xca => instructions::set(self, 1, D),
            0xcb => instructions::set(self, 1, E),
            0xcc => instructions::set(self, 1, H),
            0xcd => instructions::set(self, 1, L),
            0xce => instructions::set(self, 1, Mem(HL)),
            0xcf => instructions::set(self, 1, A),
            0xd0 => instructions::set(self, 2, B),
            0xd1 => instructions::set(self, 2, C),
            0xd2 => instructions::set(self, 2, D),
            0xd3 => instructions::set(self, 2, E),
            0xd4 => instructions::set(self, 2, H),
            0xd5 => instructions::set(self, 2, L),
            0xd6 => instructions::set(self, 2, Mem(HL)),
            0xd7 => instructions::set(self, 2, A),
            0xd8 => instructions::set(self, 3, B),
            0xd9 => instructions::set(self, 3, C),
            0xda => instructions::set(self, 3, D),
            0xdb => instructions::set(self, 3, E),
            0xdc => instructions::set(self, 3, H),
            0xdd => instructions::set(self, 3, L),
            0xde => instructions::set(self, 3, Mem(HL)),
            0xdf => instructions::set(self, 3, A),
            0xe0 => instructions::set(self, 4, B),
            0xe1 => instructions::set(self, 4, C),
            0xe2 => instructions::set(self, 4, D),
            0xe3 => instructions::set(self, 4, E),
            0xe4 => instructions::set(self, 4, H),
            0xe5 => instructions::set(self, 4, L),
            0xe6 => instructions::set(self, 4, Mem(HL)),
            0xe7 => instructions::set(self, 4, A),
            0xe8 => instructions::set(self, 5, B),
            0xe9 => instructions::set(self, 5, C),
            0xea => instructions::set(self, 5, D),
            0xeb => instructions::set(self, 5, E),
            0xec => instructions::set(self, 5, H),
            0xed => instructions::set(self, 5, L),
            0xee => instructions::set(self, 5, Mem(HL)),
            0xef => instructions::set(self, 5, A),
            0xf0 => instructions::set(self, 6, B),
            0xf1 => instructions::set(self, 6, C),
            0xf2 => instructions::set(self, 6, D),
            0xf3 => instructions::set(self, 6, E),
            0xf4 => instructions::set(self, 6, H),
            0xf5 => instructions::set(self, 6, L),
            0xf6 => instructions::set(self, 6, Mem(HL)),
            0xf7 => instructions::set(self, 6, A),
            0xf8 => instructions::set(self, 7, B),
            0xf9 => instructions::set(self, 7, C),
            0xfa => instructions::set(self, 7, D),
            0xfb => instructions::set(self, 7, E),
            0xfc => instructions::set(self, 7, H),
            0xfd => instructions::set(self, 7, L),
            0xfe => instructions::set(self, 7, Mem(HL)),
            0xff => instructions::set(self, 7, A),

            _ => panic!("CB opcode out of range: 0x{:x}\n{:#?}", opcode, self.reg),
        };

        Timing::Cb(CB_OPCODE_TIMES[opcode as usize] as u32)
    }
}

impl Flagd for CPU {
    fn status(&self, f: Flag) -> bool {
        match f {
            Flag::NF => true,
            Flag::Z => self.flag >> 7 == 1,
            Flag::NZ => self.flag >> 7 == 0,
            Flag::CY => self.flag >> 5 == 1,
            Flag::NC => self.flag >> 5 == 0,
            _ => panic!("Bad status call: {:016b}: {:016b}", cpu.pc, cpu.sp)
        }
    }

    fn zero(&mut self, b: bool) {
        if b {
            self.flag |= 0b1000_0000
        } else {
            self.flag &= 0b0111_1111
        }
    }

    fn subtract(&mut self, b: bool) {
        if b {
            self.flag |= 0b0100_0000
        } else {
            self.flag &= 0b1011_1111
        }
    }

    fn half_carry(&mut self, b: bool) {
        if b {
            self.flag |= 0b0010_0000
        } else {
            self.flag &= 0b1101_1111
        }
    }

    fn carry(&mut self, b: bool) {
        if b {
            self.flag |= 0b0001_0000
        } else {
            self.flag &= 0b1110_1111
        }
    }

    fn set_flags(&mut self, zero: bool, subtract: bool, half_carry: bool, carry: bool) {
        self.flag.zero(zero);
        self.flag.subtract(subtract);
        self.flag.half_carry(half_carry);
        self.flag.carry(carry);
    }
}

impl Registerd8 for CPU {
    /// Returns a pointer to the specified Register, will panic if asked for a non-8-bit register
    ///
    /// # Examples
    /// ```no_run
    /// use cpu::{CPU, Register8};
    ///
    /// let mut cpu = CPU::new();
    /// let mut regi = cpu.addr(Register::H);
    ///
    /// *regi = 0b1010_0100;
    ///
    /// assert_eq!(*cpu.addr(Register::H), 0b1010_0100);
    /// ```
    #[inline]
    unsafe fn addr(&mut self, r: &Register) -> *mut u8 {
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
}

impl Registerd16 for CPU {
    /// Returns a pointer to the specified Register, will panic if asked for a non-16-bit register
    ///
    /// # Examples
    /// ```no_run
    /// use cpu::{CPU, Register};
    ///
    /// let mut cpu = CPU::new();
    /// let mut regi = cpu.vaddr(Register::HL);
    ///
    /// *regi = 0b0000_0001_1010_0100;
    ///
    /// assert_eq!(*cpu.addr(Register::HL), 0b0000_0001_1010_0100);
    /// ```
    #[inline]
    unsafe fn vaddr(&mut self, r: &Register) -> *mut u16 {
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