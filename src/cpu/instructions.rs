use super::*;

pub(crate) unsafe fn halt(cpu: &mut CPU) -> Timing {
    cpu.halted = true;
    Timing::Default
}

pub(crate) unsafe fn call<S: Src<u16>>(cpu: &mut CPU, flag: Flag, src: S) -> Timing {
    let new_pc = src.read(cpu);
    if flag.status(cpu) {
        let ret = cpu.reg.pc;
        cpu.push_u16(ret);
        cpu.flag.pc(new_pc);
        Timing::Flag
    } else {
        Timing::Default
    }
}

pub(crate) unsafe fn ret(cpu: &mut CPU, flag: Flag) -> Timing {
    if flag.status(cpu) {
        let new_pc = cpu.pop_u16();
        cpu.flag.pc(new_pc);
        Timing::Flag
    } else {
        Timing::Default
    }
}

pub(crate) unsafe fn reti(cpu: &mut CPU) -> Timing {
    cpu.ei();
    cpu.ret(Flag::NF)
}

pub(crate) unsafe fn rst(cpu: &mut CPU, p: u8) -> Timing {
    let pc = cpu.reg.pc;
    cpu.push_u16(pc);
    cpu.flag.pc(p as u16);
    Timing::Default
}

pub(crate) unsafe fn ld<T, D: Dst<T>, S: Src<T>>(cpu: &mut CPU, dst: D, src: S) -> Timing {
    let value = src.read(cpu);

    dst.write(cpu, value);
    Timing::Default
}

pub(crate) unsafe fn ldi<T, D: Dst<T>, S: Src<T>>(cpu: &mut CPU, dst: D, src: S, inc: Reg16) -> Timing {
    let t = cpu.ld(dst, src);
    cpu.inc_16(inc);
    t
}

pub(crate) unsafe fn ldd<T, D: Dst<T>, S: Src<T>>(cpu: &mut CPU, dst: D, src: S, dec: Reg16) -> Timing {
    let t = cpu.ld(dst, src);
    cpu.dec_16(dec);
    t
}

pub(crate) unsafe fn jp<S: Src<u16>>(cpu: &mut CPU, flag: Flag, src: S) -> Timing {
    let new_pc = src.read(cpu);
    if flag.status(cpu) {
        cpu.flag.pc(new_pc);
        Timing::Flag
    } else {
        Timing::Default
    }
}

pub(crate) unsafe fn jr<S: Src<u8>>(cpu: &mut CPU, flag: Flag, src: S) -> Timing {
    let offset = (src.read(cpu) as i8) as i16;
    if flag.status(cpu) {
        let pc = cpu.reg.pc as i16;
        let new_pc = (pc + offset) as u16;
        cpu.flag.pc(new_pc);
        Timing::Flag
    } else {
        Timing::Default
    }
}

pub(crate) unsafe fn and<S: Src<u8>>(cpu: &mut CPU, src: S) -> Timing {
    let a = src.read(cpu);
    let r = a & cpu.reg.a;
    cpu.flag.a(r);
    cpu.set_flags(r == 0, false, true, false);
    Timing::Default
}

pub(crate) unsafe fn sbc<D: Dst<u8> + Src<u8> + Copy, S: Src<u8>>(cpu: &mut CPU, dst: D, src: S) -> Timing {
    let a = dst.read(cpu) as i16;
    let b = src.read(cpu) as i16;
    let c = if cpu.reg.carry { 1 } else { 0 };
    let r = a.wrapping_sub(b).wrapping_sub(c);
    dst.write(cpu, r as u8);
    cpu.flag.zero((r as u8) == 0);
    cpu.flag.subtract(true);
    cpu.flag.carry(r < 0);
    cpu.flag.half_carry(((a & 0x0f) - (b & 0x0f) - c) < 0);
    Timing::Default
}

pub(crate) unsafe fn adc<D: Dst<u8> + Src<u8> + Copy, S: Src<u8>>(cpu: &mut CPU, dst: D, src: S) -> Timing {
    let a = dst.read(cpu) as u16;
    let b = src.read(cpu) as u16;
    let c = if cpu.reg.carry { 1 } else { 0 };
    let r = a + b + c;
    dst.write(cpu, r as u8);
    cpu.set_flags((r as u8) == 0, false, ((a & 0x0f) + (b & 0x0f) + c) > 0x0f, r > 0x00ff);
    Timing::Default
}

pub(crate) unsafe fn add_sp(cpu: &mut CPU) -> Timing {
    let new_sp = cpu.offset_sp();
    cpu.flag.sp(new_sp);
    Timing::Default
}

pub(crate) unsafe fn ld_hl_sp(cpu: &mut CPU) -> Timing {
    let sp = cpu.offset_sp();
    cpu.flag.h((sp >> 8) as u8);
    cpu.flag.l(sp as u8);
    Timing::Default
}

pub(crate) unsafe fn offset_sp(cpu: &mut CPU) -> u16 {
    let offset = (D8.read(cpu) as i8) as i32;
    let sp = (cpu.reg.sp as i16) as i32;
    let r = sp + offset;
    cpu.flag.zero(false);
    cpu.flag.subtract(false);
    cpu.flag.carry(((sp ^ offset ^ (r & 0xffff)) & 0x100) == 0x100);
    cpu.flag.half_carry(((sp ^ offset ^ (r & 0xffff)) & 0x10) == 0x10);
    r as u16
}

pub(crate) unsafe fn add_8<D: Dst<u8> + Src<u8> + Copy, S: Src<u8>>(cpu: &mut CPU, dst: D, src: S) -> Timing {
    let a = dst.read(cpu) as u16;
    let b = src.read(cpu) as u16;
    let r = a + b;
    let c = a ^ b ^ r;
    dst.write(cpu, r as u8);
    cpu.set_flags((r as u8) == 0, false, (c & 0x0010) != 0, (c & 0x0100) != 0);
    Timing::Default
}

pub(crate) unsafe fn add_16<D: Dst<u16> + Src<u16> + Copy, S: Src<u16>>(cpu: &mut CPU, dst: D, src: S) -> Timing {
    let a = dst.read(cpu) as u32;
    let b = src.read(cpu) as u32;
    let r = a + b;
    dst.write(cpu, r as u16);
    cpu.flag.subtract(false);
    cpu.flag.carry((r & 0x10000) != 0);
    cpu.flag.half_carry(((a ^ b ^ (r & 0xffff)) & 0x1000) != 0);
    Timing::Default
}

pub(crate) unsafe fn sub_8<D: Dst<u8> + Src<u8> + Copy, S: Src<u8>>(cpu: &mut CPU, dst: D, src: S) -> Timing {
    let a = dst.read(cpu) as u16;
    let b = src.read(cpu) as u16;
    let r = a.wrapping_sub(b);
    let c = a ^ b ^ r;
    dst.write(cpu, r as u8);
    cpu.set_flags((r as u8) == 0, true, (c & 0x0010) != 0, (c & 0x0100) != 0);
    Timing::Default
}

pub(crate) unsafe fn rrca(cpu: &mut CPU) -> Timing {
    cpu.rrc(Reg8::A);
    cpu.flag.zero(false);
    Timing::Default
}

pub(crate) unsafe fn rla(cpu: &mut CPU) -> Timing {
    cpu.rl(Reg8::A);
    cpu.flag.zero(false);
    Timing::Default
}

pub(crate) unsafe fn rra(cpu: &mut CPU) -> Timing {
    cpu.rr(Reg8::A);
    cpu.flag.zero(false);
    Timing::Default
}

pub(crate) unsafe fn rlca(cpu: &mut CPU) -> Timing {
    cpu.rlc(Reg8::A);
    cpu.flag.zero(false);
    Timing::Default
}

pub(crate) unsafe fn rlc<L: Dst<u8> + Src<u8> + Copy>(cpu: &mut CPU, loc: L) {
    let a = loc.read(cpu);
    let r = a.rotate_left(1);
    loc.write(cpu, r);
    cpu.flag.zero(r == 0);
    cpu.flag.subtract(false);
    cpu.flag.half_carry(false);
    cpu.reg.carry = (a & 0x80) != 0
}

pub(crate) unsafe fn rl<L: Dst<u8> + Src<u8> + Copy>(cpu: &mut CPU, loc: L) {
    let a = loc.read(cpu);
    let r = a << 1;
    let r = if cpu.reg.carry { r | 0x01 } else { r };
    loc.write(cpu, r);
    cpu.flag.zero(r == 0);
    cpu.flag.subtract(false);
    cpu.flag.half_carry(false);
    cpu.reg.carry = (a & 0x80) != 0
}

pub(crate) unsafe fn rr<L: Dst<u8> + Src<u8> + Copy>(cpu: &mut CPU, loc: L) {
    let a = loc.read(cpu);
    let r = a >> 1;
    let r = if cpu.reg.carry { r | 0x80 } else { r };
    loc.write(cpu, r);
    cpu.set_flags(r == 0, false, false, (a & 0x01) != 0);
}

pub(crate) unsafe fn rrc<L: Dst<u8> + Src<u8> + Copy>(cpu: &mut CPU, loc: L) {
    let a = loc.read(cpu);
    let r = a.rotate_right(1);
    loc.write(cpu, r);
    cpu.flag.zero(r == 0);
    cpu.flag.subtract(false);
    cpu.flag.half_carry(false);
    cpu.reg.carry = (a & 0x01) != 0
}

pub(crate) unsafe fn sla<L: Dst<u8> + Src<u8> + Copy>(cpu: &mut CPU, loc: L) {
    let a = loc.read(cpu);
    let r = a << 1;
    loc.write(cpu, r);
    cpu.flag.zero(r == 0);
    cpu.flag.subtract(false);
    cpu.flag.half_carry(false);
    cpu.reg.carry = (a & 0x80) != 0
}

pub(crate) unsafe fn sra<L: Dst<u8> + Src<u8> + Copy>(cpu: &mut CPU, loc: L) {
    let a = loc.read(cpu);
    let r = a >> 1;
    let r = (a & 0x80) | r;
    loc.write(cpu, r);
    cpu.flag.zero(r == 0);
    cpu.flag.subtract(false);
    cpu.flag.half_carry(false);
    cpu.reg.carry = (a & 0x01) != 0
}

pub(crate) unsafe fn daa(cpu: &mut CPU) -> Timing {
    let mut a = cpu.reg.a as u16;
    let n = cpu.reg.subtract;
    let c = cpu.reg.carry;
    let h = cpu.reg.half_carry;

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
            cpu.reg.carry = true
        }
        if h || ((a & 0x0f) > 0x09) {
            a = a + 0x06
        }
    }
    cpu.flag.zero((a as u8) == 0);
    cpu.flag.half_carry(false);
    cpu.flag.a(a as u8);
    Timing::Default
}

pub(crate) unsafe fn scf(cpu: &mut CPU) -> Timing {
    cpu.flag.subtract(false);
    cpu.flag.half_carry(false);
    cpu.flag.carry(true);
    Timing::Default
}

pub(crate) unsafe fn ccf(cpu: &mut CPU) -> Timing {
    cpu.flag.subtract(false);
    cpu.flag.half_carry(false);
    cpu.flag.carry(!cpu.reg.carry);
    Timing::Default
}

pub(crate) unsafe fn bit<S: Src<u8>>(cpu: &mut CPU, bit: u8, src: S) {
    let a = src.read(cpu) >> bit;
    cpu.flag.zero((a & 0x01) == 0);
    cpu.flag.subtract(false);
    cpu.flag.half_carry(true);
}

pub(crate) unsafe fn srl<L: Dst<u8> + Src<u8> + Copy>(cpu: &mut CPU, loc: L) {
    let a = loc.read(cpu);
    let r = a >> 1;
    loc.write(cpu, r);
    cpu.set_flags(r == 0, false, false, (a & 0x01) != 0);
}

pub(crate) unsafe fn res<L: Src<u8> + Dst<u8> + Copy>(cpu: &mut CPU, bit: u8, loc: L) {
    let a = loc.read(cpu);
    let r = a & !(0x01 << bit);
    loc.write(cpu, r)
}

pub(crate) unsafe fn set<L: Src<u8> + Dst<u8> + Copy>(cpu: &mut CPU, bit: u8, loc: L) {
    let a = loc.read(cpu);
    let r = a | (0x01 << bit);
    loc.write(cpu, r)
}

pub(crate) unsafe fn swap_8<L: Dst<u8> + Src<u8> + Copy>(cpu: &mut CPU, loc: L) {
    let a = loc.read(cpu);
    let r = (a << 4) | (a >> 4);
    loc.write(cpu, r);
    cpu.flag.zero(r == 0);
    cpu.flag.subtract(false);
    cpu.flag.half_carry(false);
    cpu.reg.carry = false
}

pub(crate) unsafe fn xor<S: Src<u8>>(cpu: &mut CPU, src: S) -> Timing {
    let a = src.read(cpu);
    let r = cpu.reg.a ^ a;
    cpu.set_flags(r == 0, false, false, false);
    cpu.flag.a(r);
    Timing::Default
}

pub(crate) unsafe fn or<S: Src<u8>>(cpu: &mut CPU, src: S) -> Timing {
    let a = src.read(cpu);
    let r = cpu.reg.a | a;
    cpu.set_flags(r == 0, false, false, false);
    cpu.flag.a(r);
    Timing::Default
}

pub(crate) unsafe fn cpl(cpu: &mut CPU) -> Timing {
    let a = cpu.reg.a;
    cpu.flag.a(!a);
    cpu.flag.subtract(true);
    cpu.flag.half_carry(true);
    Timing::Default
}

pub(crate) unsafe fn cp<S: Src<u8>>(cpu: &mut CPU, src: S) -> Timing {
    let a = cpu.reg.a;
    let value = src.read(cpu);
    cpu.flag.subtract(true);
    cpu.flag.carry(a < value);
    cpu.flag.zero(a == value);
    cpu.flag.half_carry((a.wrapping_sub(value) & 0xf) > (a & 0xf));
    Timing::Default
}

pub(crate) unsafe fn inc_8<L: Dst<u8> + Src<u8> + Copy>(cpu: &mut CPU, loc: L) -> Timing {
    let value = loc.read(cpu);
    let result = value.wrapping_add(1);
    loc.write(cpu, result);
    cpu.flag.zero(result == 0);
    cpu.flag.subtract(false);
    cpu.flag.half_carry((result & 0x0f) == 0x00);
    Timing::Default
}

pub(crate) unsafe fn inc_16<L: Dst<u16> + Src<u16> + Copy>(cpu: &mut CPU, loc: L) -> Timing {
    // No condition bits are affected for 16 bit inc
    let value = loc.read(cpu);
    loc.write(cpu, value.wrapping_add(1));
    Timing::Default
}

pub(crate) unsafe fn dec_8<L: Dst<u8> + Src<u8> + Copy>(cpu: &mut CPU, loc: L) -> Timing {
    let value = loc.read(cpu);
    let result = value.wrapping_sub(1);
    loc.write(cpu, result);
    cpu.flag.zero(result == 0);
    cpu.flag.subtract(true);
    cpu.flag.half_carry((result & 0x0f) == 0x0f);
    Timing::Default
}

pub(crate) unsafe fn dec_16<L: Dst<u16> + Src<u16> + Copy>(cpu: &mut CPU, loc: L) -> Timing {
    // No condition bits are affected for 16 bit dec
    let value = loc.read(cpu);
    loc.write(cpu, value.wrapping_sub(1));
    Timing::Default
}

pub(crate) unsafe fn push<S: Src<u16>>(cpu: &mut CPU, src: S) -> Timing {
    let value = src.read(cpu);
    cpu.push_u16(value);
    Timing::Default
}

pub(crate) unsafe fn pop<D: Dst<u16>>(cpu: &mut CPU, dst: D) -> Timing {
    let value = cpu.pop_u16();
    dst.write(cpu, value);
    Timing::Default
}

pub(crate) unsafe fn di(cpu: &mut CPU) -> Timing {
    cpu.ime = false;
    Timing::Default
}

pub(crate) unsafe fn ei(cpu: &mut CPU) -> Timing {
    cpu.ime = true;
    Timing::Default
}

pub(crate) unsafe fn fetch_u8(cpu: &mut CPU) -> u8 {
    let pc = cpu.reg.pc;
    let value = cpu.interconnect.read(pc);
    cpu.flag.pc(pc.wrapping_add(1));
    value
}

pub(crate) unsafe fn fetch_u16(cpu: &mut CPU) -> u16 {
    let low = cpu.pc.d8() as u16;
    let high = cpu.pc.d8() as u16;
    (high << 8) | low
}

pub(crate) unsafe fn push_u8(cpu: &mut CPU, value: u8) {
    let sp = cpu.reg.sp.wrapping_sub(1);
    cpu.interconnect.write(sp, value);
    cpu.reg.sp = sp
}

pub(crate) unsafe fn push_u16(cpu: &mut CPU, value: u16) {
    cpu.push_u8((value >> 8) as u8);
    cpu.push_u8(value as u8);
}

pub(crate) unsafe fn pop_u8(cpu: &mut CPU) -> u8 {
    let sp = cpu.reg.sp;
    let value = cpu.interconnect.read(sp);
    cpu.flag.sp(sp.wrapping_add(1));
    value
}

pub(crate) unsafe fn pop_u16(cpu: &mut CPU) -> u16 {
    let low = cpu.pop_u8() as u16;
    let high = cpu.pop_u8() as u16;
    (high << 8) | low
}

pub(crate) unsafe fn stop() -> Timing {
    // http://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html
    //
    // Instruction STOP has according to manuals opcode 10 00 and
    // thus is 2 bytes long. Anyhow it seems there is no reason for
    // it so some assemblers code it simply as one byte instruction 10
    //
    Timing::Default
}