mod program_counter;
mod flag_register;
mod instructions;
mod memory_bus;
mod register;

pub use program_counter::*;
pub use flag_register::*;
pub use instructions::*;
pub use memory_bus::*;
pub use register::*;

/*

impl Dst<u8> for ZMem<Reg8> {
    fn write(self, cpu: &mut Cpu, val: u8) {
        let ZMem(reg) = self;
        let offset = reg.read(cpu) as u16;
        let addr = 0xff00 + offset;
        cpu.interconnect.write(addr, val)
    }
}

impl Dst<u8> for ZMem<Imm8> {
    fn write(self, cpu: &mut Cpu, val: u8) {
        let ZMem(imm) = self;
        let offset = imm.read(cpu) as u16;
        let addr = 0xff00 + offset;
        cpu.interconnect.write(addr, val)
    }
}




impl Src<u16> for Reg16 {
    fn read(self, cpu: &mut Cpu) -> u16 {
        cpu.reg.read_u16(self)
    }
}

impl Src<u16> for Imm16 {
    fn read(self, cpu: &mut Cpu) -> u16 {
        cpu.fetch_u16()
    }
}

impl Dst<u16> for Reg16 {
    fn write(self, cpu: &mut Cpu, val: u16) {
        cpu.reg.write_u16(self, val)
    }
}

impl Dst<u16> for Mem<Imm16> {
    fn write(self, cpu: &mut Cpu, val: u16) {
        let Mem(imm) = self;
        let addr = imm.read(cpu);
        let l = val as u8;
        let h = (val >> 8) as u8;
        cpu.interconnect.write(addr, l);
        cpu.interconnect.write(addr + 1, h)
    }
}
*/