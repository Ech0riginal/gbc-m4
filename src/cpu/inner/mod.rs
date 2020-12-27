mod flag_register;
mod instructions;
mod memory_bus;

pub use flag_register::*;
pub use instructions::*;
pub use memory_bus::*;

pub enum Register {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
    AF,
    BC,
    DE,
    HL,
}
