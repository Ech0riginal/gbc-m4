mod flag_register;
mod instructions;
mod memory_bus;

pub use flag_register::*;
pub use instructions::*;
pub use memory_bus::*;

// pub enum RegisterTarget {
//     Register(Register),
//     VirtualRegister(VirtualRegister),
// }

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

// pub enum VirtualRegister {
//     AF,
//     BC,
//     DE,
//     HL,
// }
