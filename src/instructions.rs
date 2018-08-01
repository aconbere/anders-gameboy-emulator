use bytes;
use mmu;

use registers::Registers8;
use registers::Registers16;
use registers::Registers;
use registers::Flag;

#[derive(Debug, Clone, Copy)]
pub enum Destination8 {
    R(Registers8),
    Mem(Registers16),
    N
}

#[derive(Debug, Clone, Copy)]
pub enum Destination16 {
    R(Registers16),
    Mem(Registers16),
    N,
}

#[derive(Debug, Clone, Copy)]
pub enum ADD16Args {
    R(Registers16)
}

#[derive(Debug, Clone, Copy)]
pub enum CheckFlag {
    Z,
    NZ,
    C,
    NC,
}

#[derive(Debug, Clone, Copy)]
pub enum JrArgs {
    CheckFlag(CheckFlag),
    N,
}

#[derive(Debug, Clone, Copy)]
pub enum JpArgs {
    CheckFlag(CheckFlag),
    N,
    HL,
}

#[derive(Debug, Clone, Copy)]
pub enum LoadFF00Targets {
    C,
    N,
    A,
}

#[derive(Debug, Clone, Copy)]
pub enum Op {
    NotImplemented,
    STOP,
    NOP,
    DI, // Disable interrupts
    EI, // Enable interrupts
    LD8(Destination8, Destination8),
    LD16(Destination16, Destination16),

    INC8(Destination8),
    Inc16(Destination16),
    DEC8(Destination8),
    DEC16(Destination16),
    OR(Destination8),
    XOR(Destination8),
    AND(Destination8),
    SUB(Destination8),
    ADD(Destination8),
    ADC(Destination8),
    ADD16(ADD16Args, ADD16Args),

    RLCA,
    RRCA,
    RRA,
    DAA,
    CPL,
    CCF,
    SCF,
    SBC(Destination8),

    LoadAndInc,
    LoadAndIncR,
    LoadAndDec,
    LoadAndDecR,
    JR(JrArgs),
    JP(JpArgs),
    LoadFF00(LoadFF00Targets, LoadFF00Targets),
    CALL(Option<CheckFlag>),
    POP(Registers16),
    PUSH(Registers16),
    RET(Option<CheckFlag>),
    RETI,
    RST(RstArgs),
    CP(Destination8),
    HALT,
    PrefixCB,

    // CB extras
    BIT(u8, Destination8),
    RES(u8, Destination8),
    SET(u8, Destination8),
    RLC(Destination8),
    RRC(Destination8),
    RL(Destination8),
    RR(Destination8),
    SRL(Destination8),
    SLA(Destination8),
    SRA(Destination8),
    SWAP(Destination8),
}

#[derive(Debug, Clone, Copy)]
pub enum RstArgs {
    H00,
    H08,
    H10,
    H18,
    H20,
    H28,
    H30,
    H38
}

fn rst_jump_location(r:&RstArgs) -> u16 {
    match r {
        RstArgs::H00 => 0x00,
        RstArgs::H08 => 0x08,
        RstArgs::H10 => 0x10,
        RstArgs::H18 => 0x18,
        RstArgs::H20 => 0x20,
        RstArgs::H28 => 0x28,
        RstArgs::H30 => 0x30,
        RstArgs::H38 => 0x38,
    }
}

fn rst(registers: &mut Registers, mmu: &mut mmu::MMU, r:&RstArgs)  {
    push(registers, mmu, &Registers16::PC);
    jump(registers, rst_jump_location(r));

}

fn cpl(registers: &mut Registers, v:u8) -> u8 {
    let out = !v;

    registers.set_flag(Flag::Z, false);
    registers.set_flag(Flag::N, true);
    registers.set_flag(Flag::H, true);
    registers.set_flag(Flag::C, false);

    out
}

fn set(v: u8, n: u8) -> u8 {
    v | (1 << n)
}

fn res(v: u8, n: u8) -> u8 {
    v & !(0x01 << n)
}

fn bit(registers:&mut Registers, v:u8, location:u8) -> bool {
    let out = bytes::check_bit(v, location);

    registers.set_flag(Flag::Z, out);
    registers.set_flag(Flag::N, false);
    registers.set_flag(Flag::H, true);
    
    out
}

fn swap(registers:&mut Registers, v: u8) -> u8 {
    let high = v << 4;
    let low = v >> 4;

    let out = high | low;

    registers.set_flag(Flag::Z, out == 0);
    registers.set_flag(Flag::N, false);
    registers.set_flag(Flag::H, false);
    registers.set_flag(Flag::C, false);

    out
}

/* shift right but retain original value
 */
fn sra(registers:&mut Registers, v: u8) -> u8 {
    let c = bytes::check_bit(v, 0);

    let mut out = v >> 1;
    
    if c {
        out = out | 0x80;
    }

    registers.set_flag(Flag::Z, out == 0);
    registers.set_flag(Flag::N, false);
    registers.set_flag(Flag::H, false);
    registers.set_flag(Flag::C, bytes::check_bit(v, 0));

    out
}

/* shift right but new value is always zeroed
 */
fn srl(registers:&mut Registers, v: u8) -> u8 {
    let out = v >> 1;

    registers.set_flag(Flag::Z, out == 0);
    registers.set_flag(Flag::N, false);
    registers.set_flag(Flag::H, false);
    registers.set_flag(Flag::C, bytes::check_bit(v, 0));

    out
}

fn rr(registers: &mut Registers, v: u8) -> u8 {
    let c = registers.get_flag(Flag::C);

    let out = if c { (v >> 1) | 0xFF } else { v >> 1 };

    registers.set_flag(Flag::Z, out == 0);
    registers.set_flag(Flag::N, false);
    registers.set_flag(Flag::H, false);
    registers.set_flag(Flag::C, bytes::check_bit(v, 0));

    out
}

/* Rotate n right. Old bit 0 to Carry flag.
 */
fn rrc(registers: &mut Registers, v: u8) -> u8 {
    let out = v >> 1;

    registers.set_flag(Flag::Z, out == 0);
    registers.set_flag(Flag::N, false);
    registers.set_flag(Flag::H, false);
    registers.set_flag(Flag::C, bytes::check_bit(v, 0));

    out
}

/* Rotate n left. Old bit 7 to Carry flag
 */
fn rlc(registers: &mut Registers, v: u8) -> u8 {
    let out = v << 1;

    registers.set_flag(Flag::Z, out == 0);
    registers.set_flag(Flag::N, false);
    registers.set_flag(Flag::H, false);
    registers.set_flag(Flag::C, bytes::check_bit(v, 7));

    out
}

fn rl(registers: &mut Registers, v: u8) -> u8 {
    let c = registers.get_flag(Flag::C);

    let out = if c { (v << 1) | 0x01 } else { v << 1 };

    registers.set_flag(Flag::Z, out == 0);
    registers.set_flag(Flag::N, false);
    registers.set_flag(Flag::H, false);
    registers.set_flag(Flag::C, bytes::check_bit(v, 7));

    out
}

/* Shift n left into Carry. LSB of n set to 0.
 */
fn sla(registers:&mut Registers, v: u8) -> u8 {
    let out = v << 1;

    registers.set_flag(Flag::Z, out == 0);
    registers.set_flag(Flag::N, false);
    registers.set_flag(Flag::H, false);
    registers.set_flag(Flag::C, bytes::check_bit(v, 7));

    out
}

fn or(registers: &mut Registers, v:u8) -> u8 {
    let a = registers.get8(&Registers8::A);
    let n = a | v;

    registers.set_flag(Flag::Z, n == 0);
    registers.set_flag(Flag::N, false);
    registers.set_flag(Flag::H, false);
    registers.set_flag(Flag::C, false);

    registers.set8(&Registers8::A, n);

    n
}

fn xor(registers: &mut Registers, v:u8) {
    let a = registers.get8(&Registers8::A);
    let n = a ^ v;

    registers.set_flag(Flag::Z, n == 0);
    registers.set_flag(Flag::N, false);
    registers.set_flag(Flag::H, false);
    registers.set_flag(Flag::C, false);

    registers.set8(&Registers8::A, n);
}

fn and(registers: &mut Registers, v:u8) {
    let a = registers.get8(&Registers8::A);
    let n = a & v;

    registers.set_flag(Flag::Z, n == 0);
    registers.set_flag(Flag::N, false);
    registers.set_flag(Flag::H, true);
    registers.set_flag(Flag::C, false);

    registers.set8(&Registers8::A, n);
}

fn call(registers: &mut Registers, mmu: &mut mmu::MMU, v: u16) {
    push(registers, mmu, &Registers16::PC);
    registers.set16( &Registers16::PC, v);
}

fn ret(registers: &mut Registers, mmu: &mmu::MMU) {
    pop(registers, mmu, &Registers16::PC);
}

fn dec8(registers: &mut Registers, a: u8) -> u8 {
    let n = a.wrapping_sub(1);

    registers.set_flag(Flag::Z, n == 0);
    registers.set_flag(Flag::N, true);
    registers.set_flag(Flag::H, (n & 0xF) == 0xF);

    n
}

fn dec16(_registers: &mut Registers, v: u16) -> u16 {
    let n = v.wrapping_sub(1);

    // docs say no flag adjusted
    // registers.set_flag(Flag::N, true);
    // registers.set_flag(Flag::Z, n == 0);
    // registers.set_flag(Flag::H, v & 0xFF == 0xFF);

    n
}

pub fn jump(registers: &mut Registers, v: u16) {
    registers.set16(&Registers16::PC, v);
}

fn jump_relative(registers: &mut Registers, v: i8) {
    let pc = registers.get16(&Registers16::PC);
    let out = bytes::add_unsigned_signed(pc, v);
    registers.set16(&Registers16::PC, out);
}

fn compare(registers: &mut Registers, v: u8) {
    let a = registers.get8(&Registers8::A);

    sub(registers, a, v);
}

fn check_flags(registers: &Registers, f: &CheckFlag) -> bool {
    match f {
        CheckFlag::Z => registers.get_flag(Flag::Z),
        CheckFlag::NZ => !registers.get_flag(Flag::Z),
        CheckFlag::C => registers.get_flag(Flag::C),
        CheckFlag::NC => !registers.get_flag(Flag::C),
    }
}

fn sub(registers: &mut Registers, a: u8, v: u8) -> u8 {
    let n = a.wrapping_sub(v);

    registers.set_flag(Flag::Z, n == 0);
    registers.set_flag(Flag::N, true);
    registers.set_flag(Flag::H, check_half_carry_8_sub(a, v));
    registers.set_flag(Flag::C, n > a);

    n
}

fn sbc(registers: &mut Registers, a: u8, v: u8) -> u8 {
    let c = registers.get_flag(Flag::C);

    let n = if c {
        a.wrapping_sub(v).wrapping_sub(1)
    } else {
        a.wrapping_sub(v)
    };

    registers.set_flag(Flag::Z, n == 0);
    registers.set_flag(Flag::N, true);
    registers.set_flag(Flag::H, check_half_carry_8_sub(a, v));
    registers.set_flag(Flag::C, n > a);

    n
}

fn check_half_carry_8_add(a: u8, b: u8) -> bool {
    (((a & 0xF) + (b & 0xF)) & 0x10) == 0x10
}

fn check_half_carry_16_add(a: u16, b: u16) -> bool {
    (a & 0x0FFF) + (b & 0x0FFF) > 0x0FFF
}

fn check_half_carry_8_sub(a: u8, b: u8) -> bool {
    (a & 0xF0) < (b & 0xF0)
}

fn inc8(registers: &mut Registers, a: u8) -> u8 {
    let out = a.wrapping_add(1);

    registers.set_flag(Flag::Z, out == 0);
    registers.set_flag(Flag::N, false);
    registers.set_flag(Flag::H, (out & 0xF) == 0);

    out
}

fn add(registers: &mut Registers, a: u8, v: u8) -> u8 {
    let out = a.wrapping_add(v);

    registers.set_flag(Flag::Z, out == 0);
    registers.set_flag(Flag::N, false);
    registers.set_flag(Flag::H, check_half_carry_8_add(a,v));
    registers.set_flag(Flag::C, out < a);

    out
}

fn adc(registers: &mut Registers, a: u8, v: u8) -> u8 {
    let c = registers.get_flag(Flag::C);

    let n = if c {
        a.wrapping_add(v).wrapping_add(1)
    } else {
        a.wrapping_add(v)
    };

    registers.set_flag(Flag::Z, n == 0);
    registers.set_flag(Flag::N, false);
    registers.set_flag(Flag::H, check_half_carry_8_add(a, v));
    registers.set_flag(Flag::C, n < a);

    n
}


fn add16(registers: &mut Registers, destination: &Registers16, v: u16) {
    let a = registers.get16(destination);
    let n = a.wrapping_add(v);

    registers.set_flag(Flag::Z, n == 0);
    registers.set_flag(Flag::N, false);
    registers.set_flag(Flag::H, check_half_carry_16_add(a, v));
    registers.set_flag(Flag::C, n < a);

    registers.set16(destination, n);
}

fn load_to_memory(
    registers: &mut Registers,
    mmu: &mut mmu::MMU,
    rm: &Registers16,
    rv: &Registers8,
) {
    let m = registers.get16(rm);
    let v = registers.get8(rv);
    mmu.set(m, v);
}

fn load_from_memory(
    registers: &mut Registers,
    mmu: &mut mmu::MMU,
    rm: &Registers16,
    rv: &Registers8,
) {
    let m = registers.get16(rm);
    let v = mmu.get(m);
    registers.set8(rv, v);
}

pub fn push(registers: &mut Registers, mmu: &mut mmu::MMU, r: &Registers16) {
    let sp = registers.get16(&Registers16::SP);
    let v = registers.get16(r);
    let (vh, vl) = bytes::split_u16(v);
    mmu.set(sp - 1, vh);
    mmu.set(sp - 2, vl);
    registers.set16(&Registers16::SP, sp - 2);
}

fn pop(registers: &mut Registers, mmu: &mmu::MMU, r: &Registers16) {
    let sp = registers.get16(&Registers16::SP);
    let v = mmu.get16(sp);
    registers.set16(r, v);
    registers.set16(&Registers16::SP, sp + 2);
}

impl Op {
    pub fn args(&self) -> u8 {
        match self {
            Op::NotImplemented => 0,
            Op::STOP => 1,
            Op::NOP => 0,
            Op::DI => 0,
            Op::EI => 0,
            Op::HALT => 0,
            Op::PrefixCB => 0,

            Op::LD8(_, Destination8::N) => 1,
            Op::LD8(Destination8::N, _) => 2,
            Op::LD8(_, _) => 0,

            Op::LD16(_, Destination16::N) => 2,
            Op::LD16(Destination16::N, _) => 2,
            Op::LD16(_, _) => 0,

            Op::INC8(_) => 0,
            Op::Inc16(_) => 0,
            Op::DEC8(_) => 0,
            Op::DEC16(_) => 0,
            Op::LoadAndInc => 0,
            Op::LoadAndDec => 0,
            Op::LoadAndIncR => 0,
            Op::LoadAndDecR => 0,
            Op::OR(Destination8::N) => 1,
            Op::OR(_) => 0,
            Op::XOR(Destination8::N) => 1,
            Op::XOR(_) => 0,
            Op::AND(Destination8::N) => 1,
            Op::AND(_) => 0,
            Op::SUB(Destination8::N) => 1,
            Op::SUB(_) => 0,
            Op::ADD(Destination8::N) => 1,
            Op::ADD(_) => 0,
            Op::ADC(Destination8::N) => 1,
            Op::ADC(_) => 0,
            Op::ADD16(_, _) => 0,
            Op::JR(_) => 1,
            Op::JP(_) => 2,
            Op::LoadFF00(_, LoadFF00Targets::N) => 1,
            Op::LoadFF00(LoadFF00Targets::N, _) => 1,
            Op::LoadFF00(_, _) => 0,
            Op::CALL(_) => 2,
            Op::PUSH(_) => 0,
            Op::POP(_) => 0,
            Op::RET(_) => 0,
            Op::RETI => 0,
            Op::RST(_) => 0,
            Op::CP(Destination8::N) => 1,
            Op::CP(_) => 0,
            Op::RLCA => 0,
            Op::RRCA => 0,
            Op::RRA => 0,
            Op::DAA => 0,
            Op::CPL => 0,
            Op::CCF => 0,
            Op::SCF => 0,
            Op::SBC(Destination8::N) => 1,
            Op::SBC(_) => 0,

            // cb instructions
            Op::RLC(_) => 0,
            Op::RRC(_) => 0,
            Op::RL(_) => 0,
            Op::RR(_) => 0,
            Op::SLA(_) => 0,
            Op::SRA(_) => 0,
            Op::SWAP(_) => 0,
            Op::SRL(_) => 0,
            Op::BIT(_, _) => 0,
            Op::RES(_, _) => 0,
            Op::SET(_, _) => 0,
        }
    }

    pub fn call(
        &self,
        registers: &mut Registers,
        mmu: &mut mmu::MMU,
        args: &Vec<u8>,
    ) -> u8 {
        match self {
            Op::NotImplemented => 0,
            Op::STOP => {
                println!("stopping!");
                4
            },
            Op::NOP => 4,
            Op::DI => {
                registers.set_interrupts_enabled(false);
                4
            }
            Op::EI => {
                registers.set_interrupts_enabled(true);
                4
            }
            Op::HALT => {
                println!("HALTing!");
                4
            },
            Op::PrefixCB => 4,
            Op::LD8(Destination8::R(r1), Destination8::R(r2)) => {
                let v = registers.get8(r2);
                registers.set8(r1, v);
                4
            }
            Op::LD8(Destination8::R(r1), Destination8::N) => {
                registers.set8(r1, args[0]);
                4
            }
            Op::LD8(Destination8::R(r1), Destination8::Mem(r2)) => {
                let v = mmu.get(registers.get16(r2));
                registers.set8(r1, v);
                8
            }
            Op::LD8(Destination8::Mem(r1), Destination8::R(r2)) => {
                load_to_memory(registers, mmu, r1, r2);
                8
            }
            Op::LD8(Destination8::N, Destination8::R(r)) => {
                let a = bytes::combine_little(args[0], args[1]);
                let v = registers.get8(r);
                mmu.set(a, v);
                8
            }
            Op::LD8(Destination8::Mem(r1), Destination8::N) => {
                let v = args[0];
                let rm = registers.get16(r1);
                mmu.set(rm, v);
                12
            }
            Op::LD8(_, _) => panic!("Not Implemented"),

            Op::LD16(Destination16::R(r1), Destination16::R(r2)) => {
                let v = registers.get16(r2);
                registers.set16(r1, v);
                12
            }
            Op::LD16(Destination16::R(r1), Destination16::N) => {
                let v = bytes::combine_little(args[0], args[1]);
                registers.set16(r1, v);
                12
            }
            Op::LD16(Destination16::N, Destination16::R(r)) => {
                let v = registers.get16(r);
                let (v_high, v_low) = bytes::split_u16(v);
                let a = bytes::combine_little(args[0], args[1]);
                mmu.set(a, v_high);
                mmu.set(a+1, v_low);
                20
            }
            Op::LD16(Destination16::Mem(_), _) => {
                panic!("Not Implemented");
                //20
            }
            Op::LD16(_, _) => panic!("Not Implemented"),

            Op::LoadFF00(LoadFF00Targets::C, LoadFF00Targets::A) => {
                let c = registers.get8(&Registers8::C) as u16;
                let a = registers.get8(&Registers8::A);
                mmu.set(c + 0xFF00, a);
                8
            }
            Op::LoadFF00(LoadFF00Targets::A, LoadFF00Targets::C) => {
                let c = registers.get8(&Registers8::C) as u16;
                let v = mmu.get(c + 0xFF00);
                registers.set8(&Registers8::A, v);
                8
            }
            Op::LoadFF00(LoadFF00Targets::A, LoadFF00Targets::N) => {
                let ma = args[0] as u16;
                let v = mmu.get(ma + 0xFF00);
                registers.set8(&Registers8::A, v);
                8
            }
            Op::LoadFF00(LoadFF00Targets::N, LoadFF00Targets::A) => {
                let a = registers.get8(&Registers8::A);
                let ma = args[0] as u16;
                mmu.set(ma + 0xFF00, a);
                8
            }
            Op::LoadFF00(_, _) => panic!("invalid loadFF00 inputs"),

            Op::LoadAndDec => {
                load_to_memory(registers, mmu, &Registers16::HL, &Registers8::A);
                registers.dec_hl();
                8
            }
            Op::LoadAndInc => {
                load_to_memory(registers, mmu, &Registers16::HL, &Registers8::A);
                registers.inc_hl();
                8
            }
            Op::LoadAndIncR => {
                load_from_memory(registers, mmu, &Registers16::HL, &Registers8::A);
                registers.inc_hl();
                8
            }
            Op::LoadAndDecR => {
                load_from_memory(registers, mmu, &Registers16::HL, &Registers8::A);
                registers.dec_hl();
                8
            }
            Op::JR(JrArgs::CheckFlag(f)) => {
                if check_flags(registers, f) {
                    jump_relative(registers, args[0] as i8);
                    16
                } else {
                    12
                }
            }
            Op::JR(JrArgs::N) => {
                jump_relative(registers, args[0] as i8);
                12
            }

            Op::JP(JpArgs::CheckFlag(f)) => {
                if check_flags(registers, f) {
                    jump(registers, bytes::combine_little(args[0], args[1]));
                    16
                } else {
                    12
                }
            }
            Op::JP(JpArgs::N) => {
                jump(registers, bytes::combine_little(args[0], args[1]));
                16
            }
            Op::JP(JpArgs::HL) => {
                let m = registers.get16(&Registers16::HL);
                let v = mmu.get16(m);
                jump(registers, v);
                16
            }

            Op::CALL(None) => {
                call(registers, mmu, bytes::combine_little(args[0], args[1]));
                24
            }

            Op::CALL(Some(flag)) => {
                if check_flags(registers, &flag) {
                    call(registers, mmu, bytes::combine_little(args[0], args[1]));
                    24
                } else {
                    12
                }
            }

            Op::PUSH(r) => {
                push(registers, mmu, r);
                16
            }
            Op::POP(r) => {
                pop(registers, mmu, r);
                12
            }
            Op::RET(None) => {
                ret(registers, mmu);
                16
            }
            Op::RET(Some(f)) => {
                if check_flags(registers, f) {
                    ret(registers, mmu);
                    20
                } else {
                    8
                }

            }
            Op::RST(r) => {
                rst(registers, mmu, r);
                32
            }
            Op::RETI => {
                ret(registers, mmu);
                registers.set_interrupts_enabled(true);
                16
            }

            // ALU Codes
            Op::INC8(Destination8::R(r)) => {
                let v = registers.get8(r);

                let out = inc8(registers, v);

                registers.set8(r, out);
                4
            }
            Op::INC8(Destination8::Mem(r)) => {
                let rm = registers.get16(r);
                let v = mmu.get(rm);
                let out = inc8(registers, v);

                mmu.set(rm, out);
                12
            }
            Op::INC8(Destination8::N) => panic!("Not Implemented"),

            Op::Inc16(Destination16::R(r)) => {
                let v = registers.get16(r);
                let n = v.wrapping_add(1);

                registers.set16(r, n);
                8
            }
            Op::Inc16(Destination16::Mem(_)) => panic!("Not Implemented"),
            Op::Inc16(Destination16::N) => panic!("Not Implemented"),

            Op::DEC8(Destination8::R(r)) => {
                let v = registers.get8(r);
                let n = dec8(registers, v);

                registers.set8(r, n);
                4
            }
            Op::DEC8(Destination8::Mem(r)) => {
                let a = registers.get16(r);
                let v = mmu.get(a);
                let n = dec8(registers, v);

                mmu.set(a, n);
                12
            }
            Op::DEC8(Destination8::N) => panic!("Not Implemented"),
            Op::DEC16(Destination16::R(r)) => {
                let v = registers.get16(r);
                let n = dec16(registers, v);

                registers.set16(r, n);
                8
            }
            Op::DEC16(Destination16::Mem(_)) => panic!("Not Implemented"),
            Op::DEC16(Destination16::N) => panic!("Not Implemented"),

            Op::CP(Destination8::N) => {
                compare(registers, args[0]);
                8
            }
            Op::CP(Destination8::R(r)) => {
                let v = registers.get8(r);
                compare(registers, v);
                4
            }
            Op::CP(Destination8::Mem(r)) => {
                let m = registers.get16(r);
                let v = mmu.get(m);
                compare(registers, v);
                8
            }
            Op::AND(Destination8::R(r)) => {
                let v = registers.get8(r);
                and(registers,v);
                4
            }
            Op::AND(Destination8::Mem(_)) => panic!("Not Implemented"),
            Op::AND(Destination8::N) => {
                and(registers, args[0]);
                8
            }

            Op::OR(Destination8::R(r)) => {
                let v = registers.get8(r);
                or(registers, v);
                4
            }
            Op::OR(Destination8::Mem(r)) => {
                let rm = registers.get16(r);
                let v = mmu.get(rm);
                or(registers, v);
                8
            }
            Op::OR(Destination8::N) => panic!("Not Implemented"),

            Op::XOR(Destination8::R(r)) => {
                let v = registers.get8(r);
                xor(registers, v);
                4
            }
            Op::XOR(Destination8::Mem(r)) => {
                let m = registers.get16(r);
                let v = mmu.get(m);
                xor(registers, v);
                8
            }
            Op::XOR(Destination8::N) => {
                xor(registers, args[0]);
                8
            }

            Op::SBC(Destination8::R(r)) => {
                let ra = &Registers8::A;
                let a = registers.get8(ra);

                let v = registers.get8(r);

                let out = sbc(registers, a, v);

                registers.set8(ra, out);
                4
            }
            Op::SBC(Destination8::Mem(r)) => {
                let ra = &Registers8::A;
                let a = registers.get8(ra);
                let mr = registers.get16(r);
                let v = mmu.get(mr);
                let out = sbc(registers, a, v);

                registers.set8(ra, out);
                8
            }
            Op::SBC(Destination8::N) => {
                let ra = &Registers8::A;
                let a = registers.get8(ra);
                let v = args[0];
                let out = sbc(registers, a, v);
                registers.set8(ra, out);
                4
            }

            Op::SUB(Destination8::R(r)) => {
                let ra = &Registers8::A;
                let a = registers.get8(ra);

                let v = registers.get8(r);

                let out = sub(registers, a, v);

                registers.set8(ra, out);

                4
            }

            Op::SUB(Destination8::Mem(r)) => {
                let ra = &Registers8::A;
                let a = registers.get8(ra);

                let m = registers.get16(r);
                let v = mmu.get(m);

                let out = sub(registers, a, v);

                registers.set8(ra, out);

                8
            }

            Op::SUB(Destination8::N) => {
                let ra = &Registers8::A;
                let a = registers.get8(ra);

                let out = sub(registers, a, args[0]);

                registers.set8(ra, out);

                8
            }

            Op::ADD(Destination8::R(r)) => {
                let ra = &Registers8::A;
                let a = registers.get8(ra);

                let v = registers.get8(r);

                let out = add(registers, a, v);

                registers.set8(ra, out);
                4
            }

            Op::ADD(Destination8::Mem(r)) => {
                let ra = &Registers8::A;
                let a = registers.get8(ra);

                let m = registers.get16(r);
                let v = mmu.get(m);

                let out = add(registers, a, v);

                registers.set8(ra, out);

                8
            }
            Op::ADD(Destination8::N) => {
                let ra = &Registers8::A;
                let a = registers.get8(ra);

                let v = args[0];

                let out = add(registers, a, v);

                registers.set8(ra, out);
                4
            }

            Op::ADC(Destination8::R(r)) => {
                let ra = &Registers8::A;
                let a = registers.get8(ra);

                let v = registers.get8(r);

                let out = adc(registers, a, v);

                registers.set8(ra, out);
                4
            }
            Op::ADC(Destination8::Mem(r)) => {
                let ra = &Registers8::A;
                let a = registers.get8(ra);

                let mr = registers.get16(r);
                let v = mmu.get(mr);

                let out = adc(registers, a, v);

                registers.set8(ra, out);
                8
            }
            Op::ADC(Destination8::N) => {
                let ra = &Registers8::A;
                let a = registers.get8(ra);

                let v = args[0];

                let out = adc(registers, a, v);

                registers.set8(ra, out);
                4
            }

            Op::ADD16(ADD16Args::R(r1), ADD16Args::R(r2)) => {
                let v = registers.get16(r2);
                add16(registers, r1, v);
                8
            }

            Op::RLCA => {
                let v = registers.get8(&Registers8::A);
                let out = rlc(registers, v);
                registers.set8(&Registers8::A, out);
                8
            }
            Op::RRA => {
                let v = registers.get8(&Registers8::A);
                let out = rr(registers, v);
                registers.set8(&Registers8::A, out);
                8
            }
            Op::DAA => {
                /* When this instruction is executed, the A register is BCD corrected using the
                 * contents of the flags. The exact process is the following: if the least
                 * significant four bits of A contain a non-BCD digit (i. e. it is greater than 9)
                 * or the H flag is set, then $06 is added to the register. Then the four most
                 * significant bits are checked. If this more significant digit also happens to be
                 * greater than 9 or the C flag is set, then $60 is added. */
                4
            }
            Op::RRCA => {
                let v = registers.get8(&Registers8::A);
                let out = rrc(registers, v);
                registers.set8(&Registers8::A, out);
                8
            }
            Op::CPL => {
                let r = &Registers8::A;
                let v = registers.get8(r);

                let out = cpl(registers, v);

                registers.set8(r, out);
                8
            }

            Op::CCF => {
                let v = !registers.get_flag(Flag::C);
                registers.set_flag(Flag::C, v);
                4
            }
            Op::SCF => {
                registers.set_flag(Flag::C, true);
                4
            }


            // Cb instructions
            Op::RLC(Destination8::R(r)) => {
                let v = registers.get8(r);
                let out = rlc(registers, v);
                registers.set8(r, out);
                8
            }
            Op::RLC(Destination8::Mem(r)) => {
                let rm = registers.get16(r);
                let v = mmu.get(rm);
                let out = rlc(registers, v);

                mmu.set(rm, out);
                16
            }
            Op::RLC(Destination8::N) => panic!("Not Implemented"),

            Op::RRC(Destination8::R(r)) => {
                let v = registers.get8(r);
                let out = rrc(registers, v);
                registers.set8(r, out);
                8
            }
            Op::RRC(Destination8::Mem(r)) => {
                let rm = registers.get16(r);
                let v = mmu.get(rm);
                let out = rrc(registers, v);
                mmu.set(rm, out);
                16
            }
            Op::RRC(Destination8::N) => panic!("Not Implemented"),

            Op::RL(Destination8::R(r)) => {
                let v = registers.get8(r);
                let out = rl(registers, v);
                registers.set8(r, out);
                8
            }
            Op::RL(Destination8::Mem(_)) => panic!("Not Implemented"),
            Op::RL(Destination8::N) => panic!("Not Implemented"),

            Op::RR(Destination8::R(r)) => {
                let v = registers.get8(r);
                let out = rr(registers, v);
                registers.set8(r, out);
                8
            }
            Op::RR(Destination8::Mem(r)) => {
                let rm = registers.get16(r);
                let v = mmu.get(rm);
                let out = rr(registers, v);
                mmu.set(rm, out);
                16
            }
            Op::RR(Destination8::N) => panic!("Not Implemented"),

            Op::SLA(Destination8::R(r)) => {
                let v = registers.get8(r);
                let out = sla(registers, v);
                registers.set8(r, out);
                8
            }
            Op::SLA(Destination8::Mem(r)) => {
                let rm = registers.get16(r);
                let v = mmu.get(rm);
                let out = sla(registers, v);
                mmu.set(rm, out);
                16
            }
            Op::SLA(Destination8::N) => panic!("Not Implemented"),

            Op::SRA(Destination8::R(r)) => {
                let v = registers.get8(r);
                let out = sra(registers, v);
                registers.set8(r, out);
                8
            }
            Op::SRA(Destination8::Mem(r)) => {
                let rm = registers.get16(r);
                let v = mmu.get(rm);
                let out = sra(registers, v);
                mmu.set(rm, out);
                16
            }
            Op::SRA(Destination8::N) => panic!("Not Implemented"),

            Op::SWAP(Destination8::R(r)) => {
                let v = registers.get8(r);
                let out = swap(registers, v);
                registers.set8(r, out);
                8
            }
            Op::SWAP(Destination8::Mem(r)) => {
                let rm = registers.get16(r);
                let v = mmu.get(rm);
                let out = swap(registers, v);
                mmu.set(rm, out);
                16
            }
            Op::SWAP(Destination8::N) => panic!("Not Implemented"),

            Op::SRL(Destination8::R(r)) => {
                let v = registers.get8(r);
                let out = srl(registers, v);
                registers.set8(r, out);
                8
            }
            Op::SRL(Destination8::Mem(_)) => panic!("Not Implemented"),
            Op::SRL(Destination8::N) => panic!("Not Implemented"),


            Op::BIT(location, Destination8::R(r)) => {
                let v = registers.get8(r);
                bit(registers, v, *location);
                8
            }
            Op::BIT(_, Destination8::Mem(_)) => panic!("Not Implemented"),
            Op::BIT(_, Destination8::N) => panic!("Not Implemented"),

            Op::RES(location, Destination8::R(r)) => {
                let v = registers.get8(r);
                let out = res(v, *location);
                registers.set8(r, out);
                8
            }
            Op::RES(_, Destination8::Mem(_)) => panic!("Not Implemented"),
            Op::RES(_, Destination8::N) => panic!("Not Implemented"),

            Op::SET(location, Destination8::R(r)) => {
                let v = registers.get8(r);
                let out = set(v, *location);
                registers.set8(r, out);
                8
            }
            Op::SET(_, Destination8::Mem(_)) => panic!("Not Implemented"),
            Op::SET(_, Destination8::N) => panic!("Not Implemented"),
        }
    }
}

pub struct Instructions {
    instructions: Vec<Op>,
    cb_instructions: Vec<Op>,
}

impl Instructions {
    pub fn get(&self, opcode: u8) -> &Op {
        let o = opcode as usize;
        &self.instructions[o]
    }

    pub fn get_cb(&self, opcode: u8) -> &Op {
        let o = opcode as usize;
        &self.cb_instructions[o]
    }
}

pub fn new() -> Instructions {
    let mut instructions = vec![Op::NotImplemented; 256];

    instructions[0x0000] = Op::NOP;
    instructions[0x0001] = Op::LD16(Destination16::R(Registers16::BC), Destination16::N);
    instructions[0x0002] = Op::LD8(Destination8::Mem(Registers16::BC), Destination8::R(Registers8::A));
    instructions[0x0003] = Op::Inc16(Destination16::R(Registers16::BC));
    instructions[0x0004] = Op::INC8(Destination8::R(Registers8::B));
    instructions[0x0005] = Op::DEC8(Destination8::R(Registers8::B));
    instructions[0x0006] = Op::LD8(Destination8::R(Registers8::B), Destination8::N);
    instructions[0x0007] = Op::RLCA;
    instructions[0x0008] = Op::LD16(Destination16::N, Destination16::R(Registers16::SP));
    instructions[0x0009] = Op::ADD16(ADD16Args::R(Registers16::HL), ADD16Args::R(Registers16::BC));
    instructions[0x000A] = Op::LD8(Destination8::R(Registers8::C), Destination8::Mem(Registers16::BC));
    instructions[0x000B] = Op::DEC16(Destination16::R(Registers16::BC));
    instructions[0x000C] = Op::INC8(Destination8::R(Registers8::C));
    instructions[0x000D] = Op::DEC8(Destination8::R(Registers8::C));
    instructions[0x000E] = Op::LD8(Destination8::R(Registers8::C), Destination8::N);
    instructions[0x000F] = Op::RRCA;

    instructions[0x0010] = Op::STOP;
    instructions[0x0011] = Op::LD16(Destination16::R(Registers16::DE), Destination16::N);
    instructions[0x0012] = Op::LD8(Destination8::Mem(Registers16::DE), Destination8::R(Registers8::A));
    instructions[0x0013] = Op::Inc16(Destination16::R(Registers16::DE));
    instructions[0x0014] = Op::INC8(Destination8::R(Registers8::D));
    instructions[0x0015] = Op::DEC8(Destination8::R(Registers8::D));
    instructions[0x0016] = Op::LD8(Destination8::R(Registers8::D), Destination8::N);
    instructions[0x0017] = Op::RL(Destination8::R(Registers8::A));
    instructions[0x0018] = Op::JR(JrArgs::N);
    instructions[0x0019] = Op::ADD16(ADD16Args::R(Registers16::HL), ADD16Args::R(Registers16::DE));
    instructions[0x001A] = Op::LD8(Destination8::R(Registers8::A), Destination8::Mem(Registers16::DE));
    instructions[0x001B] = Op::DEC16(Destination16::R(Registers16::DE));
    instructions[0x001C] = Op::INC8(Destination8::R(Registers8::E));
    instructions[0x001D] = Op::DEC8(Destination8::R(Registers8::E));
    instructions[0x001E] = Op::LD8(Destination8::R(Registers8::E), Destination8::N);
    instructions[0x001F] = Op::RRA;

    instructions[0x0020] = Op::JR(JrArgs::CheckFlag(CheckFlag::NZ));
    instructions[0x0021] = Op::LD16(Destination16::R(Registers16::HL), Destination16::N);
    instructions[0x0022] = Op::LoadAndInc;
    instructions[0x0023] = Op::Inc16(Destination16::R(Registers16::HL));
    instructions[0x0024] = Op::INC8(Destination8::R(Registers8::H));
    instructions[0x0025] = Op::DEC8(Destination8::R(Registers8::H));
    instructions[0x0026] = Op::LD8(Destination8::R(Registers8::H), Destination8::N);
    instructions[0x0027] = Op::NotImplemented;
    instructions[0x0028] = Op::JR(JrArgs::CheckFlag(CheckFlag::Z));
    instructions[0x0029] = Op::ADD16(ADD16Args::R(Registers16::HL), ADD16Args::R(Registers16::HL));
    instructions[0x002A] = Op::LoadAndIncR;
    instructions[0x002B] = Op::DEC16(Destination16::R(Registers16::HL));
    instructions[0x002C] = Op::INC8(Destination8::R(Registers8::L));
    instructions[0x002D] = Op::DEC8(Destination8::R(Registers8::L));
    instructions[0x002E] = Op::LD8(Destination8::R(Registers8::L), Destination8::N);
    instructions[0x002F] = Op::CPL;

    instructions[0x0030] = Op::JR(JrArgs::CheckFlag(CheckFlag::NC));
    instructions[0x0031] = Op::LD16(Destination16::R(Registers16::SP), Destination16::N);
    instructions[0x0032] = Op::LoadAndDec;
    instructions[0x0033] = Op::Inc16(Destination16::R(Registers16::SP));
    instructions[0x0034] = Op::INC8(Destination8::Mem(Registers16::HL));
    instructions[0x0035] = Op::DEC8(Destination8::Mem(Registers16::HL));
    instructions[0x0036] = Op::LD8(Destination8::Mem(Registers16::HL), Destination8::N);
    instructions[0x0037] = Op::SCF;
    instructions[0x0038] = Op::JR(JrArgs::CheckFlag(CheckFlag::C));
    instructions[0x0039] = Op::ADD16(ADD16Args::R(Registers16::HL), ADD16Args::R(Registers16::SP));
    instructions[0x003A] = Op::LoadAndDecR;
    instructions[0x003B] = Op::DEC16(Destination16::R(Registers16::SP));
    instructions[0x003C] = Op::INC8(Destination8::R(Registers8::A));
    instructions[0x003D] = Op::DEC8(Destination8::R(Registers8::A));
    instructions[0x003E] = Op::LD8(Destination8::R(Registers8::A), Destination8::N);
    instructions[0x003F] = Op::CCF;

    instructions[0x0040] = Op::LD8(Destination8::R(Registers8::B), Destination8::R(Registers8::B));
    instructions[0x0041] = Op::LD8(Destination8::R(Registers8::B), Destination8::R(Registers8::C));
    instructions[0x0042] = Op::LD8(Destination8::R(Registers8::B), Destination8::R(Registers8::D));
    instructions[0x0043] = Op::LD8(Destination8::R(Registers8::B), Destination8::R(Registers8::E));
    instructions[0x0044] = Op::LD8(Destination8::R(Registers8::B), Destination8::R(Registers8::H));
    instructions[0x0045] = Op::LD8(Destination8::R(Registers8::B), Destination8::R(Registers8::L));
    instructions[0x0046] = Op::LD8(Destination8::R(Registers8::B), Destination8::Mem(Registers16::HL));
    instructions[0x0047] = Op::LD8(Destination8::R(Registers8::B), Destination8::R(Registers8::A));
    instructions[0x0048] = Op::LD8(Destination8::R(Registers8::C), Destination8::R(Registers8::B));
    instructions[0x0049] = Op::LD8(Destination8::R(Registers8::C), Destination8::R(Registers8::C));
    instructions[0x004A] = Op::LD8(Destination8::R(Registers8::C), Destination8::R(Registers8::D));
    instructions[0x004B] = Op::LD8(Destination8::R(Registers8::C), Destination8::R(Registers8::E));
    instructions[0x004C] = Op::LD8(Destination8::R(Registers8::C), Destination8::R(Registers8::H));
    instructions[0x004D] = Op::LD8(Destination8::R(Registers8::C), Destination8::R(Registers8::L));
    instructions[0x004E] = Op::LD8(Destination8::R(Registers8::C), Destination8::Mem(Registers16::HL));
    instructions[0x004F] = Op::LD8(Destination8::R(Registers8::C), Destination8::R(Registers8::A));

    instructions[0x0050] = Op::LD8(Destination8::R(Registers8::D), Destination8::R(Registers8::B));
    instructions[0x0051] = Op::LD8(Destination8::R(Registers8::D), Destination8::R(Registers8::C));
    instructions[0x0052] = Op::LD8(Destination8::R(Registers8::D), Destination8::R(Registers8::D));
    instructions[0x0053] = Op::LD8(Destination8::R(Registers8::D), Destination8::R(Registers8::E));
    instructions[0x0054] = Op::LD8(Destination8::R(Registers8::D), Destination8::R(Registers8::H));
    instructions[0x0055] = Op::LD8(Destination8::R(Registers8::D), Destination8::R(Registers8::L));
    instructions[0x0056] = Op::LD8(Destination8::R(Registers8::D), Destination8::Mem(Registers16::HL));
    instructions[0x0057] = Op::LD8(Destination8::R(Registers8::D), Destination8::R(Registers8::A));
    instructions[0x0058] = Op::LD8(Destination8::R(Registers8::E), Destination8::R(Registers8::B));
    instructions[0x0059] = Op::LD8(Destination8::R(Registers8::E), Destination8::R(Registers8::C));
    instructions[0x005A] = Op::LD8(Destination8::R(Registers8::E), Destination8::R(Registers8::D));
    instructions[0x005B] = Op::LD8(Destination8::R(Registers8::E), Destination8::R(Registers8::E));
    instructions[0x005C] = Op::LD8(Destination8::R(Registers8::E), Destination8::R(Registers8::H));
    instructions[0x005D] = Op::LD8(Destination8::R(Registers8::E), Destination8::R(Registers8::L));
    instructions[0x005E] = Op::LD8(Destination8::R(Registers8::E), Destination8::Mem(Registers16::HL));
    instructions[0x005F] = Op::LD8(Destination8::R(Registers8::E), Destination8::R(Registers8::A));

    instructions[0x0060] = Op::LD8(Destination8::R(Registers8::H), Destination8::R(Registers8::B));
    instructions[0x0061] = Op::LD8(Destination8::R(Registers8::H), Destination8::R(Registers8::C));
    instructions[0x0062] = Op::LD8(Destination8::R(Registers8::H), Destination8::R(Registers8::D));
    instructions[0x0063] = Op::LD8(Destination8::R(Registers8::H), Destination8::R(Registers8::E));
    instructions[0x0064] = Op::LD8(Destination8::R(Registers8::H), Destination8::R(Registers8::H));
    instructions[0x0065] = Op::LD8(Destination8::R(Registers8::H), Destination8::R(Registers8::L));
    instructions[0x0066] = Op::LD8(Destination8::R(Registers8::H), Destination8::Mem(Registers16::HL));
    instructions[0x0067] = Op::LD8(Destination8::R(Registers8::H), Destination8::R(Registers8::A));
    instructions[0x0068] = Op::LD8(Destination8::R(Registers8::L), Destination8::R(Registers8::B));
    instructions[0x0069] = Op::LD8(Destination8::R(Registers8::L), Destination8::R(Registers8::C));
    instructions[0x006A] = Op::LD8(Destination8::R(Registers8::L), Destination8::R(Registers8::D));
    instructions[0x006B] = Op::LD8(Destination8::R(Registers8::L), Destination8::R(Registers8::E));
    instructions[0x006C] = Op::LD8(Destination8::R(Registers8::L), Destination8::R(Registers8::H));
    instructions[0x006D] = Op::LD8(Destination8::R(Registers8::L), Destination8::R(Registers8::L));
    instructions[0x006E] = Op::LD8(Destination8::R(Registers8::L), Destination8::Mem(Registers16::HL));
    instructions[0x006F] = Op::LD8(Destination8::R(Registers8::L), Destination8::R(Registers8::A));

    instructions[0x0070] = Op::LD8(Destination8::Mem(Registers16::HL), Destination8::R(Registers8::B));
    instructions[0x0071] = Op::LD8(Destination8::Mem(Registers16::HL), Destination8::R(Registers8::C));
    instructions[0x0072] = Op::LD8(Destination8::Mem(Registers16::HL), Destination8::R(Registers8::D));
    instructions[0x0073] = Op::LD8(Destination8::Mem(Registers16::HL), Destination8::R(Registers8::E));
    instructions[0x0074] = Op::LD8(Destination8::Mem(Registers16::HL), Destination8::R(Registers8::H));
    instructions[0x0075] = Op::LD8(Destination8::Mem(Registers16::HL), Destination8::R(Registers8::L));
    instructions[0x0076] = Op::HALT;
    instructions[0x0077] = Op::LD8(Destination8::Mem(Registers16::HL), Destination8::R(Registers8::A));
    instructions[0x0078] = Op::LD8(Destination8::R(Registers8::A), Destination8::R(Registers8::B));
    instructions[0x0079] = Op::LD8(Destination8::R(Registers8::A), Destination8::R(Registers8::C));
    instructions[0x007A] = Op::LD8(Destination8::R(Registers8::A), Destination8::R(Registers8::D));
    instructions[0x007B] = Op::LD8(Destination8::R(Registers8::A), Destination8::R(Registers8::E));
    instructions[0x007C] = Op::LD8(Destination8::R(Registers8::A), Destination8::R(Registers8::H));
    instructions[0x007D] = Op::LD8(Destination8::R(Registers8::A), Destination8::R(Registers8::L));
    instructions[0x007E] = Op::LD8(Destination8::R(Registers8::A), Destination8::Mem(Registers16::HL));
    instructions[0x007F] = Op::LD8(Destination8::R(Registers8::A), Destination8::R(Registers8::A));
 
    instructions[0x0080] = Op::ADD(Destination8::R(Registers8::B));
    instructions[0x0081] = Op::ADD(Destination8::R(Registers8::C));
    instructions[0x0082] = Op::ADD(Destination8::R(Registers8::D));
    instructions[0x0083] = Op::ADD(Destination8::R(Registers8::E));
    instructions[0x0084] = Op::ADD(Destination8::R(Registers8::H));
    instructions[0x0085] = Op::ADD(Destination8::R(Registers8::L));
    instructions[0x0086] = Op::ADD(Destination8::Mem(Registers16::HL));
    instructions[0x0087] = Op::ADD(Destination8::R(Registers8::A));
    instructions[0x0088] = Op::ADC(Destination8::R(Registers8::B));
    instructions[0x0089] = Op::ADC(Destination8::R(Registers8::C));
    instructions[0x008A] = Op::ADC(Destination8::R(Registers8::D));
    instructions[0x008B] = Op::ADC(Destination8::R(Registers8::E));
    instructions[0x008C] = Op::ADC(Destination8::R(Registers8::H));
    instructions[0x008D] = Op::ADC(Destination8::R(Registers8::L));
    instructions[0x008E] = Op::ADC(Destination8::Mem(Registers16::HL));
    instructions[0x008F] = Op::ADC(Destination8::R(Registers8::A));

    instructions[0x0090] = Op::SUB(Destination8::R(Registers8::B));
    instructions[0x0091] = Op::SUB(Destination8::R(Registers8::C));
    instructions[0x0092] = Op::SUB(Destination8::R(Registers8::D));
    instructions[0x0093] = Op::SUB(Destination8::R(Registers8::E));
    instructions[0x0094] = Op::SUB(Destination8::R(Registers8::H));
    instructions[0x0095] = Op::SUB(Destination8::R(Registers8::L));
    instructions[0x0096] = Op::SUB(Destination8::Mem(Registers16::HL));
    instructions[0x0097] = Op::SUB(Destination8::R(Registers8::A));
    instructions[0x0098] = Op::SBC(Destination8::R(Registers8::B));
    instructions[0x0099] = Op::SBC(Destination8::R(Registers8::C));
    instructions[0x009A] = Op::SBC(Destination8::R(Registers8::D));
    instructions[0x009B] = Op::SBC(Destination8::R(Registers8::E));
    instructions[0x009C] = Op::SBC(Destination8::R(Registers8::H));
    instructions[0x009D] = Op::SBC(Destination8::R(Registers8::L));
    instructions[0x009E] = Op::SBC(Destination8::Mem(Registers16::HL));
    instructions[0x009F] = Op::SBC(Destination8::R(Registers8::A));

    instructions[0x00A0] = Op::AND(Destination8::R(Registers8::B));
    instructions[0x00A1] = Op::AND(Destination8::R(Registers8::C));
    instructions[0x00A2] = Op::AND(Destination8::R(Registers8::D));
    instructions[0x00A3] = Op::AND(Destination8::R(Registers8::E));
    instructions[0x00A4] = Op::AND(Destination8::R(Registers8::H));
    instructions[0x00A5] = Op::AND(Destination8::R(Registers8::L));
    instructions[0x00A6] = Op::AND(Destination8::Mem(Registers16::HL));
    instructions[0x00A7] = Op::AND(Destination8::R(Registers8::A));
    instructions[0x00A8] = Op::XOR(Destination8::R(Registers8::B));
    instructions[0x00A9] = Op::XOR(Destination8::R(Registers8::C));
    instructions[0x00AA] = Op::XOR(Destination8::R(Registers8::D));
    instructions[0x00AB] = Op::XOR(Destination8::R(Registers8::E));
    instructions[0x00AC] = Op::XOR(Destination8::R(Registers8::H));
    instructions[0x00AD] = Op::XOR(Destination8::R(Registers8::L));
    instructions[0x00AE] = Op::XOR(Destination8::Mem(Registers16::HL));
    instructions[0x00AF] = Op::XOR(Destination8::R(Registers8::A));

    instructions[0x00B0] = Op::OR(Destination8::R(Registers8::B));
    instructions[0x00B1] = Op::OR(Destination8::R(Registers8::C));
    instructions[0x00B2] = Op::OR(Destination8::R(Registers8::D));
    instructions[0x00B3] = Op::OR(Destination8::R(Registers8::E));
    instructions[0x00B4] = Op::OR(Destination8::R(Registers8::H));
    instructions[0x00B5] = Op::OR(Destination8::R(Registers8::L));
    instructions[0x00B6] = Op::OR(Destination8::Mem(Registers16::HL));
    instructions[0x00B7] = Op::OR(Destination8::R(Registers8::A));
    instructions[0x00B8] = Op::CP(Destination8::R(Registers8::B));
    instructions[0x00B9] = Op::CP(Destination8::R(Registers8::C));
    instructions[0x00BA] = Op::CP(Destination8::R(Registers8::D));
    instructions[0x00BB] = Op::CP(Destination8::R(Registers8::E));
    instructions[0x00BC] = Op::CP(Destination8::R(Registers8::H));
    instructions[0x00BD] = Op::CP(Destination8::R(Registers8::L));
    instructions[0x00BE] = Op::CP(Destination8::Mem(Registers16::HL));
    instructions[0x00BF] = Op::CP(Destination8::R(Registers8::A));

    instructions[0x00C0] = Op::RET(Some(CheckFlag::NZ));
    instructions[0x00C1] = Op::POP(Registers16::BC);
    instructions[0x00C2] = Op::JP(JpArgs::CheckFlag(CheckFlag::NZ));
    instructions[0x00C3] = Op::JP(JpArgs::N);
    instructions[0x00C4] = Op::CALL(Some(CheckFlag::NZ));
    instructions[0x00C5] = Op::PUSH(Registers16::BC);
    instructions[0x00C6] = Op::ADD(Destination8::N);
    instructions[0x00C7] = Op::RST(RstArgs::H00);
    instructions[0x00C8] = Op::RET(Some(CheckFlag::Z));
    instructions[0x00C9] = Op::RET(None);
    instructions[0x00CA] = Op::JP(JpArgs::CheckFlag(CheckFlag::Z));
    instructions[0x00CB] = Op::PrefixCB;
    instructions[0x00CC] = Op::CALL(Some(CheckFlag::Z));
    instructions[0x00CD] = Op::CALL(None);
    instructions[0x00CE] = Op::ADC(Destination8::N);
    instructions[0x00CF] = Op::RST(RstArgs::H08);

    instructions[0x00D0] = Op::RET(Some(CheckFlag::NC));
    instructions[0x00D1] = Op::POP(Registers16::DE);
    instructions[0x00D2] = Op::JP(JpArgs::CheckFlag(CheckFlag::NC));
    instructions[0x00D3] = Op::NotImplemented;
    instructions[0x00D4] = Op::CALL(Some(CheckFlag::Z));
    instructions[0x00D5] = Op::PUSH(Registers16::DE);
    instructions[0x00D6] = Op::SUB(Destination8::N);
    instructions[0x00D7] = Op::RST(RstArgs::H10);
    instructions[0x00D8] = Op::RET(Some(CheckFlag::C));
    instructions[0x00D9] = Op::RETI;
    instructions[0x00DA] = Op::JP(JpArgs::CheckFlag(CheckFlag::C));
    instructions[0x00DB] = Op::NotImplemented;
    instructions[0x00DC] = Op::CALL(Some(CheckFlag::C));
    instructions[0x00DD] = Op::NotImplemented;
    instructions[0x00DE] = Op::SBC(Destination8::N);
    instructions[0x00DF] = Op::RST(RstArgs::H18);

    instructions[0x00E0] = Op::LoadFF00(LoadFF00Targets::N, LoadFF00Targets::A);
    instructions[0x00E1] = Op::POP(Registers16::HL);
    instructions[0x00E2] = Op::LoadFF00(LoadFF00Targets::C, LoadFF00Targets::A);
    instructions[0x00E3] = Op::NotImplemented;
    instructions[0x00E4] = Op::NotImplemented;
    instructions[0x00E5] = Op::PUSH(Registers16::HL);
    instructions[0x00E6] = Op::AND(Destination8::N);
    instructions[0x00E7] = Op::RST(RstArgs::H20);
    // Note that this instruction actually takes a signed 8bit value
    // instructions[0x00E8] = Op::ADD16(ADD16Args::R(Registers16::SP), ADD16Args::N);
    instructions[0x00E9] = Op::JP(JpArgs::HL);
    instructions[0x00EA] = Op::LD8(Destination8::N, Destination8::R(Registers8::A));
    instructions[0x00EB] = Op::NotImplemented;
    instructions[0x00EC] = Op::NotImplemented;
    instructions[0x00ED] = Op::NotImplemented;
    instructions[0x00EE] = Op::XOR(Destination8::N);
    instructions[0x00EF] = Op::RST(RstArgs::H28);

    instructions[0x00F0] = Op::LoadFF00(LoadFF00Targets::A, LoadFF00Targets::N);
    instructions[0x00F1] = Op::POP(Registers16::AF);
    instructions[0x00F2] = Op::LoadFF00(LoadFF00Targets::A, LoadFF00Targets::C);
    instructions[0x00F3] = Op::DI;
    instructions[0x00F4] = Op::NotImplemented;
    instructions[0x00F5] = Op::PUSH(Registers16::AF);
    instructions[0x00F6] = Op::OR(Destination8::N);
    instructions[0x00F7] = Op::RST(RstArgs::H30);
    // instructions[0x00F8] = LD HL,SP+r8
    instructions[0x00F9] = Op::LD16(Destination16::R(Registers16::SP), Destination16::R(Registers16::HL));
    instructions[0x00FA] = Op::LD8(Destination8::R(Registers8::A), Destination8::N);
    instructions[0x00FB] = Op::EI;
    instructions[0x00FC] = Op::NotImplemented;
    instructions[0x00FD] = Op::NotImplemented;
    instructions[0x00FE] = Op::CP(Destination8::N);
    instructions[0x00FF] = Op::RST(RstArgs::H38);

    let mut cb_instructions = vec![Op::NotImplemented; 256];
    cb_instructions[0x0000] = Op::RLC(Destination8::R(Registers8::B));
    cb_instructions[0x0001] = Op::RLC(Destination8::R(Registers8::C));
    cb_instructions[0x0002] = Op::RLC(Destination8::R(Registers8::D));
    cb_instructions[0x0003] = Op::RLC(Destination8::R(Registers8::E));
    cb_instructions[0x0004] = Op::RLC(Destination8::R(Registers8::H));
    cb_instructions[0x0005] = Op::RLC(Destination8::R(Registers8::L));
    cb_instructions[0x0006] = Op::RLC(Destination8::Mem(Registers16::HL));
    cb_instructions[0x0007] = Op::RLC(Destination8::R(Registers8::A));
    cb_instructions[0x0008] = Op::RRC(Destination8::R(Registers8::B));
    cb_instructions[0x0009] = Op::RRC(Destination8::R(Registers8::C));
    cb_instructions[0x000A] = Op::RRC(Destination8::R(Registers8::D));
    cb_instructions[0x000B] = Op::RRC(Destination8::R(Registers8::E));
    cb_instructions[0x000C] = Op::RRC(Destination8::R(Registers8::H));
    cb_instructions[0x000D] = Op::RRC(Destination8::R(Registers8::L));
    cb_instructions[0x000E] = Op::RRC(Destination8::Mem(Registers16::HL));
    cb_instructions[0x000F] = Op::RRC(Destination8::R(Registers8::A));

    cb_instructions[0x0010] = Op::RL(Destination8::R(Registers8::B));
    cb_instructions[0x0011] = Op::RL(Destination8::R(Registers8::C));
    cb_instructions[0x0012] = Op::RL(Destination8::R(Registers8::D));
    cb_instructions[0x0013] = Op::RL(Destination8::R(Registers8::E));
    cb_instructions[0x0014] = Op::RL(Destination8::R(Registers8::H));
    cb_instructions[0x0015] = Op::RL(Destination8::R(Registers8::L));
    cb_instructions[0x0016] = Op::RL(Destination8::Mem(Registers16::HL));
    cb_instructions[0x0017] = Op::RL(Destination8::R(Registers8::A));
    cb_instructions[0x0018] = Op::RR(Destination8::R(Registers8::B));
    cb_instructions[0x0019] = Op::RR(Destination8::R(Registers8::C));
    cb_instructions[0x001A] = Op::RR(Destination8::R(Registers8::D));
    cb_instructions[0x001B] = Op::RR(Destination8::R(Registers8::E));
    cb_instructions[0x001C] = Op::RR(Destination8::R(Registers8::H));
    cb_instructions[0x001D] = Op::RR(Destination8::R(Registers8::L));
    cb_instructions[0x001E] = Op::RR(Destination8::Mem(Registers16::HL));
    cb_instructions[0x001F] = Op::RR(Destination8::R(Registers8::A));

    cb_instructions[0x0020] = Op::SLA(Destination8::R(Registers8::B));
    cb_instructions[0x0021] = Op::SLA(Destination8::R(Registers8::C));
    cb_instructions[0x0022] = Op::SLA(Destination8::R(Registers8::D));
    cb_instructions[0x0023] = Op::SLA(Destination8::R(Registers8::E));
    cb_instructions[0x0024] = Op::SLA(Destination8::R(Registers8::H));
    cb_instructions[0x0025] = Op::SLA(Destination8::R(Registers8::L));
    cb_instructions[0x0026] = Op::SLA(Destination8::Mem(Registers16::HL));
    cb_instructions[0x0027] = Op::SLA(Destination8::R(Registers8::A));
    cb_instructions[0x0028] = Op::SRA(Destination8::R(Registers8::B));
    cb_instructions[0x0029] = Op::SRA(Destination8::R(Registers8::C));
    cb_instructions[0x002A] = Op::SRA(Destination8::R(Registers8::D));
    cb_instructions[0x002B] = Op::SRA(Destination8::R(Registers8::E));
    cb_instructions[0x002C] = Op::SRA(Destination8::R(Registers8::H));
    cb_instructions[0x002D] = Op::SRA(Destination8::R(Registers8::L));
    cb_instructions[0x002E] = Op::SRA(Destination8::Mem(Registers16::HL));
    cb_instructions[0x002F] = Op::SRA(Destination8::R(Registers8::A));

    cb_instructions[0x0030] = Op::SWAP(Destination8::R(Registers8::B));
    cb_instructions[0x0031] = Op::SWAP(Destination8::R(Registers8::C));
    cb_instructions[0x0032] = Op::SWAP(Destination8::R(Registers8::D));
    cb_instructions[0x0033] = Op::SWAP(Destination8::R(Registers8::E));
    cb_instructions[0x0034] = Op::SWAP(Destination8::R(Registers8::H));
    cb_instructions[0x0035] = Op::SWAP(Destination8::R(Registers8::L));
    cb_instructions[0x0036] = Op::SWAP(Destination8::Mem(Registers16::HL));
    cb_instructions[0x0037] = Op::SWAP(Destination8::R(Registers8::A));
    cb_instructions[0x0038] = Op::SRL(Destination8::R(Registers8::B));
    cb_instructions[0x0039] = Op::SRL(Destination8::R(Registers8::C));
    cb_instructions[0x003A] = Op::SRL(Destination8::R(Registers8::D));
    cb_instructions[0x003B] = Op::SRL(Destination8::R(Registers8::E));
    cb_instructions[0x003C] = Op::SRL(Destination8::R(Registers8::H));
    cb_instructions[0x003D] = Op::SRL(Destination8::R(Registers8::L));
    cb_instructions[0x003E] = Op::SRL(Destination8::Mem(Registers16::HL));
    cb_instructions[0x003F] = Op::SRL(Destination8::R(Registers8::A));

    cb_instructions[0x0040] = Op::BIT(0, Destination8::R(Registers8::B));
    cb_instructions[0x0041] = Op::BIT(0, Destination8::R(Registers8::C));
    cb_instructions[0x0042] = Op::BIT(0, Destination8::R(Registers8::D));
    cb_instructions[0x0043] = Op::BIT(0, Destination8::R(Registers8::E));
    cb_instructions[0x0044] = Op::BIT(0, Destination8::R(Registers8::H));
    cb_instructions[0x0045] = Op::BIT(0, Destination8::R(Registers8::L));
    cb_instructions[0x0046] = Op::BIT(0, Destination8::Mem(Registers16::HL));
    cb_instructions[0x0047] = Op::BIT(0, Destination8::R(Registers8::A));
    cb_instructions[0x0048] = Op::BIT(1, Destination8::R(Registers8::B));
    cb_instructions[0x0049] = Op::BIT(1, Destination8::R(Registers8::C));
    cb_instructions[0x004A] = Op::BIT(1, Destination8::R(Registers8::D));
    cb_instructions[0x004B] = Op::BIT(1, Destination8::R(Registers8::E));
    cb_instructions[0x004C] = Op::BIT(1, Destination8::R(Registers8::H));
    cb_instructions[0x004D] = Op::BIT(1, Destination8::R(Registers8::L));
    cb_instructions[0x004E] = Op::BIT(1, Destination8::Mem(Registers16::HL));
    cb_instructions[0x004F] = Op::BIT(1, Destination8::R(Registers8::A));

    cb_instructions[0x0050] = Op::BIT(2, Destination8::R(Registers8::B));
    cb_instructions[0x0051] = Op::BIT(2, Destination8::R(Registers8::C));
    cb_instructions[0x0052] = Op::BIT(2, Destination8::R(Registers8::D));
    cb_instructions[0x0053] = Op::BIT(2, Destination8::R(Registers8::E));
    cb_instructions[0x0054] = Op::BIT(2, Destination8::R(Registers8::H));
    cb_instructions[0x0055] = Op::BIT(2, Destination8::R(Registers8::L));
    cb_instructions[0x0056] = Op::BIT(2, Destination8::Mem(Registers16::HL));
    cb_instructions[0x0057] = Op::BIT(2, Destination8::R(Registers8::A));
    cb_instructions[0x0058] = Op::BIT(3, Destination8::R(Registers8::B));
    cb_instructions[0x0059] = Op::BIT(3, Destination8::R(Registers8::C));
    cb_instructions[0x005A] = Op::BIT(3, Destination8::R(Registers8::D));
    cb_instructions[0x005B] = Op::BIT(3, Destination8::R(Registers8::E));
    cb_instructions[0x005C] = Op::BIT(3, Destination8::R(Registers8::H));
    cb_instructions[0x005D] = Op::BIT(3, Destination8::R(Registers8::L));
    cb_instructions[0x005E] = Op::BIT(3, Destination8::Mem(Registers16::HL));
    cb_instructions[0x005F] = Op::BIT(3, Destination8::R(Registers8::A));

    cb_instructions[0x0060] = Op::BIT(4, Destination8::R(Registers8::B));
    cb_instructions[0x0061] = Op::BIT(4, Destination8::R(Registers8::C));
    cb_instructions[0x0062] = Op::BIT(4, Destination8::R(Registers8::D));
    cb_instructions[0x0063] = Op::BIT(4, Destination8::R(Registers8::E));
    cb_instructions[0x0064] = Op::BIT(4, Destination8::R(Registers8::H));
    cb_instructions[0x0065] = Op::BIT(4, Destination8::R(Registers8::L));
    cb_instructions[0x0066] = Op::BIT(4, Destination8::Mem(Registers16::HL));
    cb_instructions[0x0067] = Op::BIT(4, Destination8::R(Registers8::A));
    cb_instructions[0x0068] = Op::BIT(5, Destination8::R(Registers8::B));
    cb_instructions[0x0069] = Op::BIT(5, Destination8::R(Registers8::C));
    cb_instructions[0x006A] = Op::BIT(5, Destination8::R(Registers8::D));
    cb_instructions[0x006B] = Op::BIT(5, Destination8::R(Registers8::E));
    cb_instructions[0x006C] = Op::BIT(5, Destination8::R(Registers8::H));
    cb_instructions[0x006D] = Op::BIT(5, Destination8::R(Registers8::L));
    cb_instructions[0x006E] = Op::BIT(5, Destination8::Mem(Registers16::HL));
    cb_instructions[0x006F] = Op::BIT(5, Destination8::R(Registers8::A));

    cb_instructions[0x0070] = Op::BIT(6, Destination8::R(Registers8::B));
    cb_instructions[0x0071] = Op::BIT(6, Destination8::R(Registers8::C));
    cb_instructions[0x0072] = Op::BIT(6, Destination8::R(Registers8::D));
    cb_instructions[0x0073] = Op::BIT(6, Destination8::R(Registers8::E));
    cb_instructions[0x0074] = Op::BIT(6, Destination8::R(Registers8::H));
    cb_instructions[0x0075] = Op::BIT(6, Destination8::R(Registers8::L));
    cb_instructions[0x0076] = Op::BIT(6, Destination8::Mem(Registers16::HL));
    cb_instructions[0x0077] = Op::BIT(6, Destination8::R(Registers8::A));
    cb_instructions[0x0078] = Op::BIT(7, Destination8::R(Registers8::B));
    cb_instructions[0x0079] = Op::BIT(7, Destination8::R(Registers8::C));
    cb_instructions[0x007A] = Op::BIT(7, Destination8::R(Registers8::D));
    cb_instructions[0x007B] = Op::BIT(7, Destination8::R(Registers8::E));
    cb_instructions[0x007C] = Op::BIT(7, Destination8::R(Registers8::H));
    cb_instructions[0x007D] = Op::BIT(7, Destination8::R(Registers8::L));
    cb_instructions[0x007E] = Op::BIT(7, Destination8::Mem(Registers16::HL));
    cb_instructions[0x007F] = Op::BIT(7, Destination8::R(Registers8::A));

    cb_instructions[0x0080] = Op::RES(0, Destination8::R(Registers8::B));
    cb_instructions[0x0081] = Op::RES(0, Destination8::R(Registers8::C));
    cb_instructions[0x0082] = Op::RES(0, Destination8::R(Registers8::D));
    cb_instructions[0x0083] = Op::RES(0, Destination8::R(Registers8::E));
    cb_instructions[0x0084] = Op::RES(0, Destination8::R(Registers8::H));
    cb_instructions[0x0085] = Op::RES(0, Destination8::R(Registers8::L));
    cb_instructions[0x0086] = Op::RES(0, Destination8::Mem(Registers16::HL));
    cb_instructions[0x0087] = Op::RES(0, Destination8::R(Registers8::A));
    cb_instructions[0x0088] = Op::RES(1, Destination8::R(Registers8::B));
    cb_instructions[0x0089] = Op::RES(1, Destination8::R(Registers8::C));
    cb_instructions[0x008A] = Op::RES(1, Destination8::R(Registers8::D));
    cb_instructions[0x008B] = Op::RES(1, Destination8::R(Registers8::E));
    cb_instructions[0x008C] = Op::RES(1, Destination8::R(Registers8::H));
    cb_instructions[0x008D] = Op::RES(1, Destination8::R(Registers8::L));
    cb_instructions[0x008E] = Op::RES(1, Destination8::Mem(Registers16::HL));
    cb_instructions[0x008F] = Op::RES(1, Destination8::R(Registers8::A));

    cb_instructions[0x0090] = Op::RES(2, Destination8::R(Registers8::B));
    cb_instructions[0x0091] = Op::RES(2, Destination8::R(Registers8::C));
    cb_instructions[0x0092] = Op::RES(2, Destination8::R(Registers8::D));
    cb_instructions[0x0093] = Op::RES(2, Destination8::R(Registers8::E));
    cb_instructions[0x0094] = Op::RES(2, Destination8::R(Registers8::H));
    cb_instructions[0x0095] = Op::RES(2, Destination8::R(Registers8::L));
    cb_instructions[0x0096] = Op::RES(2, Destination8::Mem(Registers16::HL));
    cb_instructions[0x0097] = Op::RES(2, Destination8::R(Registers8::A));
    cb_instructions[0x0098] = Op::RES(3, Destination8::R(Registers8::B));
    cb_instructions[0x0099] = Op::RES(3, Destination8::R(Registers8::C));
    cb_instructions[0x009A] = Op::RES(3, Destination8::R(Registers8::D));
    cb_instructions[0x009B] = Op::RES(3, Destination8::R(Registers8::E));
    cb_instructions[0x009C] = Op::RES(3, Destination8::R(Registers8::H));
    cb_instructions[0x009D] = Op::RES(3, Destination8::R(Registers8::L));
    cb_instructions[0x009E] = Op::RES(3, Destination8::Mem(Registers16::HL));
    cb_instructions[0x009F] = Op::RES(3, Destination8::R(Registers8::A));

    cb_instructions[0x00A0] = Op::RES(4, Destination8::R(Registers8::B));
    cb_instructions[0x00A1] = Op::RES(4, Destination8::R(Registers8::C));
    cb_instructions[0x00A2] = Op::RES(4, Destination8::R(Registers8::D));
    cb_instructions[0x00A3] = Op::RES(4, Destination8::R(Registers8::E));
    cb_instructions[0x00A4] = Op::RES(4, Destination8::R(Registers8::H));
    cb_instructions[0x00A5] = Op::RES(4, Destination8::R(Registers8::L));
    cb_instructions[0x00A6] = Op::RES(4, Destination8::Mem(Registers16::HL));
    cb_instructions[0x00A7] = Op::RES(4, Destination8::R(Registers8::A));
    cb_instructions[0x00A8] = Op::RES(5, Destination8::R(Registers8::B));
    cb_instructions[0x00A9] = Op::RES(5, Destination8::R(Registers8::C));
    cb_instructions[0x00AA] = Op::RES(5, Destination8::R(Registers8::D));
    cb_instructions[0x00AB] = Op::RES(5, Destination8::R(Registers8::E));
    cb_instructions[0x00AC] = Op::RES(5, Destination8::R(Registers8::H));
    cb_instructions[0x00AD] = Op::RES(5, Destination8::R(Registers8::L));
    cb_instructions[0x00AE] = Op::RES(5, Destination8::Mem(Registers16::HL));
    cb_instructions[0x00AF] = Op::RES(5, Destination8::R(Registers8::A));

    cb_instructions[0x00B0] = Op::RES(6, Destination8::R(Registers8::B));
    cb_instructions[0x00B1] = Op::RES(6, Destination8::R(Registers8::C));
    cb_instructions[0x00B2] = Op::RES(6, Destination8::R(Registers8::D));
    cb_instructions[0x00B3] = Op::RES(6, Destination8::R(Registers8::E));
    cb_instructions[0x00B4] = Op::RES(6, Destination8::R(Registers8::H));
    cb_instructions[0x00B5] = Op::RES(6, Destination8::R(Registers8::L));
    cb_instructions[0x00B6] = Op::RES(6, Destination8::Mem(Registers16::HL));
    cb_instructions[0x00B7] = Op::RES(6, Destination8::R(Registers8::A));
    cb_instructions[0x00B8] = Op::RES(7, Destination8::R(Registers8::B));
    cb_instructions[0x00B9] = Op::RES(7, Destination8::R(Registers8::C));
    cb_instructions[0x00BA] = Op::RES(7, Destination8::R(Registers8::D));
    cb_instructions[0x00BB] = Op::RES(7, Destination8::R(Registers8::E));
    cb_instructions[0x00BC] = Op::RES(7, Destination8::R(Registers8::H));
    cb_instructions[0x00BD] = Op::RES(7, Destination8::R(Registers8::L));
    cb_instructions[0x00BE] = Op::RES(7, Destination8::Mem(Registers16::HL));
    cb_instructions[0x00BF] = Op::RES(7, Destination8::R(Registers8::A));

    cb_instructions[0x00C0] = Op::SET(0, Destination8::R(Registers8::B));
    cb_instructions[0x00C1] = Op::SET(0, Destination8::R(Registers8::C));
    cb_instructions[0x00C2] = Op::SET(0, Destination8::R(Registers8::D));
    cb_instructions[0x00C3] = Op::SET(0, Destination8::R(Registers8::E));
    cb_instructions[0x00C4] = Op::SET(0, Destination8::R(Registers8::H));
    cb_instructions[0x00C5] = Op::SET(0, Destination8::R(Registers8::L));
    cb_instructions[0x00C6] = Op::SET(0, Destination8::Mem(Registers16::HL));
    cb_instructions[0x00C7] = Op::SET(0, Destination8::R(Registers8::A));
    cb_instructions[0x00C8] = Op::SET(1, Destination8::R(Registers8::B));
    cb_instructions[0x00C9] = Op::SET(1, Destination8::R(Registers8::C));
    cb_instructions[0x00CA] = Op::SET(1, Destination8::R(Registers8::D));
    cb_instructions[0x00CB] = Op::SET(1, Destination8::R(Registers8::E));
    cb_instructions[0x00CC] = Op::SET(1, Destination8::R(Registers8::H));
    cb_instructions[0x00CD] = Op::SET(1, Destination8::R(Registers8::L));
    cb_instructions[0x00CE] = Op::SET(1, Destination8::Mem(Registers16::HL));
    cb_instructions[0x00CF] = Op::SET(1, Destination8::R(Registers8::A));

    cb_instructions[0x00D0] = Op::SET(2, Destination8::R(Registers8::B));
    cb_instructions[0x00D1] = Op::SET(2, Destination8::R(Registers8::C));
    cb_instructions[0x00D2] = Op::SET(2, Destination8::R(Registers8::D));
    cb_instructions[0x00D3] = Op::SET(2, Destination8::R(Registers8::E));
    cb_instructions[0x00D4] = Op::SET(2, Destination8::R(Registers8::H));
    cb_instructions[0x00D5] = Op::SET(2, Destination8::R(Registers8::L));
    cb_instructions[0x00D6] = Op::SET(2, Destination8::Mem(Registers16::HL));
    cb_instructions[0x00D7] = Op::SET(2, Destination8::R(Registers8::A));
    cb_instructions[0x00D8] = Op::SET(3, Destination8::R(Registers8::B));
    cb_instructions[0x00D9] = Op::SET(3, Destination8::R(Registers8::C));
    cb_instructions[0x00DA] = Op::SET(3, Destination8::R(Registers8::D));
    cb_instructions[0x00DB] = Op::SET(3, Destination8::R(Registers8::E));
    cb_instructions[0x00DC] = Op::SET(3, Destination8::R(Registers8::H));
    cb_instructions[0x00DD] = Op::SET(3, Destination8::R(Registers8::L));
    cb_instructions[0x00DE] = Op::SET(3, Destination8::Mem(Registers16::HL));
    cb_instructions[0x00DF] = Op::SET(3, Destination8::R(Registers8::A));

    cb_instructions[0x00E0] = Op::SET(4, Destination8::R(Registers8::B));
    cb_instructions[0x00E1] = Op::SET(4, Destination8::R(Registers8::C));
    cb_instructions[0x00E2] = Op::SET(4, Destination8::R(Registers8::D));
    cb_instructions[0x00E3] = Op::SET(4, Destination8::R(Registers8::E));
    cb_instructions[0x00E4] = Op::SET(4, Destination8::R(Registers8::H));
    cb_instructions[0x00E5] = Op::SET(4, Destination8::R(Registers8::L));
    cb_instructions[0x00E6] = Op::SET(4, Destination8::Mem(Registers16::HL));
    cb_instructions[0x00E7] = Op::SET(4, Destination8::R(Registers8::A));
    cb_instructions[0x00E8] = Op::SET(5, Destination8::R(Registers8::B));
    cb_instructions[0x00E9] = Op::SET(5, Destination8::R(Registers8::C));
    cb_instructions[0x00EA] = Op::SET(5, Destination8::R(Registers8::D));
    cb_instructions[0x00EB] = Op::SET(5, Destination8::R(Registers8::E));
    cb_instructions[0x00EC] = Op::SET(5, Destination8::R(Registers8::H));
    cb_instructions[0x00ED] = Op::SET(5, Destination8::R(Registers8::L));
    cb_instructions[0x00EE] = Op::SET(5, Destination8::Mem(Registers16::HL));
    cb_instructions[0x00EF] = Op::SET(5, Destination8::R(Registers8::A));

    cb_instructions[0x00F0] = Op::SET(6, Destination8::R(Registers8::B));
    cb_instructions[0x00F1] = Op::SET(6, Destination8::R(Registers8::C));
    cb_instructions[0x00F2] = Op::SET(6, Destination8::R(Registers8::D));
    cb_instructions[0x00F3] = Op::SET(6, Destination8::R(Registers8::E));
    cb_instructions[0x00F4] = Op::SET(6, Destination8::R(Registers8::H));
    cb_instructions[0x00F5] = Op::SET(6, Destination8::R(Registers8::L));
    cb_instructions[0x00F6] = Op::SET(6, Destination8::Mem(Registers16::HL));
    cb_instructions[0x00F7] = Op::SET(6, Destination8::R(Registers8::A));
    cb_instructions[0x00F8] = Op::SET(7, Destination8::R(Registers8::B));
    cb_instructions[0x00F9] = Op::SET(7, Destination8::R(Registers8::C));
    cb_instructions[0x00FA] = Op::SET(7, Destination8::R(Registers8::D));
    cb_instructions[0x00FB] = Op::SET(7, Destination8::R(Registers8::E));
    cb_instructions[0x00FC] = Op::SET(7, Destination8::R(Registers8::H));
    cb_instructions[0x00FD] = Op::SET(7, Destination8::R(Registers8::L));
    cb_instructions[0x00FE] = Op::SET(7, Destination8::Mem(Registers16::HL));
    cb_instructions[0x00FF] = Op::SET(7, Destination8::R(Registers8::A));

    Instructions {
        instructions: instructions,
        cb_instructions: cb_instructions,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cpu;
    use registers;
    use device::boot_rom;
    use device::cartridge;
    use config;

    #[test]
    fn test_reading_gbm() {
        let instructions = new();
        let mut registers = registers::new();
        let mut mmu = mmu::new(boot_rom::zero(), cartridge::zero());
        let mut cpu = cpu::new(config::zero());

        assert_eq!(4, cpu.tick(&instructions, &mut registers, &mut mmu));
        assert_eq!(1, registers.get16(&Registers16::PC));
        assert_eq!(0xFFFE, registers.get16(&Registers16::SP));
    }

    #[test]
    fn test_di() {
        let instructions = new();
        let mut context = cpu::new_context();
        let mut registers = registers::new();
        let mut mmu = mmu::new(boot_rom::zero(), cartridge::zero());
        let mut cpu = cpu::new(config::zero());

        assert_eq!(cpu.execute(&mut context, &Op::DI, &mut registers, &mut mmu), 4);
        assert_eq!(registers.get_interrupts_enabled(), false);
    }

    #[test]
    fn test_swap() {
        // This is 11110000 in binary
        let n = 0xF0;

        let mut registers = registers::new();

        let out = swap(&mut registers, n);

        // 0x0F is 00001111 in binary
        assert_eq!(out, 0x0F);
    }

    #[test]
    fn test_sla() {
        // This is 255 when we shift everything left we'll get FE aka 254
        let n = 0xFF;

        let mut registers = registers::new();

        let out = sla(&mut registers, n);

        assert_eq!(out, 0xFE);
        assert_eq!(registers.get_flag(Flag::C), true);
    }

    #[test]
    fn test_rl() {
        // This is 255 when we shift everything left we'll get FE aka 254
        let n = 0xFF;

        let mut registers = registers::new();
        registers.set_flag(Flag::C, true);

        let out = rl(&mut registers, n);

        assert_eq!(out, 0xFF);
        assert_eq!(registers.get_flag(Flag::C), true);
    }

    #[test]
    fn test_rl_c_unset() {
        // This is 255 when we shift everything left we'll get FE aka 254
        let n = 0xFF;

        let mut registers = registers::new();

        let out = rl(&mut registers, n);

        assert_eq!(out, 0xFE);
        assert_eq!(registers.get_flag(Flag::C), true);
    }

    #[test]
    fn test_sra() {
        // This is 255 when we shift everything right we'll get 7F but retain first bit so should
        // stay FF
        let n = 0xFF;

        let mut registers = registers::new();

        let out = sra(&mut registers, n);

        assert_eq!(out, 0xFF);
        assert_eq!(registers.get_flag(Flag::C), true);
    }

    #[test]
    fn test_srl() {
        // This is 255 when we shift everything right we'll get 7F
        // stay FF
        let n = 0xFF;

        let mut registers = registers::new();
        registers.set_flag(Flag::C, true);

        let out = srl(&mut registers, n);

        assert_eq!(out, 0x7F);
        assert_eq!(registers.get_flag(Flag::C), true);
    }

    #[test]
    fn test_rr() {
        // This is 255 when we shift everything right we'll get 7F
        // stay FF
        let n = 0xFF;

        let mut registers = registers::new();
        registers.set_flag(Flag::C, true);

        let out = rr(&mut registers, n);

        assert_eq!(out, 0xFF);
        assert_eq!(registers.get_flag(Flag::C), true);
    }

    #[test]
    fn test_cpl() {
        // This is all set when we take the compliment we should get 0
        let n = 0xFF;

        let mut registers = registers::new();

        let out = cpl(&mut registers, n);

        assert_eq!(out, 0x00);
    }

    #[test]
    fn test_cpl_better() {
        // This is all set when we take the compliment we should get 0
        let n = 0xFE;

        let mut registers = registers::new();

        let out = cpl(&mut registers, n);

        assert_eq!(out, 0x01);
    }

    #[test]
    fn test_bit() {
        // 0xFE all bits are one except 0th bit
        let n = 0xFE;

        let mut registers = registers::new();
        assert_eq!(bit(&mut registers, n, 7), true);
        assert_eq!(bit(&mut registers, n, 6), true);
        assert_eq!(bit(&mut registers, n, 0), false);
    }

    #[test]
    fn test_set() {
        // 0xFE all bits are one except 0th bit
        let n = 0xFE;

        assert_eq!(set(n, 0), 0xFF);
    }

    #[test]
    fn test_res() {
        // 0xFF all bits are set
        let n = 0xFF;

        assert_eq!(res(n, 0), 0xFE);
    }
}
