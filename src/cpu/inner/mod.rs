mod flag_register;
mod memory_bus;

pub use flag_register::*;
pub use memory_bus::*;

pub enum Register {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
    AF,
    BC,
    DE,
    HL,
}

pub enum Instruction {
    ADD(Register), // ADD adds the register to A


}