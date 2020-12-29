use super::super::CPU;
use crate::cpu::inner::Countd;

pub(crate) trait Src<T> {
    unsafe fn read(&self, cpu: &mut CPU) -> T;
}

pub(crate) trait Dst<T> {
    unsafe fn write(&self, cpu: &mut CPU, val: T);
}

/// Wraps a Register so we can recognize it as containing a vram address instead of a value
pub(crate) struct ZMem<T: Src<u8>>(pub T);
/// Wraps a Register so we can recognize it as containing a ram  address instead of a value
pub(crate) struct Mem<T: Src<u16>>(pub T);

/// Lets us construct concise `Instruction`s for our `CPU` to operate on. As you can imagine, they're
/// tightly coupled and will most likely remain that way.
pub(crate) enum Register {
    /// Pseudo-register we use to tell the cpu to consume the first byte of the Program Counter.
    D8,
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
    /// Pseudo-register we use to tell the cpu to consume all (two) bytes of the Program Counter.
    D16,
}

// TODO DOC ALL of this

impl Register {
    pub fn is_virtual(&self) -> bool {
        match self {
            Self::AF | Self::BC | Self::DE => true,
            Self::HL | Self::HLi | Self::HLd => true,
            _ => false
        }
    }
}

impl Src<u8> for Register {
    unsafe fn read(&self, cpu: &mut CPU) -> u8 {
        match self {
            Register::D8 => cpu.pc.d8(),
            _ => *(cpu.addr(self))
        }
    }
}

impl Src<u8> for Mem<Register> {
    unsafe fn read(&self, cpu: &mut CPU) -> u8 {
        let Mem(reg) = self;
        let addr = *(cpu.vaddr(reg));
        cpu.read_mem(addr)
    }
}

impl Src<u8> for ZMem<Register> {
    unsafe fn read(&self, cpu: &mut CPU) -> u8 {
        let ZMem(reg) = self;
        let addr = 0xff00 + match reg {
            Register::D8 => cpu.pc.d8(),
            _ => *(cpu.addr(reg)),
        } as u16;
        cpu.read_mem(addr)
    }
}

impl Dst<u8> for Register {
    unsafe fn write(&self, cpu: &mut CPU, val: u8) { *(cpu.addr(self)) = val; }
}

impl Dst<u8> for Mem<Register> {
    unsafe fn write(&self, cpu: &mut CPU, val: u8) {
        let Mem(reg) = self;
        let addr = match reg {
            Register::D16 => cpu.pc.d16(),
            _ => *(cpu.vaddr(reg))
        };
        cpu.write_mem(addr, val);
    }
}

impl Dst<u8> for ZMem<Register> {
    unsafe fn write(&self, cpu: &mut CPU, val: u8) {
        let ZMem(reg) = self;
        let addr = 0xff00 + match reg {
            Register::D8 => cpu.pc.d8(),
            _ => *(cpu.addr(reg))
        } as u16;
        cpu.write_mem(addr, val);
    }
}

impl Src<u16> for Register {
    unsafe fn read(&self, cpu: &mut CPU) -> u16 {
        match self {
            Register::D16 => cpu.pc.d16(),
            _ => *(cpu.vaddr(self))
        }
    }
}

impl Dst<u16> for Register {
    unsafe fn write(&self, cpu: &mut CPU, val: u16) { *(cpu.vaddr(self)) = val; }
}

impl Dst<u16> for Mem<Register> {
    unsafe fn write(&self, cpu: &mut CPU, val: u16) {
        let Mem(reg) = self;
        let addr = *(cpu.vaddr(reg));
        let l = val as u8;
        let h = (val >> 8) as u8;
        cpu.write_mem(addr, l);
        cpu.write_mem(addr + 1, h);

    }
}