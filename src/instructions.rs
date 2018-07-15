use bytes;
use mmu;

use registers::Registers8;
use registers::Registers16;
use registers::Registers;
use registers::Flag;

/*
 * LD r,r
 * LD r,n
 */
#[derive(Debug, Clone, Copy)]
pub enum Source8 {
    R(Registers8),
    Mem(Registers16),
    N,
}

#[derive(Debug, Clone, Copy)]
pub enum Source16 {
    R(Registers16),
    R8(Registers16),
    N,
}

#[derive(Debug, Clone, Copy)]
pub enum Destination8 {
    R(Registers8),
    Mem(Registers16),
}

#[derive(Debug, Clone, Copy)]
pub enum SubArgs {
    R(Registers8),
    Mem(Registers16),
    N
}

#[derive(Debug, Clone, Copy)]
pub enum Destination16 {
    R(Registers16),
    Mem(Registers16),
}

#[derive(Debug, Clone, Copy)]
pub enum Load16Args {
    R(Registers16),
    Mem(Registers16),
    N,
}

#[derive(Debug, Clone, Copy)]
pub enum Load8Args {
    R(Registers8),
    Mem(Registers16),
    N,
}

#[derive(Debug, Clone, Copy)]
pub enum Add16Args {
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
pub enum CallArgs {
    CheckFlag(CheckFlag),
    N,
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
pub enum RetArgs {
    CheckFlag(CheckFlag),
    Null,
}

#[derive(Debug, Clone, Copy)]
pub enum Op {
    NotImplemented,
    NOP,
    DI, // Disable interrupts
    EI, // Enable interrupts
    Load8(Load8Args, Load8Args),
    Load16(Load16Args, Load16Args),

    Inc8(Destination8),
    Inc16(Destination16),
    Dec8(Destination8),
    Dec16(Destination16),
    OR(Destination8),
    XOR(Destination8),
    Sub(SubArgs),
    Add(Destination8),
    Add16(Add16Args, Add16Args),

    RLCA,

    LoadAndInc,
    LoadAndIncR,
    LoadAndDec,
    LoadAndDecR,
    JR(JrArgs),
    JP(JpArgs),
    LoadFF00(LoadFF00Targets, LoadFF00Targets),
    Call(CallArgs),
    Pop(Registers16),
    Push(Registers16),
    Ret(RetArgs),
    Compare(Source8),
    Halt,
    PrefixCB,

    // CB extras
    BIT(u8, Destination8),
    RL(Destination8),
}

fn jump(registers: &mut Registers, v: u16) {
    registers.set16(&Registers16::PC, v);
}

fn jump_relative(registers: &mut Registers, v: i8) {
    let pc = registers.get16(&Registers16::PC);
    let out = bytes::add_unsigned_signed(pc, v);
    registers.set16(&Registers16::PC, out);
    // println!("jump: PC=({}{})={}", pc, v, out);
}

fn compare(registers: &mut Registers, v: u8) {
    let a = registers.get8(&Registers8::A);

    // println!("\tCompare: A {:X} to V {:X}", a, v);
    let n = a.wrapping_sub(v);

    if n == 0 {
        // println!("Compare: ZERO");
    }

    registers.set_flag(Flag::Z, n == 0);
    registers.set_flag(Flag::C, a < v);
    registers.set_flag(Flag::N, true);
    registers.set_flag(Flag::H, (0x0F & v) > (0x0F & a));
}

fn sub(registers: &mut Registers, v: u8) {
    let a = registers.get8(&Registers8::A);
    let n = a.wrapping_sub(v);

    registers.set_flag(Flag::Z, n == 0);
    registers.set_flag(Flag::C, false);
    registers.set_flag(Flag::N, true);
    registers.set_flag(Flag::H, v & 0x0F == 0x0F);

    registers.set8(&Registers8::A, n);
}

fn add(registers: &mut Registers, v: u8) {
    let a = registers.get8(&Registers8::A);
    let n = a.wrapping_add(v);

    registers.set_flag(Flag::Z, n == 0);
    registers.set_flag(Flag::C, false);
    registers.set_flag(Flag::N, false);
    registers.set_flag(Flag::H, v & 0x0F == 0x0F);

    registers.set8(&Registers8::A, n);
}

fn add16(registers: &mut Registers, destination: &Registers16, v: u16) {
    let a = registers.get16(destination);
    let n = a.wrapping_add(v);

    registers.set_flag(Flag::Z, n == 0);
    registers.set_flag(Flag::C, false);
    registers.set_flag(Flag::N, false);
    registers.set_flag(Flag::H, v & 0x0F == 0x0F);

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

fn push_stack(
    registers: &mut Registers,
    mmu: &mut mmu::MMU,
    r: &Registers16,
) {
    let sp = registers.get16(&Registers16::SP);
    let v = registers.get16(r);
    let (vh, vl) = bytes::split_u16(v);
    mmu.set(sp - 1, vh);
    mmu.set(sp - 2, vl);
    registers.set16(&Registers16::SP, sp - 2);
}

fn pop_stack(registers: &mut Registers, mmu: &mut mmu::MMU, r: &Registers16) {
    let sp = registers.get16(&Registers16::SP);
    let v = mmu.get16(sp);
    registers.set16(r, v);
    registers.set16(&Registers16::SP, sp + 2);
}

impl Op {
    pub fn args(&self) -> u8 {
        match self {
            Op::NotImplemented => 0,
            Op::NOP => 0,
            Op::DI => 0,
            Op::EI => 0,
            Op::Halt => 0,
            Op::PrefixCB => 0,

            Op::Load8(_, Load8Args::N) => 1,
            Op::Load8(Load8Args::N, _) => 2,
            Op::Load8(_, _) => 0,

            Op::Load16(_, Load16Args::N) => 2,
            Op::Load16(Load16Args::N, _) => 2,
            Op::Load16(_, _) => 0,

            Op::Inc8(_) => 0,
            Op::Inc16(_) => 0,
            Op::Dec8(_) => 0,
            Op::Dec16(_) => 0,
            Op::LoadAndInc => 0,
            Op::LoadAndDec => 0,
            Op::LoadAndIncR => 0,
            Op::LoadAndDecR => 0,
            Op::OR(_) => 0,
            Op::XOR(_) => 0,
            Op::Sub(SubArgs::N) => 1,
            Op::Sub(_) => 0,
            Op::Add(_) => 0,
            Op::Add16(_, _) => 0,
            Op::JR(_) => 1,
            Op::JP(_) => 2,
            Op::LoadFF00(_, LoadFF00Targets::N) => 1,
            Op::LoadFF00(LoadFF00Targets::N, _) => 1,
            Op::LoadFF00(_, _) => 0,
            Op::Call(_) => 2,
            Op::Push(_) => 0,
            Op::Pop(_) => 0,
            Op::Ret(_) => 0,
            Op::Compare(Source8::N) => 1,
            Op::Compare(_) => 0,
            Op::RLCA => 0,

            // cb instructions
            Op::BIT(_, _) => 0,
            Op::RL(_) => 0,
        }
    }

    pub fn call(
        &self,
        registers: &mut Registers,
        mmu: &mut mmu::MMU,
        args: &Vec<u8>,
    ) -> u8 {
        match self {
            Op::NotImplemented => panic!("NotImplemented Instruction"),
            Op::NOP => 4,
            Op::DI => {
                registers.set_interrupts_enabled(false);
                4
            }
            Op::EI => {
                registers.set_interrupts_enabled(true);
                4
            }
            Op::Halt => 4,
            Op::PrefixCB => 4,
            Op::Load8(Load8Args::R(r1), Load8Args::R(r2)) => {
                let v = registers.get8(r2);
                registers.set8(r1, v);
                // println!("Load8({:?}, {:?}) {:?}={:X}", r1, r2, r1, registers.get8(r1));
                4
            }
            Op::Load8(Load8Args::R(r1), Load8Args::N) => {
                registers.set8(r1, args[0]);
                4
            }
            Op::Load8(Load8Args::R(r1), Load8Args::Mem(r2)) => {
                let v = mmu.get(registers.get16(r2));
                registers.set8(r1, v);
                8
            }
            Op::Load8(Load8Args::Mem(r1), Load8Args::R(r2)) => {
                load_to_memory(registers, mmu, r1, r2);
                8
            }
            Op::Load8(Load8Args::N, Load8Args::R(r)) => {
                let a = bytes::combine_little(args[0], args[1]);
                let v = registers.get8(r);
                mmu.set(a, v);
                8
            }
            Op::Load8(_, _) => panic!("Not Implemented"),

            Op::Load16(Load16Args::R(r1), Load16Args::R(r2)) => {
                let v = registers.get16(r2);
                registers.set16(r1, v);
                12
            }
            Op::Load16(Load16Args::R(r1), Load16Args::N) => {
                let v = bytes::combine_little(args[0], args[1]);
                registers.set16(r1, v);
                12
            }
            Op::Load16(Load16Args::N, Load16Args::R(r)) => {
                let v = registers.get16(r);
                let (v_high, v_low) = bytes::split_u16(v);
                let a = bytes::combine_little(args[0], args[1]);
                mmu.set(a, v_high);
                mmu.set(a+1, v_low);
                20
            }
            Op::Load16(Load16Args::Mem(_), _) => {
                panic!("Not Implemented");
                //20
            }
            Op::Load16(_, _) => panic!("Not Implemented"),

            Op::LoadFF00(LoadFF00Targets::C, LoadFF00Targets::A) => {
                let c = registers.get8(&Registers8::C) as u16;
                let a = registers.get8(&Registers8::A);
                // println!("Load FF00+{:X} with {:X}", c, a);
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
                // println!("LoadFF00: Loading A with {:X}", v);
                registers.set8(&Registers8::A, v);
                8
            }
            Op::LoadFF00(LoadFF00Targets::N, LoadFF00Targets::A) => {
                let a = registers.get8(&Registers8::A);
                let ma = args[0] as u16;
                // println!("Load FF00+{:X} with {:X}", ma, a);
                mmu.set(ma + 0xFF00, a);
                8
            }
            Op::LoadFF00(_, _) => panic!("invalid loadFF00 inputs"),

            Op::LoadAndDec => {
                load_to_memory(
                    registers,
                    mmu,
                    &Registers16::HL,
                    &Registers8::A,
                );
                registers.dec_hl();
                8
            }
            Op::LoadAndInc => {
                load_to_memory(
                    registers,
                    mmu,
                    &Registers16::HL,
                    &Registers8::A,
                );
                registers.inc_hl();
                8
            }
            Op::LoadAndIncR => {
                load_from_memory(
                    registers,
                    mmu,
                    &Registers16::HL,
                    &Registers8::A,
                );
                registers.inc_hl();
                8
            }
            Op::LoadAndDecR => {
                load_from_memory(
                    registers,
                    mmu,
                    &Registers16::HL,
                    &Registers8::A,
                );
                registers.dec_hl();
                8
            }

            Op::JR(JrArgs::CheckFlag(f)) => {
                let check = match f {
                    CheckFlag::Z => registers.get_flag(Flag::Z),
                    CheckFlag::NZ => !registers.get_flag(Flag::Z),
                    CheckFlag::C => registers.get_flag(Flag::C),
                    CheckFlag::NC => !registers.get_flag(Flag::C),
                };

                if check {
                    // println!("JR: Check {:?} {} jumping", f, check);
                    jump_relative(registers, args[0] as i8);
                    16
                } else {
                    // println!("JR: Check {:?} {} continuing", f, check);
                    12
                }
            }
            Op::JR(JrArgs::N) => {
                jump_relative(registers, args[0] as i8);
                12
            }

            Op::JP(JpArgs::CheckFlag(f)) => {
                let check = match f {
                    CheckFlag::Z => registers.get_flag(Flag::Z),
                    CheckFlag::NZ => !registers.get_flag(Flag::Z),
                    CheckFlag::C => registers.get_flag(Flag::C),
                    CheckFlag::NC => !registers.get_flag(Flag::C),
                };

                if check {
                    // println!("JP: flag set!");
                    12
                } else {
                    // println!("JP: flag unset!");
                    jump(registers, bytes::combine_little(args[0], args[1]));
                    16
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

            Op::Call(CallArgs::N) => {
                push_stack(registers, mmu, &Registers16::PC);
                registers.set16(
                    &Registers16::PC,
                    bytes::combine_little(args[0], args[1]),
                );
                // println!(
                //     "Call - SP:{:X} PC{:X}",
                //     registers.get16(&Registers16::SP),
                //     registers.get16(&Registers16::PC)
                // );
                24
            }
            Op::Call(_) => panic!("Not Implemented"),
            Op::Push(r) => {
                push_stack(registers, mmu, r);
                16
            }
            Op::Pop(r) => {
                pop_stack(registers, mmu, r);
                12
            }
            Op::Ret(RetArgs::Null) => {
                let sp = registers.get16(&Registers16::SP);
                let v = mmu.get16(sp);
                registers.set16(&Registers16::PC, v);
                registers.set16(&Registers16::SP, sp + 2);
                16
            }
            Op::Ret(RetArgs::CheckFlag(_)) => panic!("Not Implemented"),

            // ALU Codes
            Op::Inc8(Destination8::R(r)) => {
                let v = registers.get8(r);
                let n = v.wrapping_add(1);

                registers.set_flag(Flag::N, false);
                registers.set_flag(Flag::Z, n == 0);
                registers.set_flag(Flag::H, v & 0x0F == 0x0F);

                registers.set8(r, n);
                4
            }
            Op::Inc8(Destination8::Mem(r)) => {
                let a = registers.get16(r);
                let v = mmu.get(a);
                let n = v.wrapping_add(1);

                registers.set_flag(Flag::N, false);
                registers.set_flag(Flag::Z, n == 0);
                registers.set_flag(Flag::H, v & 0x0F == 0x0F);

                mmu.set(a, n);
                12
            }
            Op::Inc16(Destination16::R(r)) => {
                let v = registers.get16(r);
                let n = v.wrapping_add(1);

                registers.set_flag(Flag::N, false);
                registers.set_flag(Flag::Z, n == 0);
                registers.set_flag(Flag::H, v & 0x00FF == 0x00FF);

                registers.set16(r, n);
                // println!("Inc16: Incrementing {:?} to {:X} - {:X}", r, n, registers.get16(r));
                8
            }
            Op::Inc16(Destination16::Mem(_)) => panic!("Not Implemented"),

            Op::Dec8(Destination8::R(r)) => {
                let v = registers.get8(r);
                let n = v.wrapping_sub(1);

                registers.set_flag(Flag::N, true);
                registers.set_flag(Flag::Z, n == 0);
                registers.set_flag(Flag::H, v & 0x0F == 0x0F);

                registers.set8(r, n);
                // println!("Dec8: Decrementing {:?} to {:X}", r, n);
                4
            }
            Op::Dec8(Destination8::Mem(r)) => {
                let a = registers.get16(r);
                let v = mmu.get(a);
                let n = v.wrapping_sub(1);

                registers.set_flag(Flag::N, true);
                registers.set_flag(Flag::Z, n == 0);
                registers.set_flag(Flag::H, v & 0x0F == 0x0F);

                mmu.set(a, n);
                12
            }
            Op::Dec16(Destination16::R(r)) => {
                let v = registers.get16(r);
                let n = v.wrapping_sub(1);

                registers.set_flag(Flag::N, false);
                registers.set_flag(Flag::Z, n == 0);
                registers.set_flag(Flag::H, v & 0x00FF == 0x00FF);

                registers.set16(r, v + 1);
                8
            }
            Op::Dec16(_) => panic!("Not Implemented"),

            Op::Compare(Source8::N) => {
                compare(registers, args[0]);
                8
            }
            Op::Compare(Source8::R(_)) => panic!("Not Implemented"),
            Op::Compare(Source8::Mem(r)) => {
                let m = registers.get16(r);
                let v = mmu.get(m);
                compare(registers, v);
                8
            }
            Op::OR(Destination8::R(r)) => {
                let v = registers.get8(r);
                let a = registers.get8(&Registers8::A);
                let n = a | v;

                registers.set_flag(Flag::N, n == 0);
                registers.set_flag(Flag::Z, false);
                registers.set_flag(Flag::H, false);
                registers.set_flag(Flag::C, false);

                registers.set8(&Registers8::A, n);
                4
            }
            Op::OR(Destination8::Mem(_)) => panic!("Not Implemented"),
            Op::XOR(Destination8::R(r)) => {
                let v = registers.get8(r);
                let a = registers.get8(&Registers8::A);
                let n = a ^ v;

                registers.set_flag(Flag::N, n == 0);
                registers.set_flag(Flag::Z, false);
                registers.set_flag(Flag::H, false);
                registers.set_flag(Flag::C, false);

                registers.set8(&Registers8::A, n);
                4
            }
            Op::XOR(Destination8::Mem(_)) => panic!("Not Implemented"),
            Op::Sub(SubArgs::R(r)) => {
                let v = registers.get8(r);
                sub(registers, v);
                4
            }
            Op::Sub(SubArgs::Mem(r)) => {
                let m = registers.get16(r);
                let v = mmu.get(m);
                sub(registers, v);
                8
            }
            Op::Sub(SubArgs::N) => {
                sub(registers, args[0]);
                8
            }

            Op::Add(Destination8::R(r)) => {
                let v = registers.get8(r);
                add(registers, v);
                4
            }
            Op::Add(Destination8::Mem(r)) => {
                let m = registers.get16(r);
                let v = mmu.get(m);
                add(registers, v);
                8
            }

            Op::Add16(Add16Args::R(r1), Add16Args::R(r2)) => {
                let v = registers.get16(r2);
                add16(registers, r1, v);
                8
            }
            Op::RLCA => {
                let r = &Registers8::A;
                let v = registers.get8(r);

                let out = v << 1;

                registers.set_flag(Flag::Z, false);
                registers.set_flag(Flag::C, bytes::check_bit(v, 7));
                registers.set_flag(Flag::N, false);
                registers.set_flag(Flag::H, false);

                registers.set8(r, out);
                8
            }

            // End ALU Codes



            // Cb instructions
            Op::BIT(location, Destination8::R(r)) => {
                let v = registers.get8(r);
                registers.set_flag(Flag::Z, bytes::check_bit(v, *location));
                registers.set_flag(Flag::C, false);
                registers.set_flag(Flag::N, false);
                registers.set_flag(Flag::H, false);
                8
            }
            Op::BIT(_, Destination8::Mem(_)) => panic!("Not Implemented"),

            Op::RL(Destination8::R(r)) => {
                let v = registers.get8(r);
                let c = registers.get_flag(Flag::C);

                let out = if c { (v << 1) | 0x0001 } else { v << 1 };

                registers.set_flag(Flag::Z, false);
                registers.set_flag(Flag::C, bytes::check_bit(v, 7));
                registers.set_flag(Flag::N, false);
                registers.set_flag(Flag::H, false);

                registers.set8(r, out);
                8
            }
            Op::RL(Destination8::Mem(_)) => panic!("Not Implemented"),
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
    instructions[0x0001] = Op::Load16(Load16Args::R(Registers16::BC), Load16Args::N);
    instructions[0x0002] = Op::Load8(Load8Args::Mem(Registers16::BC), Load8Args::R(Registers8::A));
    instructions[0x0003] = Op::Inc16(Destination16::R(Registers16::BC));
    instructions[0x0004] = Op::Inc8(Destination8::R(Registers8::B));
    instructions[0x0005] = Op::Dec8(Destination8::R(Registers8::B));
    instructions[0x0006] = Op::Load8(Load8Args::R(Registers8::B), Load8Args::N);
    instructions[0x0007] = Op::RLCA;
    instructions[0x0008] = Op::Load16(Load16Args::N, Load16Args::R(Registers16::SP));
    instructions[0x0009] = Op::Add16(Add16Args::R(Registers16::HL), Add16Args::R(Registers16::BC));
    instructions[0x0010] = Op::NOP; // this should be Op::STOP not sure what it does yet though
    instructions[0x000A] = Op::Load8(Load8Args::R(Registers8::C), Load8Args::Mem(Registers16::BC));
    instructions[0x000B] = Op::Dec16(Destination16::R(Registers16::BC));
    instructions[0x000C] = Op::Inc8(Destination8::R(Registers8::C));
    instructions[0x000D] = Op::Dec8(Destination8::R(Registers8::C));
    instructions[0x000E] = Op::Load8(Load8Args::R(Registers8::C), Load8Args::N);
    instructions[0x0011] = Op::Load16(Load16Args::R(Registers16::DE), Load16Args::N);
    instructions[0x0013] = Op::Inc16(Destination16::R(Registers16::DE));
    instructions[0x0015] = Op::Dec8(Destination8::R(Registers8::D));
    instructions[0x0016] = Op::Load8(Load8Args::R(Registers8::D), Load8Args::N);
    instructions[0x0017] = Op::RL(Destination8::R(Registers8::A));
    instructions[0x0018] = Op::JR(JrArgs::N);
    instructions[0x001A] = Op::Load8(
        Load8Args::R(Registers8::A),
        Load8Args::Mem(Registers16::DE),
    );
    instructions[0x001D] = Op::Dec8(Destination8::R(Registers8::E));
    instructions[0x001E] = Op::Load8(Load8Args::R(Registers8::E), Load8Args::N);
    instructions[0x0020] = Op::JR(JrArgs::CheckFlag(CheckFlag::NZ));
    instructions[0x0021] = Op::Load16(Load16Args::R(Registers16::HL), Load16Args::N);
    instructions[0x0022] = Op::LoadAndInc;
    instructions[0x0023] = Op::Inc16(Destination16::R(Registers16::HL));
    instructions[0x0024] = Op::Inc8(Destination8::R(Registers8::H));
    instructions[0x0028] = Op::JR(JrArgs::CheckFlag(CheckFlag::Z));
    instructions[0x002A] = Op::LoadAndIncR;
    instructions[0x002E] = Op::Load8(Load8Args::R(Registers8::L), Load8Args::N);
    instructions[0x0031] = Op::Load16(Load16Args::R(Registers16::SP), Load16Args::N);
    instructions[0x0032] = Op::LoadAndDec;
    instructions[0x003D] = Op::Dec8(Destination8::R(Registers8::A));
    instructions[0x003E] = Op::Load8(Load8Args::R(Registers8::A), Load8Args::N);
    instructions[0x004F] = Op::Load8(
        Load8Args::R(Registers8::C),
        Load8Args::R(Registers8::A),
    );
    instructions[0x0057] = Op::Load8(
        Load8Args::R(Registers8::D),
        Load8Args::R(Registers8::A),
    );
    instructions[0x0067] = Op::Load8(
        Load8Args::R(Registers8::H),
        Load8Args::R(Registers8::A),
    );
    instructions[0x0076] = Op::Halt;
    instructions[0x0077] = Op::Load8(
        Load8Args::Mem(Registers16::HL),
        Load8Args::R(Registers8::A),
    );
    instructions[0x0078] = Op::Load8(
        Load8Args::R(Registers8::A),
        Load8Args::R(Registers8::B),
    );
    instructions[0x007B] = Op::Load8(
        Load8Args::R(Registers8::A),
        Load8Args::R(Registers8::E),
    );
    instructions[0x007C] = Op::Load8(
        Load8Args::R(Registers8::A),
        Load8Args::R(Registers8::H),
    );
    instructions[0x007D] = Op::Load8(
        Load8Args::R(Registers8::A),
        Load8Args::R(Registers8::L),
    );
    instructions[0x0086] = Op::Add(Destination8::Mem(Registers16::HL));
    instructions[0x0090] = Op::Sub(SubArgs::R(Registers8::B));
    instructions[0x00AF] = Op::XOR(Destination8::R(Registers8::A));
    instructions[0x00B1] = Op::OR(Destination8::R(Registers8::A));
    instructions[0x00BE] = Op::Compare(Source8::Mem(Registers16::HL));
    instructions[0x00C1] = Op::Pop(Registers16::BC);
    instructions[0x00C2] = Op::JP(JpArgs::CheckFlag(CheckFlag::NZ));
    instructions[0x00C3] = Op::JP(JpArgs::N);
    instructions[0x00C5] = Op::Push(Registers16::BC);
    instructions[0x00CB] = Op::PrefixCB;
    instructions[0x00CD] = Op::Call(CallArgs::N);
    instructions[0x00C9] = Op::Ret(RetArgs::Null);
    instructions[0x00D6] = Op::Sub(SubArgs::N);
    instructions[0x00E2] = Op::LoadFF00(LoadFF00Targets::C, LoadFF00Targets::A);
    instructions[0x00E0] = Op::LoadFF00(LoadFF00Targets::N, LoadFF00Targets::A);
    instructions[0x00E1] = Op::Pop(Registers16::HL);
    instructions[0x00E5] = Op::Push(Registers16::HL);
    instructions[0x00EA] = Op::Load8(Load8Args::N, Load8Args::R(Registers8::A));
    instructions[0x00F0] = Op::LoadFF00(LoadFF00Targets::A, LoadFF00Targets::N);
    instructions[0x00F1] = Op::Pop(Registers16::AF);
    instructions[0x00F3] = Op::DI;
    instructions[0x00F5] = Op::Push(Registers16::AF);
    instructions[0x00FA] = Op::Load8(Load8Args::R(Registers8::A), Load8Args::N);
    instructions[0x00FE] = Op::Compare(Source8::N);

    let mut cb_instructions = vec![Op::NotImplemented; 256];
    cb_instructions[0x007C] = Op::BIT(7, Destination8::R(Registers8::H));
    cb_instructions[0x0011] = Op::RL(Destination8::R(Registers8::C));

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

    #[test]
    fn test_reading_gbm() {
        let instructions = new();
        let mut registers = registers::new();
        let mut mmu = mmu::new();
        let mut cpu = cpu::new();

        assert_eq!(12, cpu.tick(&instructions, &mut registers, &mut mmu));
        assert_eq!(3, registers.get16(&Registers16::PC));
        assert_eq!(0xFFFE, registers.get16(&Registers16::SP));
    }

    #[test]
    fn test_di() {
        let instructions = new();
        let mut registers = registers::new();
        let mut mmu = mmu::new();
        let mut cpu = cpu::new();

        assert_eq!(cpu.execute(&Op::DI, &mut registers, &mut mmu), 4);
        assert_eq!(registers.get_interrupts_enabled(), false);
    }
}
