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