use super::*;
use crate::cpu::CPU;

/// After a HALT instruction is executed, the system clock is stopped and HALT mode is entered.
/// Although the system clock is stopped in this status, the oscillator circuit and LCD controller continue to operate.
///
/// In addition, the status of the internal RAM register ports remains unchanged.
///
/// HALT mode is cancelled by an interrupt or reset signal.
///
/// The program counter is halted at the step after the HALT instruction. If both the interrupt
/// request flag and the corresponding interrupt enable flag are set, HALT mode is exited, even if
/// the interrupt master enable flag is not set.
///
/// Once HALT mode is cancelled, the program starts from the address indicated by the program counter.
///
/// If the interrupt master enable flag is set, the contents of the program counter are pushed to
/// the stack and control jumps to the starting address of the interrupt.
///
/// If the RESET terminal goes LOW in HALT moode, the mode becomes that of a normal reset.
#[inline]
pub(crate) unsafe fn halt(cpu: &mut CPU) -> Timing {
    cpu.halted = true;
    Timing::Default
}

/// In memory, push the program counter PC value corresponding to the address following the CALL
/// instruction to the 2 bytes following the byte specified by the current stack pointer SP. Then
/// load the 16-bit immediate operand a16 into PC.
///
/// The subroutine is placed after the location specified by the new PC value. When the subroutine
/// finishes, control is returned to the source program using a return instruction and by popping
/// the starting address of the next instruction (which was just pushed) and moving it to the PC.
///
/// With the push, the current value of SP is decremented by 1, and the higher-order byte of PC is
/// loaded in the memory address specified by the new SP value. The value of SP is then decremented
/// by 1 again, and the lower-order byte of PC is loaded in the memory address specified by that
/// value of SP.
///
/// The lower-order byte of a16 is placed in byte 2 of the object code, and the higher-order byte
/// in placed in byte 3.
///
/// If the Carry or Zero flag(s) is passed, the program counter PC value corresponding to the memory location of the
/// instruction following the CALL instruction is pushed to the 2 bytes following the memory byte
/// specified by the stack pointer SP. The 16-bit immediate operand a16 is then loaded into PC.
#[inline]
pub(crate) unsafe fn call(cpu: &mut CPU, f: Flag) -> Timing {
    let new_pc = D16.read(cpu);
    if cpu.status(f) {
        let ret = cpu.pc;
        push_u16(cpu, ret);
        cpu.pc = new_pc;
        Timing::Flag
    } else {
        Timing::Default
    }
}

/// The contents of the address specified by the stack pointer SP are loaded in the lower-order
/// byte of PC, and the contents of SP are incremented by 1. The contents of the address specified
/// by the new SP value are then loaded in the higher-order byte of PC, and the contents of SP are
/// incremented by 1 again. (The value of SP is 2 larger than before instruction execution.) The
/// next instruction is fetched from the address specified by the content of PC (as usual).
#[inline]
pub(crate) unsafe fn ret(cpu: &mut CPU, f: Flag) -> Timing {
    if cpu.status(f) {
        let new_pc = pop_u16(cpu);
        cpu.pc = new_pc;
        Timing::Flag
    } else {
        Timing::Default
    }
}

/// Used when an interrupt-service routine finishes. The address for the return from the interrupt
/// is loaded in the program counter PC. The master interrupt enable flag is returned to its
/// pre-interrupt status.
///
/// The contents of the address specified by the stack pointer SP are loaded in the lower-order
/// byte of PC, and the contents of SP are incremented by 1. The contents of the address specified
/// by the new SP value are then loaded in the higher-order byte of PC, and the contents of SP are
/// incremented by 1 again. (THe value of SP is 2 larger than before instruction execution.) The
/// next instruction is fetched from the address specified by the content of PC (as usual).
#[inline]
pub(crate) unsafe fn reti(cpu: &mut CPU) -> Timing {
    ei(cpu);
    ret(cpu, Flag::NF)
}

/// Push the current value of the program counter PC onto the memory stack, and load into PC the
/// Nth byte of page 0 memory addresses, 0xN0. The next instruction is fetched from the address
/// specified by the new content of PC (as usual).
///
/// With the push, the contents of the stack pointer SP are decremented by 1, and the higher-order
/// byte of PC is loaded in the memory address specified by the new SP value. The value of SP is then
/// again decremented by 1, and the lower-order byte of the PC is loaded in the memory address
/// specified by that value of SP.
///
/// The RST instruction can be used to jump to 1 of 8 addresses. Because all addresses are held in
/// page 0 memory, 0x00 is loaded in the higher-order byte of the PC, and 0x30 is loaded in the
/// lower-order byte.
#[inline]
pub(crate) unsafe fn rst(cpu: &mut CPU, n: u8) -> Timing {
    let pc = cpu.pc;
    push_u16(cpu, pc);
    cpu.pc = n as u16;
    Timing::Default
}

/// Load the contents of source register into destination register.
///
/// If the source register identifies as an address in memory, the cpu will use it's value
/// to grab a value out of memory and story that value in the destination register.
///
/// If the destination identifies as an address in memory, the cpu will use it's value to
/// insert to with the value from the source register.
#[inline]
pub(crate) unsafe fn ld<T, D: Dst<T>, S: Src<T>>(cpu: &mut CPU, dst: D, src: S) -> Timing {
    let value = src.read(cpu);
    dst.write(cpu, value);
    Timing::Default
}

/// Load the contents of source register into destination register.
///
/// Same behavior as LD, but will increment a 16-bit register
#[inline]
pub(crate) unsafe fn ldi<T, D: Dst<T>, S: Src<T>>(
    cpu: &mut CPU,
    dst: D,
    src: S,
    inc: Register,
) -> Timing {
    let t = ld(cpu, dst, src);
    inc_16(cpu, inc);
    t
}

/// Load the contents of source register into destination register.
///
/// Same behavior as LD, but will decrement a 16-bit register
#[inline]
pub(crate) unsafe fn ldd<T, D: Dst<T>, S: Src<T>>(
    cpu: &mut CPU,
    dst: D,
    src: S,
    dec: Register,
) -> Timing {
    let t = ld(cpu, dst, src);
    dec_16(cpu, dec);
    t
}

/// Load the 16-bit immediate operand a16 into the program counter PC if the Z flag is 0. If the
/// flag condition is met, then the subsequent instruction starts at address a16. If not, or no
/// flag was specified, the contents of PC are incremented, and the next instruction following the
/// current JP instruction is executed (as usual).
#[inline]
pub(crate) unsafe fn jp<S: Src<u16>>(cpu: &mut CPU, f: Flag, src: S) -> Timing {
    let new_pc = src.read(cpu);
    if cpu.status(f) {
        cpu.pc = new_pc;
        Timing::Flag
    } else {
        Timing::Default
    }
}

/// If the flagged condition is met, jump s8 steps from the current address stored in the program
/// counter (PC). If not, the instruction following the current JP instruction is executed (as usual).
#[inline]
pub(crate) unsafe fn jr<S: Src<u8>>(cpu: &mut CPU, f: Flag, src: S) -> Timing {
    let offset = (src.read(cpu) as i8) as i16;
    if cpu.status(f) {
        let pc = cpu.pc as i16;
        let new_pc = (pc + offset) as u16;
        cpu.pc = new_pc;
        Timing::Flag
    } else {
        Timing::Default
    }
}

/// Take the logical AND for each bit of the contents of the source register and the contents
/// of register A, and store the results in register A.
#[inline]
pub(crate) unsafe fn and<S: Src<u8>>(cpu: &mut CPU, src: S) -> Timing {
    let a = src.read(cpu);
    let r = a & cpu.a;
    cpu.a = r;
    cpu.set_flags(r == 0, false, true, false);
    Timing::Default
}

/// Subtract the contents of the source register and the CY flag from the contents of register A,
/// and store the results in register A.
#[inline]
pub(crate) unsafe fn sbc<D: Dst<u8> + Src<u8>, S: Src<u8>>(
    cpu: &mut CPU,
    dst: D,
    src: S,
) -> Timing {
    let a = dst.read(cpu) as i16;
    let b = src.read(cpu) as i16;
    let c = if cpu.status(Flag::CY) { 1 } else { 0 };
    let r = a.wrapping_sub(b).wrapping_sub(c);

    dst.write(cpu, r as u8);

    cpu.set_flags(
        (r as u8) == 0,
        true,
        ((a & 0x0f) - (b & 0x0f) - c) < 0,
        r < 0,
    );

    Timing::Default
}

/// Add the contents of the source register and the CY flag to the contents of register A, and
/// store the results in the 8-bit accumulator. If the source is a virtual register, it will use
/// the value at the address given by that register.
#[inline]
pub(crate) unsafe fn adc<D: Dst<u8> + Src<u8>, S: Src<u8>>(
    cpu: &mut CPU,
    dst: D,
    src: S,
) -> Timing {
    let a = dst.read(cpu) as u16;
    let b = src.read(cpu) as u16;
    let c = if cpu.status(Flag::CY) { 1 } else { 0 };
    let r = a + b + c;
    dst.write(cpu, r as u8);
    cpu.set_flags(
        (r as u8) == 0,
        false,
        ((a & 0x0f) + (b & 0x0f) + c) > 0x0f,
        r > 0x00ff,
    );
    Timing::Default
}

#[inline]
pub(crate) unsafe fn add_sp(cpu: &mut CPU) -> Timing {
    let new_sp = offset_sp(cpu);
    cpu.sp = new_sp;
    Timing::Default
}

#[inline]
pub(crate) unsafe fn ld_hl_sp(cpu: &mut CPU) -> Timing {
    let sp = offset_sp(cpu);
    cpu.h = (sp >> 8) as u8;
    cpu.l = sp as u8;
    Timing::Default
}

#[inline]
#[doc(ignore)]
pub(crate) unsafe fn offset_sp(cpu: &mut CPU) -> u16 {
    let o: u8 = D8.read(cpu); // TODO this may be bugged, needs to be i8
    let offset = o as i32;
    let sp = (cpu.sp as i16) as i32;
    let r = sp + offset;
    cpu.set_flags(
        false,
        false,
        ((sp ^ offset ^ (r & 0xffff)) & 0x10) == 0x10,
        ((sp ^ offset ^ (r & 0xffff)) & 0x100) == 0x100,
    );
    r as u16
}

#[inline]
pub(crate) unsafe fn add_8<D: Dst<u8> + Src<u8>, S: Src<u8>>(
    cpu: &mut CPU,
    dst: D,
    src: S,
) -> Timing {
    let a = dst.read(cpu) as u16;
    let b = src.read(cpu) as u16;
    let r = a + b;
    let c = a ^ b ^ r;
    dst.write(cpu, r as u8);
    cpu.set_flags((r as u8) == 0, false, (c & 0x0010) != 0, (c & 0x0100) != 0);
    Timing::Default
}

#[inline]
pub(crate) unsafe fn add_16<D: Dst<u16> + Src<u16>, S: Src<u16>>(
    cpu: &mut CPU,
    dst: D,
    src: S,
) -> Timing {
    let a = dst.read(cpu) as u32;
    let b = src.read(cpu) as u32;
    let r = a + b;
    dst.write(cpu, r as u16);
    cpu.subtract(false);
    cpu.carry((r & 0x10000) != 0);
    cpu.half_carry(((a ^ b ^ (r & 0xffff)) & 0x1000) != 0);
    Timing::Default
}

#[inline]
pub(crate) unsafe fn sub_8<D: Dst<u8> + Src<u8>, S: Src<u8>>(
    cpu: &mut CPU,
    dst: D,
    src: S,
) -> Timing {
    let a = dst.read(cpu) as u16;
    let b = src.read(cpu) as u16;
    let r = a.wrapping_sub(b);
    let c = a ^ b ^ r;
    dst.write(cpu, r as u8);
    cpu.set_flags((r as u8) == 0, true, (c & 0x0010) != 0, (c & 0x0100) != 0);
    Timing::Default
}

#[inline]
pub(crate) unsafe fn rrca(cpu: &mut CPU) -> Timing {
    rrc(cpu, A);
    cpu.zero(false);
    Timing::Default
}

#[inline]
pub(crate) unsafe fn rla(cpu: &mut CPU) -> Timing {
    rl(cpu, A);
    cpu.zero(false);
    Timing::Default
}

#[inline]
pub(crate) unsafe fn rra(cpu: &mut CPU) -> Timing {
    rr(cpu, A);
    cpu.zero(false);
    Timing::Default
}

#[inline]
pub(crate) unsafe fn rlca(cpu: &mut CPU) -> Timing {
    rlc(cpu, A);
    cpu.zero(false);
    Timing::Default
}

#[inline]
pub(crate) unsafe fn rlc<L: Dst<u8> + Src<u8>>(cpu: &mut CPU, loc: L) {
    let a = loc.read(cpu);
    let r = a.rotate_left(1);
    loc.write(cpu, r);
    cpu.set_flags(r == 0, false, false, (a & 0x80) != 0)
}

#[inline]
pub(crate) unsafe fn rl<L: Dst<u8> + Src<u8>>(cpu: &mut CPU, loc: L) {
    let a = loc.read(cpu);
    let r = a << 1;
    let r = if cpu.status(Flag::CY) { r | 0x01 } else { r };
    loc.write(cpu, r);
    cpu.set_flags(r == 0, false, false, (a & 0x80) != 0)
}

#[inline]
pub(crate) unsafe fn rr<L: Dst<u8> + Src<u8>>(cpu: &mut CPU, loc: L) {
    let a = loc.read(cpu);
    let r = a >> 1;
    let r = if cpu.status(Flag::CY) { r | 0x80 } else { r };
    loc.write(cpu, r);
    cpu.set_flags(r == 0, false, false, (a & 0x01) != 0);
}

#[inline]
pub(crate) unsafe fn rrc<L: Dst<u8> + Src<u8>>(cpu: &mut CPU, loc: L) {
    let a = loc.read(cpu);
    let r = a.rotate_right(1);
    loc.write(cpu, r);
    cpu.set_flags(r == 0, false, false, (a & 0x01) != 0)
}

#[inline]
pub(crate) unsafe fn sla<L: Dst<u8> + Src<u8>>(cpu: &mut CPU, loc: L) {
    let a = loc.read(cpu);
    let r = a << 1;
    loc.write(cpu, r);
    cpu.set_flags(r == 0, false, false, (a & 0x80) != 0)
}

#[inline]
pub(crate) unsafe fn sra<L: Dst<u8> + Src<u8>>(cpu: &mut CPU, loc: L) {
    let a = loc.read(cpu);
    let r = a >> 1;
    let r = (a & 0x80) | r;
    loc.write(cpu, r);
    cpu.set_flags(r == 0, false, false, (a & 0x01) != 0)
}

/// Adjust the accumulator to a binary-coded decimal (BCD) number after BCD addition and subtraction operations.
#[inline]
pub(crate) unsafe fn daa(cpu: &mut CPU) -> Timing {
    let mut a = cpu.a as u16;
    let n = cpu.status(S);
    let c = cpu.status(CY);
    let h = cpu.status(HC);

    if n {
        if c {
            a = a.wrapping_sub(0x60)
        }
        if h {
            a = a.wrapping_sub(0x06)
        }
    } else {
        if c || ((a & 0xff) > 0x99) {
            a = a + 0x60;
            cpu.carry(true)
        }
        if h || ((a & 0x0f) > 0x09) {
            a = a + 0x06
        }
    }
    cpu.zero((a as u8) == 0);
    cpu.half_carry(false);

    cpu.a = a as u8;

    Timing::Default
}

#[inline]
pub(crate) unsafe fn scf(cpu: &mut CPU) -> Timing {
    cpu.subtract(false);
    cpu.half_carry(false);
    cpu.carry(true);
    Timing::Default
}

#[inline]
pub(crate) unsafe fn ccf(cpu: &mut CPU) -> Timing {
    cpu.subtract(false);
    cpu.half_carry(false);
    cpu.carry(!cpu.status(Flag::CY));
    Timing::Default
}

#[inline]
pub(crate) unsafe fn bit<S: Src<u8>>(cpu: &mut CPU, bit: u8, src: S) {
    let a = src.read(cpu) >> bit;
    cpu.zero((a & 0x01) == 0);
    cpu.subtract(false);
    cpu.half_carry(true);
}

#[inline]
pub(crate) unsafe fn srl<L: Dst<u8> + Src<u8>>(cpu: &mut CPU, loc: L) {
    let a = loc.read(cpu);
    let r = a >> 1;
    loc.write(cpu, r);
    cpu.set_flags(r == 0, false, false, (a & 0x01) != 0);
}

#[inline]
pub(crate) unsafe fn res<L: Src<u8> + Dst<u8>>(cpu: &mut CPU, bit: u8, loc: L) {
    let a = loc.read(cpu);
    let r = a & !(0x01 << bit);
    loc.write(cpu, r)
}

#[inline]
pub(crate) unsafe fn set<L: Src<u8> + Dst<u8>>(cpu: &mut CPU, bit: u8, loc: L) {
    let a = loc.read(cpu);
    let r = a | (0x01 << bit);
    loc.write(cpu, r)
}

#[inline]
pub(crate) unsafe fn swap_8<L: Dst<u8> + Src<u8>>(cpu: &mut CPU, loc: L) {
    let a = loc.read(cpu);
    let r = (a << 4) | (a >> 4);
    loc.write(cpu, r);
    cpu.set_flags(r == 0, false, false, false);
}

#[inline]
pub(crate) unsafe fn xor<S: Src<u8>>(cpu: &mut CPU, src: S) -> Timing {
    let a = src.read(cpu);
    let r = cpu.a ^ a;
    cpu.a = r;
    cpu.set_flags(r == 0, false, false, false);
    Timing::Default
}

#[inline]
pub(crate) unsafe fn or<S: Src<u8>>(cpu: &mut CPU, src: S) -> Timing {
    let a = src.read(cpu);
    let r = cpu.a | a;
    cpu.a = r;
    cpu.set_flags(r == 0, false, false, false);
    Timing::Default
}

/// Take the one's complement (i.e., flip all bits) of the contents of register A.
#[inline]
pub(crate) unsafe fn cpl(cpu: &mut CPU) -> Timing {
    let a = cpu.a;
    cpu.a = !a;
    cpu.subtract(true);
    cpu.half_carry(true);
    Timing::Default
}

/// Compare the contents of source register and the contents of register A by subtracting the value
/// of the source register from the value of register A, and set the Z flag if they are equal.
///
/// The execution of this instruction does not affect the contents of register A.
#[inline]
pub(crate) unsafe fn cp<S: Src<u8>>(cpu: &mut CPU, src: S) -> Timing {
    let a = cpu.a;
    let value = src.read(cpu);
    cpu.set_flags(
        a == value,
        true,
        (a.wrapping_sub(value) & 0xf) > (a & 0xf),
        a < value,
    );
    Timing::Default
}

/// Increments the contents of `loc` by one.
#[inline]
pub(crate) unsafe fn inc_8<L: Dst<u8> + Src<u8>>(cpu: &mut CPU, loc: L) -> Timing {
    let value = loc.read(cpu);
    let result = value.wrapping_add(1);
    loc.write(cpu, result);
    cpu.zero(result == 0);
    cpu.subtract(false);
    cpu.half_carry((result & 0x0f) == 0x00);
    Timing::Default
}

/// Increments the contents of `loc` by one.
#[inline]
pub(crate) unsafe fn inc_16<L: Dst<u16> + Src<u16>>(cpu: &mut CPU, loc: L) -> Timing {
    // No condition bits are affected for 16 bit inc
    let value = loc.read(cpu);
    loc.write(cpu, value.wrapping_add(1));
    Timing::Default
}

/// Decrements the contents of `loc` by one.
#[inline]
pub(crate) unsafe fn dec_8<L: Dst<u8> + Src<u8>>(cpu: &mut CPU, loc: L) -> Timing {
    let value = loc.read(cpu);
    let result = value.wrapping_sub(1);
    loc.write(cpu, result);
    cpu.zero(result == 0);
    cpu.subtract(true);
    cpu.half_carry((result & 0x0f) == 0x0f);
    Timing::Default
}

/// Decrements the contents of `loc` by one.
#[inline]
pub(crate) unsafe fn dec_16<L: Dst<u16> + Src<u16>>(cpu: &mut CPU, loc: L) -> Timing {
    // No condition bits are affected for 16 bit dec
    let value = loc.read(cpu);
    loc.write(cpu, value.wrapping_sub(1));
    Timing::Default
}

/// Push the contents of register pair BC onto the memory stack by doing the following:
///
/// Subtract 1 from the stack pointer SP, and put the contents of the higher portion of register pair BC on the stack.
/// Subtract 2 from SP, and put the lower portion of register pair BC on the stack.
#[inline]
pub(crate) unsafe fn push<S: Src<u16>>(cpu: &mut CPU, src: S) -> Timing {
    let value = src.read(cpu);
    push_u16(cpu, value);
    Timing::Default
}

/// Pop the contents from the memory stack into register pair into register pair BC by doing the following:
///
/// Load the contents of memory specified by stack pointer SP into the lower portion of BC.
/// Add 1 to SP and load the contents from the new memory location into the upper portion of BC.
#[inline]
pub(crate) unsafe fn pop<D: Dst<u16>>(cpu: &mut CPU, dst: D) -> Timing {
    let value = pop_u16(cpu);
    dst.write(cpu, value);
    Timing::Default
}

/// Reset the interrupt master enable (IME) flag and prohibit maskable interrupts.
///
/// Even if a DI instruction is executed in an interrupt routine, the IME flag is set if a return
/// is performed with a RETI instruction.
#[inline]
pub(crate) unsafe fn di(cpu: &mut CPU) -> Timing {
    cpu.ime = false;
    Timing::Default
}

/// Set the interrupt master enable (IME) flag and enable maskable interrupts. This instruction can
/// be used in an interrupt routine to enable higher-order interrupts.
///
/// The IME flag is reset immediately after an interrupt occurs. The IME flag reset remains in
/// effect if coontrol is returned from the interrupt routine by a RET instruction. However, if an
/// EI instruction is executed in the interrupt routine, control is returned with IME = 1.
#[inline]
pub(crate) unsafe fn ei(cpu: &mut CPU) -> Timing {
    cpu.ime = true;
    Timing::Default
}

/// Push 1 byte onto the memory stack
#[inline]
pub(crate) unsafe fn push_u8(cpu: &mut CPU, value: u8) {
    let sp = cpu.sp.wrapping_sub(1);
    cpu.write_mem(sp, value);
    cpu.sp = sp
}

/// Push 2 bytes onto the memory stack
#[inline]
pub(crate) unsafe fn push_u16(cpu: &mut CPU, value: u16) {
    push_u8(cpu, (value >> 8) as u8);
    push_u8(cpu, value as u8);
}

/// Pop 1 byte from the memory stack
#[inline]
pub(crate) unsafe fn pop_u8(cpu: &mut CPU) -> u8 {
    let sp = cpu.sp;
    let value = cpu.read_mem(sp);
    cpu.sp = sp.wrapping_add(1);
    value
}

/// Pop 2 bytes from the memory stack
#[inline]
pub(crate) unsafe fn pop_u16(cpu: &mut CPU) -> u16 {
    let low = cpu.pc.d8() as u16;
    let high = cpu.pc.d8() as u16;
    (high << 8) | low
}

#[inline]
pub(crate) unsafe fn stop() -> Timing {
    // http://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html
    //
    // Instruction STOP has according to manuals opcode 10 00 and
    // thus is 2 bytes long. Anyhow it seems there is no reason for
    // it so some assemblers code it simply as one byte instruction 10
    //
    Timing::Default
}
