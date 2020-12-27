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
    BIT,
    RES,
    SET,
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
    SWAP(Register),
}
