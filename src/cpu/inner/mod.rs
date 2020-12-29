mod program_counter;
mod flag_register;
mod memory_bus;
mod register;
pub mod instructions;

pub use program_counter::*;
pub use flag_register::*;
pub use memory_bus::*;
pub use register::*;

pub enum Timing {
    Default,
    Cond,
    Cb(u32),
}
