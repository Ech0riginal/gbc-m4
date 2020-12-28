use crate::cpu::inner::*;
use crate::cpu::inner::Flag::*;
use crate::cpu::inner::Register::*;

// Trusting https://blog.ryanlevick.com/DMG-01/public/book/cpu/register_data_instructions.html for now
// (\b[A-Z]*\b)\s\(.* in case i need this again

/// Usable instructions for us to execute on based on an opcode `from_memory`. Do not derive
/// any meaning from the order of the instructions, there is no method to the madness here.
/// This will probably change as I get into timings.
pub enum Instruction {
    /// No operation
    NOP,
    /// ADDs either the value of an 8-bit register to A, our 8-bit accumulator,
    /// or ADDs the value of a 16-bit register to HL, our 16-bit accumulator
    ADD(Register),
    // TODO ADC DOC
    ADC(Flag, Register),
    /// Subtract the contents of `Register` from the contents of register A, and store the results
    /// in register A. If the HL register is passed, SUB will read from memory at the address
    /// specified in HL and subtract that value from A, and store the result in register A.
    SUB(Register),
    // TODO SBC DOC
    SBC(Flag, Register),
    // TODO AND DOC
    AND(Register),
    // TODO OR DOC
    OR(Register),
    // TODO XOR DOC
    XOR(Register),
    // TODO CP DOC
    CP(Register),
    // TODO CALL DOC
    CALL(Flag, Register),
    // TODO JP DOC
    JP(Flag, Register),
    // TODO JR DOC
    JR(Flag, Register),
    // TODO INC DOC
    INC(Register), // TODO implement the is_virtual logic in CPU to get rid of these 8 by 16s
    // TODO DEC DOC
    DEC(Register),
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
    LD(Flag, Register, Register),
    // TODO LDR DOC
    LDR(Flag, Register),
    /// Add the 8-bit signed operand s8 to the stack pointer SP, and store the result in register pair HL.
    LDSP,
    // TODO BIT DOC
    BIT(u8, Register),
    // TODO RES DOC
    RES(u8, Register),
    // TODO SET DOC
    SET(u8, Register),
    // TODO SRL DOC
    SRL,
    // TODO RR DOC
    RR,
    // TODO RL DOC
    RL,
    // TODO RRC DOC
    RRC,
    // TODO RLC DOC
    RLC,
    /// Accumulator arithmetic right-shift, then place bit 7 into the CY flag
    RLCA,
    /// Right-shift target by 1
    SRA(Register),
    /// Left-shift target by 1
    SLA(Register),
    /// Swap nibbles (or bytes) of target
    SWAP(Register),
    /// Halt (and catch fire)
    HCF,
    // TODO POP DOC
    POP(Register),
    // TODO PUSH DOC
    PUSH(Register),
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

    // self\.(.*)\((.*)\),
    // Self::\U$1($2),
    fn from_raw_instruction(byte: u8) -> Self {
        match byte {
            0x00 => Self::NOP,
            0x01 => Self::LD(NF, BC, D16),
            0x02 => Self::LD(STR, BC, A),
            0x03 => Self::INC(BC),
            0x04 => Self::INC(B),
            0x05 => Self::DEC(B),
            0x06 => Self::LD(NF, B, D8),
            0x07 => Self::RLCA,
            0x08 => Self::LD(STR, D16, SP),
            0x09 => Self::ADD(BC),
            0x0A => Self::LD(GRB, A, BC),
            0x0B => Self::DEC(BC),
            0x0C => Self::INC(C),
            0x0D => Self::DEC(C),
            0x0E => Self::LD(NF, C, D8),
            0x0F => Self::RRCA,
            0x10 => Self::STOP,
            0x11 => Self::LD(NF, DE, D16),
            0x12 => Self::LD(STR, DE, A),
            0x13 => Self::INC(DE),
            0x14 => Self::INC(D),
            0x15 => Self::DEC(D),
            0x16 => Self::LD(NF, D, D8),
            0x17 => Self::RLA,
            0x18 => Self::JR(NF, D8),
            0x19 => Self::ADD(DE),
            0x1A => Self::LD(GRB, A, DE),
            0x1B => Self::DEC(DE),
            0x1C => Self::INC(E),
            0x1D => Self::DEC(E),
            0x1E => Self::LD(NF, E, D8),
            0x1F => Self::RRA,
            0x20 => Self::JR(Z, D8),
            0x21 => Self::LD(NF, HL, D16),
            0x22 => Self::LD(NF, HLi, A),
            0x23 => Self::INC(HL),
            0x24 => Self::INC(H),
            0x25 => Self::DEC(H),
            0x26 => Self::LD(NF, H, D8),
            0x27 => Self::DAA,
            0x28 => Self::JR(Z, D8),
            0x29 => Self::ADD(HL),
            0x2A => Self::LD(NF, A, HLi),
            0x2B => Self::DEC(HL),
            0x2C => Self::INC(L),
            0x2D => Self::DEC(L),
            0x2E => Self::LD(NF, L, D8),
            0x2F => Self::CPL,
            0x30 => Self::JR(NC, D8),
            0x31 => Self::LD(NF, SP, D16),
            0x32 => Self::LD(STR, HLd, A),
            0x33 => Self::INC(SP),
            0x34 => Self::INC(HL),
            0x35 => Self::DEC(HL),
            0x36 => Self::LD(STR, HL, D8),
            0x37 => Self::SCF,
            0x38 => Self::JR(CY, D8),
            0x39 => Self::ADD(SP),
            0x3A => Self::LD(GRB, A, HLi),
            0x3B => Self::DEC(SP),
            0x3C => Self::INC(A),
            0x3D => Self::DEC(A),
            0x3E => Self::LD(STR, A, D8),
            0x3F => Self::CCF,
            0x40 => Self::LD(NF, B, B),
            0x41 => Self::LD(NF, B, C),
            0x42 => Self::LD(NF, B, D),
            0x43 => Self::LD(NF, B, E),
            0x44 => Self::LD(NF, B, H),
            0x45 => Self::LD(NF, B, L),
            0x46 => Self::LD(STR, B, HL),
            0x47 => Self::LD(NF, B, A),
            0x48 => Self::LD(NF, C, B),
            0x49 => Self::LD(NF, C, C),
            0x4A => Self::LD(NF, C, D),
            0x4B => Self::LD(NF, C, E),
            0x4C => Self::LD(NF, C, H),
            0x4D => Self::LD(NF, C, L),
            0x4E => Self::LD(STR, C, HL),
            0x4F => Self::LD(NF, C, A),
            0x50 => Self::LD(NF, D, B),
            0x51 => Self::LD(NF, D, C),
            0x52 => Self::LD(NF, D, D),
            0x53 => Self::LD(NF, D, E),
            0x54 => Self::LD(NF, D, H),
            0x55 => Self::LD(NF, D, L),
            0x56 => Self::LD(STR, D, HL),
            0x57 => Self::LD(NF, D, A),
            0x58 => Self::LD(NF, E, B),
            0x59 => Self::LD(NF, E, C),
            0x5A => Self::LD(NF, E, D),
            0x5B => Self::LD(NF, E, E),
            0x5C => Self::LD(NF, E, H),
            0x5D => Self::LD(NF, E, L),
            0x5E => Self::LD(STR, E, HL),
            0x5F => Self::LD(NF, E, A),
            0x60 => Self::LD(NF, H, B),
            0x61 => Self::LD(NF, H, C),
            0x62 => Self::LD(NF, H, D),
            0x63 => Self::LD(NF, H, E),
            0x64 => Self::LD(NF, H, H),
            0x65 => Self::LD(NF, H, L),
            0x66 => Self::LD(STR, H, HL),
            0x67 => Self::LD(NF, H, A),
            0x68 => Self::LD(NF, L, B),
            0x69 => Self::LD(NF, L, C),
            0x6A => Self::LD(NF, L, D),
            0x6B => Self::LD(NF, L, E),
            0x6C => Self::LD(NF, L, H),
            0x6D => Self::LD(NF, L, L),
            0x6E => Self::LD(STR, L, HL),
            0x6F => Self::LD(NF, L, A),
            0x70 => Self::LD(STR, HL, B),
            0x71 => Self::LD(STR, HL, C),
            0x72 => Self::LD(STR, HL, D),
            0x73 => Self::LD(STR, HL, E),
            0x74 => Self::LD(STR, HL, H),
            0x75 => Self::LD(STR, HL, L),
            0x76 => Self::HCF,
            0x77 => Self::LD(STR, HL, A),
            0x78 => Self::LD(NF, A, B),
            0x79 => Self::LD(NF, A, C),
            0x7A => Self::LD(NF, A, D),
            0x7B => Self::LD(NF, A, E),
            0x7C => Self::LD(NF, A, H),
            0x7D => Self::LD(NF, A, L),
            0x7E => Self::LD(GRB, A, HL),
            0x7F => Self::LD(NF, A, A),
            0x80 => Self::ADD(B),
            0x81 => Self::ADD(C),
            0x82 => Self::ADD(D),
            0x83 => Self::ADD(E),
            0x84 => Self::ADD(H),
            0x85 => Self::ADD(L),
            0x86 => Self::ADD(HL),
            0x87 => Self::ADD(A),
            0x88 => Self::ADC(CY, B),
            0x89 => Self::ADC(CY, C),
            0x8A => Self::ADC(CY, D),
            0x8B => Self::ADC(CY, E),
            0x8C => Self::ADC(CY, H),
            0x8D => Self::ADC(CY, L),
            0x8E => Self::ADC(CY, HL),
            0x8F => Self::ADC(NF, A),
            0x90 => Self::SUB(B),
            0x91 => Self::SUB(C),
            0x92 => Self::SUB(D),
            0x93 => Self::SUB(E),
            0x94 => Self::SUB(H),
            0x95 => Self::SUB(L),
            0x96 => Self::SUB(HL),
            0x97 => Self::SUB(A),
            0x98 => Self::SBC(CY, B),
            0x99 => Self::SBC(CY, C),
            0x9A => Self::SBC(CY, D),
            0x9B => Self::SBC(CY, E),
            0x9C => Self::SBC(CY, H),
            0x9D => Self::SBC(CY, L),
            0x9E => Self::SBC(CY, HL),
            0x9F => Self::SBC(CY, A),
            0xA0 => Self::AND(B),
            0xA1 => Self::AND(C),
            0xA2 => Self::AND(D),
            0xA3 => Self::AND(E),
            0xA4 => Self::AND(H),
            0xA5 => Self::AND(L),
            0xA6 => Self::AND(HL),
            0xA7 => Self::AND(A),
            0xA8 => Self::XOR(B),
            0xA9 => Self::XOR(C),
            0xAA => Self::XOR(D),
            0xAB => Self::XOR(E),
            0xAC => Self::XOR(H),
            0xAD => Self::XOR(L),
            0xAE => Self::XOR(HL),
            0xAF => Self::XOR(A),
            0xB0 => Self::OR(B),
            0xB1 => Self::OR(C),
            0xB2 => Self::OR(D),
            0xB3 => Self::OR(E),
            0xB4 => Self::OR(H),
            0xB5 => Self::OR(L),
            0xB6 => Self::OR(HL),
            0xB7 => Self::OR(A),
            0xB8 => Self::CP(B),
            0xB9 => Self::CP(C),
            0xBA => Self::CP(D),
            0xBB => Self::CP(E),
            0xBC => Self::CP(H),
            0xBD => Self::CP(L),
            0xBE => Self::CP(HL),
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
            0xCE => Self::ADC(CY, D8),
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
            0xDE => Self::SBC(CY, D8),
            0xDF => Self::RST(0x18),
            0xE0 => Self::LD(STR, D8, A),
            0xE1 => Self::POP(HL),
            0xE2 => Self::LD(STR, C, A),
            0xE5 => Self::PUSH(HL),
            0xE6 => Self::AND(D8),
            0xE7 => Self::RST(0x20),
            0xE8 => Self::ADD(SP),
            0xE9 => Self::JP(NF, HL),
            0xEA => Self::LD(STR, D16, A),
            0xEE => Self::XOR(D8),
            0xEF => Self::RST(0x28),
            0xF0 => Self::LD(GRB, A, D8),
            0xF1 => Self::POP(AF),
            0xF2 => Self::LD(GRB, A, C),
            0xF3 => Self::DI,
            0xF5 => Self::PUSH(AF),
            0xF6 => Self::OR(D8),
            0xF7 => Self::RST(0x30),
            0xF8 => Self::LDSP,
            0xF9 => Self::LD(NF, SP, HL),
            0xFA => Self::LD(NF, A, D16),
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
