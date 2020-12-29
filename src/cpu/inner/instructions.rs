use crate::cpu::inner::*;
use crate::cpu::inner::Flag::*;
use crate::cpu::inner::Register::*;
use core::marker::PhantomData;

/// Usable instructions for us to execute on based on an opcode `from_memory`. Do not derive
/// any meaning from the order of the instructions, there is no method to the madness here.
/// This will probably change as I get into timings.
pub enum Instruction<T, D: Dst<T>, S: Src<T>> {
    PHANTOM(PhantomData<(D, S)>),
    /// No operation
    NOP,
    /// ADDs either the value of an 8-bit register to A, our 8-bit accumulator,
    /// or ADDs the value of a 16-bit register to HL, our 16-bit accumulator
    ADD(T),
    /// Add the contents of `Register` and the CY flag to the contents of register A, and store the
    /// results in register A. If the HL register is passed, ADC will read from memory at the address
    /// specified in HL and add that value to the value of A, and store the result in register A.
    ADC(T),
    /// Subtract the contents of `Register` from the contents of register A, and store the results
    /// in register A. If the HL register is passed, SUB will read from memory at the address
    /// specified in HL and subtract that value from A, and store the result in register A.
    SUB(T),
    // TODO SBC DOC
    SBC(T),
    // TODO AND DOC
    AND(T),
    // TODO OR DOC
    OR(T),
    // TODO XOR DOC
    XOR(T),
    // TODO CP DOC
    CP(T),
    /// If the Z flag is 1, the program counter PC value corresponding to the memory location of
    /// the instruction following the CALL instruction is pushed to the 2 bytes following the memory
    /// byte specified by the stack pointer SP. The 16-bit immediate operand is then loaded into PC.
    CALL(Flag, T),
    // TODO JP DOC
    JP(Flag, T),
    // TODO JR DOC
    JR(Flag, T),
    // TODO INC DOC
    INC(T), // TODO implement the is_virtual logic in CPU to get rid of these 8 by 16s
    // TODO DEC DOC
    DEC(T),
    // TODO DAA DOC
    DAA,
    // TODO CCF DOC
    CCF,
    // TODO SCF DOC
    SCF,
    // TODO RRA DOC
    RRA,
    // TODO RLA DOC
    RLA,
    // TODO RRCA DOC
    RRCA,
    // TODO RRLA DOC
    RRLA,
    // TODO CPL DOC
    CPL,
    // TODO LD DOC
    LD(T, T),
    // TODO LDR DOC
    LDR(Flag, T),
    /// Add the 8-bit signed operand s8 to the stack pointer SP, and store the result in register pair HL.
    LDSP,
    // TODO BIT DOC
    BIT(u8, T),
    // TODO RES DOC
    RES(u8, T),
    // TODO SET DOC
    SET(u8, T),
    // TODO SRL DOC
    SRL,
    // TODO RR DOC
    RR,
    // TODO RL DOC
    RL,
    // TODO RRC DOC
    RRC,
    // TODO RLC DOC
    RLC(T),
    /// Accumulator arithmetic right-shift, then place bit 7 into the CY flag
    RLCA,
    /// Right-shift target by 1
    SRA(T),
    /// Left-shift target by 1
    SLA(T),
    /// Swap nibbles (or bytes) of target
    SWAP(T),
    /// Halt (and catch fire)
    HCF,
    // TODO POP DOC
    POP(T),
    // TODO PUSH DOC
    PUSH(T),
    // TODO STOP DOC
    STOP,
    // TODO RST DOC
    RST(u16),
    // TODO RET DOC
    RET(Flag),
    // TODO RETI DOC
    RETI,
    /// Set the interrupt master enable (IME) flag and enable maskable interrupts. This instruction can be used in an interrupt routine to enable higher-order interrupts.
    EI,
    /// Reset the interrupt master enable (IME) flag and prohibit maskable interrupts.
    DI,
    // TODO CB DOC
    #[allow(non_camel_case_types)]
    CB_INSTRUCTION,
}

impl<T, D: Dst<T>, S: Src<T>> Instruction<T, D, S> {
    pub fn from_memory(
        prefixed: bool,
        byte: u8,
    ) -> Self {
        // The GBC's instruction set is about 50/50
        // prefixed vs non, so order shouldn't matter
        if !prefixed {
            Self::from_raw_instruction(byte)
        } else {
            Self::from_prefixed_instruction(byte)
        }
    }

    fn from_raw_instruction(byte: u8) -> Self {
        match byte {
            0x00 => Self::NOP,
            0x01 => Self::LD(BC, D16),
            0x02 => Self::LD(Mem(BC), A),
            0x03 => Self::INC(BC),
            0x04 => Self::INC(B),
            0x05 => Self::DEC(B),
            0x06 => Self::LD(B, D8),
            0x07 => Self::RLCA,
            0x08 => Self::LD(Mem(D16), SP),
            0x09 => Self::ADD(BC),
            0x0A => Self::LD(A, Mem(BC)),
            0x0B => Self::DEC(BC),
            0x0C => Self::INC(C),
            0x0D => Self::DEC(C),
            0x0E => Self::LD(C, D8),
            0x0F => Self::RRCA,
            0x10 => Self::STOP,
            0x11 => Self::LD(DE, D16),
            0x12 => Self::LD(Mem(DE), A),
            0x13 => Self::INC(DE),
            0x14 => Self::INC(D),
            0x15 => Self::DEC(D),
            0x16 => Self::LD(D, D8),
            0x17 => Self::RLA,
            0x18 => Self::JR(NF, D8),
            0x19 => Self::ADD(DE),
            0x1A => Self::LD(A, Mem(DE)),
            0x1B => Self::DEC(DE),
            0x1C => Self::INC(E),
            0x1D => Self::DEC(E),
            0x1E => Self::LD(E, D8),
            0x1F => Self::RRA,
            0x20 => Self::JR(Z, D8),
            0x21 => Self::LD(HL, D16),
            0x22 => Self::LD(HLi, A),
            0x23 => Self::INC(HL),
            0x24 => Self::INC(H),
            0x25 => Self::DEC(H),
            0x26 => Self::LD(H, D8),
            0x27 => Self::DAA,
            0x28 => Self::JR(Z, D8),
            0x29 => Self::ADD(HL),
            0x2A => Self::LD(A, HLi),
            0x2B => Self::DEC(HL),
            0x2C => Self::INC(L),
            0x2D => Self::DEC(L),
            0x2E => Self::LD(L, D8),
            0x2F => Self::CPL,
            0x30 => Self::JR(NC, D8),
            0x31 => Self::LD(SP, D16),
            0x32 => Self::LD(Mem(HLd), A),
            0x33 => Self::INC(SP),
            0x34 => Self::INC(HL),
            0x35 => Self::DEC(HL),
            0x36 => Self::LD(Mem(HL), D8),
            0x37 => Self::SCF,
            0x38 => Self::JR(CY, D8),
            0x39 => Self::ADD(SP),
            0x3A => Self::LD(A, Mem(HLi)),
            0x3B => Self::DEC(SP),
            0x3C => Self::INC(A),
            0x3D => Self::DEC(A),
            0x3E => Self::LD(A, D8),
            0x3F => Self::CCF,
            0x40 => Self::LD(B, B),
            0x41 => Self::LD(B, C),
            0x42 => Self::LD(B, D),
            0x43 => Self::LD(B, E),
            0x44 => Self::LD(B, H),
            0x45 => Self::LD(B, L),
            0x46 => Self::LD(B, Mem(HL)),
            0x47 => Self::LD(B, A),
            0x48 => Self::LD(C, B),
            0x49 => Self::LD(C, C),
            0x4A => Self::LD(C, D),
            0x4B => Self::LD(C, E),
            0x4C => Self::LD(C, H),
            0x4D => Self::LD(C, L),
            0x4E => Self::LD(C, Mem(HL)),
            0x4F => Self::LD(C, A),
            0x50 => Self::LD(D, B),
            0x51 => Self::LD(D, C),
            0x52 => Self::LD(D, D),
            0x53 => Self::LD(D, E),
            0x54 => Self::LD(D, H),
            0x55 => Self::LD(D, L),
            0x56 => Self::LD(D, Mem(HL)),
            0x57 => Self::LD(D, A),
            0x58 => Self::LD(E, B),
            0x59 => Self::LD(E, C),
            0x5A => Self::LD(E, D),
            0x5B => Self::LD(E, E),
            0x5C => Self::LD(E, H),
            0x5D => Self::LD(E, L),
            0x5E => Self::LD(E, Mem(HL)),
            0x5F => Self::LD(E, A),
            0x60 => Self::LD(H, B),
            0x61 => Self::LD(H, C),
            0x62 => Self::LD(H, D),
            0x63 => Self::LD(H, E),
            0x64 => Self::LD(H, H),
            0x65 => Self::LD(H, L),
            0x66 => Self::LD(H, Mem(HL)),
            0x67 => Self::LD(H, A),
            0x68 => Self::LD(L, B),
            0x69 => Self::LD(L, C),
            0x6A => Self::LD(L, D),
            0x6B => Self::LD(L, E),
            0x6C => Self::LD(L, H),
            0x6D => Self::LD(L, L),
            0x6E => Self::LD(L, Mem(HL)),
            0x6F => Self::LD(L, A),
            0x70 => Self::LD(Mem(HL), B),
            0x71 => Self::LD(Mem(HL), C),
            0x72 => Self::LD(Mem(HL), D),
            0x73 => Self::LD(Mem(HL), E),
            0x74 => Self::LD(Mem(HL), H),
            0x75 => Self::LD(Mem(HL), L),
            0x76 => Self::HCF,
            0x77 => Self::LD(Mem(HL), A),
            0x78 => Self::LD(A, B),
            0x79 => Self::LD(A, C),
            0x7A => Self::LD(A, D),
            0x7B => Self::LD(A, E),
            0x7C => Self::LD(A, H),
            0x7D => Self::LD(A, L),
            0x7E => Self::LD(A, Mem(HL)),
            0x7F => Self::LD(A, A),
            0x80 => Self::ADD(B),
            0x81 => Self::ADD(C),
            0x82 => Self::ADD(D),
            0x83 => Self::ADD(E),
            0x84 => Self::ADD(H),
            0x85 => Self::ADD(L),
            0x86 => Self::ADD(Mem(HL)),
            0x87 => Self::ADD(A),
            0x88 => Self::ADC(B),
            0x89 => Self::ADC(C),
            0x8A => Self::ADC(D),
            0x8B => Self::ADC(E),
            0x8C => Self::ADC(H),
            0x8D => Self::ADC(L),
            0x8E => Self::ADC(Mem(HL)),
            0x8F => Self::ADC(A),
            0x90 => Self::SUB(B),
            0x91 => Self::SUB(C),
            0x92 => Self::SUB(D),
            0x93 => Self::SUB(E),
            0x94 => Self::SUB(H),
            0x95 => Self::SUB(L),
            0x96 => Self::SUB(Mem(HL)),
            0x97 => Self::SUB(A),
            0x98 => Self::SBC(B),
            0x99 => Self::SBC(C),
            0x9A => Self::SBC(D),
            0x9B => Self::SBC(E),
            0x9C => Self::SBC(H),
            0x9D => Self::SBC(L),
            0x9E => Self::SBC(Mem(HL)),
            0x9F => Self::SBC(A),
            0xA0 => Self::AND(B),
            0xA1 => Self::AND(C),
            0xA2 => Self::AND(D),
            0xA3 => Self::AND(E),
            0xA4 => Self::AND(H),
            0xA5 => Self::AND(L),
            0xA6 => Self::AND(Mem(HL)),
            0xA7 => Self::AND(A),
            0xA8 => Self::XOR(B),
            0xA9 => Self::XOR(C),
            0xAA => Self::XOR(D),
            0xAB => Self::XOR(E),
            0xAC => Self::XOR(H),
            0xAD => Self::XOR(L),
            0xAE => Self::XOR(Mem(HL)),
            0xAF => Self::XOR(A),
            0xB0 => Self::OR(B),
            0xB1 => Self::OR(C),
            0xB2 => Self::OR(D),
            0xB3 => Self::OR(E),
            0xB4 => Self::OR(H),
            0xB5 => Self::OR(L),
            0xB6 => Self::OR(Mem(HL)),
            0xB7 => Self::OR(A),
            0xB8 => Self::CP(B),
            0xB9 => Self::CP(C),
            0xBA => Self::CP(D),
            0xBB => Self::CP(E),
            0xBC => Self::CP(H),
            0xBD => Self::CP(L),
            0xBE => Self::CP(Mem(HL)),
            0xBF => Self::CP(A),
            0xC0 => Self::RET(NZ),
            0xC1 => Self::POP(BC),
            0xC2 => Self::JP(NZ, D16),
            0xC3 => Self::JP(NF, D16),
            0xC4 => Self::CALL(NZ, D16),
            0xC5 => Self::PUSH(BC),
            0xC6 => Self::ADD(D8),
            0xC7 => Self::RST(00),
            0xC8 => Self::RET(NZ),
            0xC9 => Self::RET(NF),
            0xCA => Self::JP(NZ, D16),
            0xCB => Self::CB_INSTRUCTION, // TODO CB_INSTRUCTION handler
            0xCC => Self::CALL(NZ, D16),
            0xCD => Self::CALL(NF, D16),
            0xCE => Self::ADC(D8),
            0xCF => Self::RST(0x08),
            0xD0 => Self::RET(NC),
            0xD1 => Self::POP(DE),
            0xD2 => Self::JP(NC, D16),
            0xD4 => Self::CALL(NC, D16),
            0xD5 => Self::PUSH(DE),
            0xD6 => Self::SUB(D8),
            0xD7 => Self::RST(0x10),
            0xD8 => Self::RET(CY),
            0xD9 => Self::RETI,
            0xDA => Self::JP(CY, D16),
            0xDC => Self::CALL(CY, D16),
            0xDE => Self::SBC(D8),
            0xDF => Self::RST(0x18),
            0xE0 => Self::LD(ZMem(D8), A),
            0xE1 => Self::POP(HL),
            0xE2 => Self::LD(ZMem(C), A),
            0xE5 => Self::PUSH(HL),
            0xE6 => Self::AND(D8),
            0xE7 => Self::RST(0x20),
            0xE8 => Self::ADD(SP),
            0xE9 => Self::JP(NF, HL),
            0xEA => Self::LD(Mem(D16), A),
            0xEE => Self::XOR(D8),
            0xEF => Self::RST(0x28),
            0xF0 => Self::LD(A, ZMem(D8)),
            0xF1 => Self::POP(AF),
            0xF2 => Self::LD(A, ZMem(C)),
            0xF3 => Self::DI,
            0xF5 => Self::PUSH(AF),
            0xF6 => Self::OR(D8),
            0xF7 => Self::RST(0x30),
            0xF8 => Self::LDSP,
            0xF9 => Self::LD(SP, HL),
            0xFA => Self::LD(A, Mem(D16)),
            0xFB => Self::EI,
            0xFE => Self::CP(D8),
            0xFF => Self::RST(0x38),
            _ => {
                panic!("Could not match opcode {}", byte);
                Self::NOP
            },

        }
    }

    fn from_prefixed_instruction(byte: u8) -> Self {
        unimplemented!()
    }
}
