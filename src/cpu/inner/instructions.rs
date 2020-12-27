use crate::cpu::inner::*;

// Trusting https://blog.ryanlevick.com/DMG-01/public/book/cpu/register_data_instructions.html for now
// (\b[A-Z]*\b)\s\(.* in case i need this again

pub enum Instruction {
    /// ADDs an 8-bit register to A, our 8-bit accumulator
    ADD8(Register),
    /// ADDs a 16-bit register to HL, our 16-bit accumulator
    ADD16(Register),
    ADC,
    SUB(Register),
    SBC,
    AND(Register),
    OR(Register),
    XOR(Register),
    CP,
    INC,
    DEC,
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
    /// Swap nibbles of target
    SWAP8(Register),
    SWAP16(Register),

}

impl Instruction {
    pub fn from_pc(byte: u8, prefixed: bool) -> Self {
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
    }

    fn from_prefixed_instruction(byte: u8) -> Self {
        unimplemented!()
    }
}