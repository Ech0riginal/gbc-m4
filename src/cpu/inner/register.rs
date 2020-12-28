use super::super::CPU;

pub(crate) trait Src<T> {
    unsafe fn read(&self, cpu: &mut CPU) -> T;
}

pub(crate) trait Dst<T> {
    unsafe fn write(&self, cpu: &mut CPU, val: T);
}

pub(crate) struct ZRam<T: Src<u8>>(T);
pub(crate) struct VRam<T: Src<u16>>(T);

pub(crate) struct ZMem<T: Src<u8>>(T);
pub(crate) struct Mem<T: Src<u16>>(T);


/// Essentially defines a Register as containing a memory address instead of a value

/// Lets us construct concise `Instruction`s for our `CPU` to operate on. As you can imagine, they're
/// tightly coupled and will most likely remain that way.
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

impl Dst<u8> for ZMem<Register> {
    unsafe fn write(self, cpu: &mut Cpu, val: u8) {
        let ZMem(imm) = self;
        let offset = imm.read(cpu) as u16;
        let addr = 0xff00 + offset;
        cpu.write_mem(addr, val)
    }
}

impl Src<u8> for ZMem<Register> {
    unsafe fn read(self, cpu: &mut Cpu) -> u8 {
        let ZMem(reg) = self;
        let offset = reg.read(cpu) as u16;
        let addr = 0xff00 + offset;
        cpu.read_mem(addr)
    }
}

impl Src<u8> for Mem<Register> {
    unsafe fn read(self, cpu: &mut Cpu) -> u8 {
        let Mem(imm) = self;
        let addr = imm.read(cpu);
        cpu.read_mem(addr)
    }
}

impl Dst<u16> for Mem<Register> {
    unsafe fn write(self, cpu: &mut Cpu, val: u16) {
        let Mem(imm) = self;
        let addr = imm.read(cpu);
        let l = val as u8;
        let h = (val >> 8) as u8;
        cpu.write_mem(addr, l);
        cpu.write_mem(addr + 1, h)
    }
}

impl Dst<u8> for Register {
    unsafe fn write(&self, cpu: &mut CPU, v: u8) { *(cpu.addr(self)) = v }
}

impl Dst<u16> for Register {
    unsafe fn write(&self, cpu: &mut CPU, v: u16) { *(cpu.vaddr(self)) = v }
}

impl Src<u8> for Register {
    unsafe fn read(&self, cpu: &mut CPU) -> u8 { *(cpu.addr(self)) }
}

impl Src<u16> for Register {
    unsafe fn read(&self, cpu: &mut CPU) -> u16 { *(cpu.vaddr(self)) }
}

impl Register {
    pub fn is_virtual(&self) -> bool {
        match self {
            Self::AF | Self::BC | Self::DE | Self::HL => true,
            _ => false
        }
    }
}