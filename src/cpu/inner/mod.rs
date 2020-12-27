mod program_counter;
mod flag_register;
mod instructions;
mod memory_bus;

pub use program_counter::*;
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
