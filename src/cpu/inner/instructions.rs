use crate::cpu::inner::*;

// Trusting https://blog.ryanlevick.com/DMG-01/public/book/cpu/register_data_instructions.html for now
// (\b[A-Z]*\b)\s\(.* in case i need this again

struct OpCode {
    code: u8,
    flags: u8
}


pub enum Instruction {
    /// ADDs an 8-bit register to A, our 8-bit accumulator
    ADD8(Register),
    /// ADDs a 16-bit register to HL, our 16-bit accumulator
    ADD16(Register),
    ADC(Register),
    SUB(Register),
    SBC(Register),
    AND(Register),
    OR(Register),
    XOR(Register),
    CP,
    JP,
    JR,
    INC8(Register),
    INC16(Register),
    DEC8(Register),
    DEC16(Register),
    CCF,
    SCF,
    RRA,
    RLA,
    RRCA,
    RRLA,
    CPL,
    BIT(u8, Register),
    RES(u8, Register),
    SET(u8, Register),
    SRL,
    RR,
    RL,
    RRC,
    RLC,
    /// Right-shift target by 1
    SRA(Register),
    /// Left-shift target by 1
    SLA(Register),
    /// Swap nibbles of 8-bit target
    SWAP8(Register),
    /// Swap bytes of 16-bit target
    SWAP16(Register),
}

impl Instruction {
    pub fn from_memory(prefixed: bool, byte: u8) -> Self {
        // The GBC's instruction set is about 50/50
        // prefixed vs non, so order shouldn't matter
        if !prefixed {
            Self::from_raw_instruction(byte)
        } else {
            Self::from_prefixed_instruction(byte)
        }
    }

    fn from_raw_instruction(byte: u8) -> Self {
        unimplemented!()
        /*  keep these around for reference
        match opcode {
            0x00 => Timing::Default,
            0x01 => Self::LD self.ld(BC, Imm16),
            0x02 => self.ld(Mem(BC), A),
            0x03 => self.inc_16(BC),
            0x04 => self.inc_8(B),
            0x05 => self.dec_8(B),
            0x06 => self.ld(B, Imm8),
            0x07 => self.rlca(),
            0x08 => self.ld(Mem(Imm16), SP),
            0x09 => self.add_16(HL, BC),
            0x0a => self.ld(A, Mem(BC)),
            0x0b => self.dec_16(BC),
            0x0c => self.inc_8(C),
            0x0d => self.dec_8(C),
            0x0e => self.ld(C, Imm8),
            0x0f => self.rrca(),
            0x10 => self.stop(),
            0x11 => self.ld(DE, Imm16),
            0x12 => self.ld(Mem(DE), A),
            0x13 => self.inc_16(DE),
            0x14 => self.inc_8(D),
            0x15 => self.dec_8(D),
            0x16 => self.ld(D, Imm8),
            0x17 => self.rla(),
            0x18 => self.jr(Uncond, Imm8),
            0x19 => self.add_16(HL, DE),
            0x1a => self.ld(A, Mem(DE)),
            0x1b => self.dec_16(DE),
            0x1c => self.inc_8(E),
            0x1d => self.dec_8(E),
            0x1e => self.ld(E, Imm8),
            0x1f => self.rra(),
            0x20 => self.jr(NotZero, Imm8),
            0x21 => self.ld(HL, Imm16),
            0x22 => self.ldi(Mem(HL), A, HL),
            0x23 => self.inc_16(HL),
            0x24 => self.inc_8(H),
            0x25 => self.dec_8(H),
            0x26 => self.ld(H, Imm8),
            0x27 => self.daa(),
            0x28 => self.jr(Zero, Imm8),
            0x29 => self.add_16(HL, HL),
            0x2a => self.ldi(A, Mem(HL), HL),
            0x2b => self.dec_16(HL),
            0x2c => self.inc_8(L),
            0x2d => self.dec_8(L),
            0x2e => self.ld(L, Imm8),
            0x2f => self.cpl(),
            0x30 => self.jr(NotCarry, Imm8),
            0x31 => self.ld(SP, Imm16),
            0x32 => self.ldd(Mem(HL), A, HL),
            0x33 => self.inc_16(SP),
            0x34 => self.inc_8(Mem(HL)),
            0x35 => self.dec_8(Mem(HL)),
            0x36 => self.ld(Mem(HL), Imm8),
            0x37 => self.scf(),
            0x38 => self.jr(Carry, Imm8),
            0x39 => self.add_16(HL, SP),
            0x3a => self.ldd(A, Mem(HL), HL),
            0x3b => self.dec_16(SP),
            0x3c => self.inc_8(A),
            0x3d => self.dec_8(A),
            0x3e => self.ld(A, Imm8),
            0x3f => self.ccf(),
            0x40 => self.ld(B, B),
            0x41 => self.ld(B, C),
            0x42 => self.ld(B, D),
            0x43 => self.ld(B, E),
            0x44 => self.ld(B, H),
            0x45 => self.ld(B, L),
            0x46 => self.ld(B, Mem(HL)),
            0x47 => self.ld(B, A),
            0x48 => self.ld(C, B),
            0x49 => self.ld(C, C),
            0x4a => self.ld(C, D),
            0x4b => self.ld(C, E),
            0x4c => self.ld(C, H),
            0x4d => self.ld(C, L),
            0x4e => self.ld(C, Mem(HL)),
            0x4f => self.ld(C, A),
            0x50 => self.ld(D, B),
            0x51 => self.ld(D, C),
            0x52 => self.ld(D, D),
            0x53 => self.ld(D, E),
            0x54 => self.ld(D, H),
            0x55 => self.ld(D, L),
            0x56 => self.ld(D, Mem(HL)),
            0x57 => self.ld(D, A),
            0x58 => self.ld(E, B),
            0x59 => self.ld(E, C),
            0x5a => self.ld(E, D),
            0x5b => self.ld(E, E),
            0x5c => self.ld(E, H),
            0x5d => self.ld(E, L),
            0x5e => self.ld(E, Mem(HL)),
            0x5f => self.ld(E, A),
            0x60 => self.ld(H, B),
            0x61 => self.ld(H, C),
            0x62 => self.ld(H, D),
            0x63 => self.ld(H, E),
            0x64 => self.ld(H, H),
            0x65 => self.ld(H, L),
            0x66 => self.ld(H, Mem(HL)),
            0x67 => self.ld(H, A),
            0x68 => self.ld(L, B),
            0x69 => self.ld(L, C),
            0x6a => self.ld(L, D),
            0x6b => self.ld(L, E),
            0x6c => self.ld(L, H),
            0x6d => self.ld(L, L),
            0x6e => self.ld(L, Mem(HL)),
            0x6f => self.ld(L, A),
            0x70 => self.ld(Mem(HL), B),
            0x71 => self.ld(Mem(HL), C),
            0x72 => self.ld(Mem(HL), D),
            0x73 => self.ld(Mem(HL), E),
            0x74 => self.ld(Mem(HL), H),
            0x75 => self.ld(Mem(HL), L),
            0x76 => self.halt(),
            0x77 => self.ld(Mem(HL), A),
            0x78 => self.ld(A, B),
            0x79 => self.ld(A, C),
            0x7a => self.ld(A, D),
            0x7b => self.ld(A, E),
            0x7c => self.ld(A, H),
            0x7d => self.ld(A, L),
            0x7e => self.ld(A, Mem(HL)),
            0x7f => self.ld(A, A),
            0x80 => self.add_8(A, B),
            0x81 => self.add_8(A, C),
            0x82 => self.add_8(A, D),
            0x83 => self.add_8(A, E),
            0x84 => self.add_8(A, H),
            0x85 => self.add_8(A, L),
            0x86 => self.add_8(A, Mem(HL)),
            0x87 => self.add_8(A, A),
            0x88 => self.adc(A, B),
            0x89 => self.adc(A, C),
            0x8a => self.adc(A, D),
            0x8b => self.adc(A, E),
            0x8c => self.adc(A, H),
            0x8d => self.adc(A, L),
            0x8e => self.adc(A, Mem(HL)),
            0x8f => self.adc(A, A),
            0x90 => self.sub_8(A, B),
            0x91 => self.sub_8(A, C),
            0x92 => self.sub_8(A, D),
            0x93 => self.sub_8(A, E),
            0x94 => self.sub_8(A, H),
            0x95 => self.sub_8(A, L),
            0x96 => self.sub_8(A, Mem(HL)),
            0x97 => self.sub_8(A, A),
            0x98 => self.sbc(A, B),
            0x99 => self.sbc(A, C),
            0x9a => self.sbc(A, D),
            0x9b => self.sbc(A, E),
            0x9c => self.sbc(A, H),
            0x9d => self.sbc(A, L),
            0x9e => self.sbc(A, Mem(HL)),
            0x9f => self.sbc(A, A),
            0xa0 => self.and(B),
            0xa1 => self.and(C),
            0xa2 => self.and(D),
            0xa3 => self.and(E),
            0xa4 => self.and(H),
            0xa5 => self.and(L),
            0xa6 => self.and(Mem(HL)),
            0xa7 => self.and(A),
            0xa8 => self.xor(B),
            0xa9 => self.xor(C),
            0xaa => self.xor(D),
            0xab => self.xor(E),
            0xac => self.xor(H),
            0xad => self.xor(L),
            0xae => self.xor(Mem(HL)),
            0xaf => self.xor(A),
            0xb0 => self.or(B),
            0xb1 => self.or(C),
            0xb2 => self.or(D),
            0xb3 => self.or(E),
            0xb4 => self.or(H),
            0xb5 => self.or(L),
            0xb6 => self.or(Mem(HL)),
            0xb7 => self.or(A),
            0xb8 => self.cp(B),
            0xb9 => self.cp(C),
            0xba => self.cp(D),
            0xbb => self.cp(E),
            0xbc => self.cp(H),
            0xbd => self.cp(L),
            0xbe => self.cp(Mem(HL)),
            0xbf => self.cp(A),
            0xc0 => self.ret(NotZero),
            0xc1 => self.pop(BC),
            0xc2 => self.jp(NotZero, Imm16),
            0xc3 => self.jp(Uncond, Imm16),
            0xc4 => self.call(NotZero, Imm16),
            0xc5 => self.push(BC),
            0xc6 => self.add_8(A, Imm8),
            0xc7 => self.rst(00),
            0xc8 => self.ret(Zero),
            0xc9 => self.ret(Uncond),
            0xca => self.jp(Zero, Imm16),
            0xcb => self.execute_cb_instruction(),
            0xcc => self.call(Zero, Imm16),
            0xcd => self.call(Uncond, Imm16),
            0xce => self.adc(A, Imm8),
            0xcf => self.rst(0x08),
            0xd0 => self.ret(NotCarry),
            0xd1 => self.pop(DE),
            0xd2 => self.jp(NotCarry, Imm16),
            0xd4 => self.call(NotCarry, Imm16),
            0xd5 => self.push(DE),
            0xd6 => self.sub_8(A, Imm8),
            0xd7 => self.rst(0x10),
            0xd8 => self.ret(Carry),
            0xd9 => self.reti(),
            0xda => self.jp(Carry, Imm16),
            0xdc => self.call(Carry, Imm16),
            0xde => self.sbc(A, Imm8),
            0xdf => self.rst(0x18),
            0xe0 => self.ld(ZMem(Imm8), A),
            0xe1 => self.pop(HL),
            0xe2 => self.ld(ZMem(C), A),
            0xe5 => self.push(HL),
            0xe6 => self.and(Imm8),
            0xe7 => self.rst(0x20),
            0xe8 => self.add_sp(),
            0xe9 => self.jp(Uncond, HL),
            0xea => self.ld(Mem(Imm16), A),
            0xee => self.xor(Imm8),
            0xef => self.rst(0x28),
            0xf0 => self.ld(A, ZMem(Imm8)),
            0xf1 => self.pop(AF),
            0xf2 => self.ld(A, ZMem(C)),
            0xf3 => self.di(),
            0xf5 => self.push(AF),
            0xf6 => self.or(Imm8),
            0xf7 => self.rst(0x30),
            0xf8 => self.ld_hl_sp(),
            0xf9 => self.ld(SP, HL),
            0xfa => self.ld(A, Mem(Imm16)),
            0xfb => self.ei(),
            0xfe => self.cp(Imm8),
            0xff => self.rst(0x38),
         */
    }

    fn from_prefixed_instruction(byte: u8) -> Self {
        unimplemented!()
    }
}
