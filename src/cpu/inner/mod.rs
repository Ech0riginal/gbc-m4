mod program_counter;
mod flag_register;
mod instructions;
mod memory_bus;

pub use program_counter::*;
pub use flag_register::*;
pub use instructions::*;
pub use memory_bus::*;

/// Lets us construct concise `Instruction`s for our `CPU` to operate on.
pub enum Register {
    /// The accumulator register, A.
    A,
    /// Flag register, whose last nibble doesn't matter much.
    F,
    /// 8-bit general-purpose register, B.
    B,
    /// 8-bit general-purpose register, C.
    C,
    /// 8-bit general-purpose register, D.
    D,
    /// 8-bit general-purpose register, E.
    E,
    /// 8-bit general-purpose register, H.
    H,
    /// 8-bit general-purpose register, L.
    L,
    /// The 16-bit virtual accumulator register, HL.
    HL,
    /// Used to define an implicit increment to HL after using HL.
    HLi,
    /// Used to define an implicit decrement to HL after using it.
    HLd,
    /// 16-bit virtual register AF.
    AF,
    /// 16-bit virtual register BC.
    BC,
    /// 16-bit virtual register DE.
    DE,
    /// A representation of our Stack Pointer.
    SP,
    /// Pseudo-register we use to tell the cpu to consume the first byte of the Program Counter.
    D8,
    /// Pseudo-register we use to tell the cpu to consume all (two) bytes of the Program Counter.
    D16,
}

impl Register {
    pub fn is_virtual(&self) -> bool {
        match self {
            Self::AF | Self::BC | Self::DE | Self::HL => true,
            _ => false
        }
    }
}

/// The GBC's opcodes often imply behavior we define herein, pulled from @meganesu's work.
pub enum Flag {
    /// NOP, but for Flags
    NF,
    /// When we need to store a value at a memory location specified by a 16-bit register
    STR,
    /// When we need to grab the value specified by a 16-bit register into the 8-bit accumulator
    GRB,
    /// Zero flag
    Z,
    /// Not-zero flag
    NZ,
    /// Subtract flag
    S,
    /// Carry flag
    CY,
    /// Not-carry flag
    NC,
    /// Half-carry flag
    HCY,
    /// Tells the cpu that the registers given contain the value and the address to store to RAM
    RAM,
    /// Tells the cpu that the registers given contain the value and the address to store to VRAM
    VRAM,

}