use crate::cpu::inner::Register;

// Trusting https://blog.ryanlevick.com/DMG-01/public/book/cpu/register_data_instructions.html for now
// (\b[A-Z]*\b)\s\(.* in case i need this again


pub enum Instruction {
    ADD(Register), // ADDs the register to A
    ADDHL,
    ADC,
    SUB,
    SBC,
    AND,
    OR,
    XOR,
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
    RESET,
    SET,
    SRL,
    RR,
    RL,
    RRC,
    RLC,
    SRA(Register),
    SLA(Register),
    SWAP(Register),
}
