mod flag_register;
mod memory_bus;
mod program_counter;
mod register;

pub use flag_register::*;
pub use memory_bus::*;
pub use program_counter::*;
pub use register::*;

pub enum Timing {
    Default,
    Flag,
    Cb(u32),
}
