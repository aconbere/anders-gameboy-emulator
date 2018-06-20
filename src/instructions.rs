use ::mmu;
use ::registers;
use ::bytes;
use device;

/*
 * LD r,r
 * LD r,n
 */
#[derive(Debug, Clone, Copy)]
pub enum Source8 {
    R(registers::Registers8),
    Mem(registers::Registers16),
    N,
}

#[derive(Debug, Clone, Copy)]
pub enum Source16 {
    R(registers::Registers16),
    R8(registers::Registers16),
    N,
}

#[derive(Debug, Clone, Copy)]
pub enum Destination8 {
    R(registers::Registers8),
    Mem(registers::Registers16),
}

#[derive(Debug, Clone, Copy)]
pub enum Destination16 {
    R(registers::Registers16),
    Mem(registers::Registers16),
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
    N
}

#[derive(Debug, Clone, Copy)]
pub enum JrArgs {
    CheckFlag(CheckFlag),
    N
}

#[derive(Debug, Clone, Copy)]
pub enum JpArgs {
    CheckFlag(CheckFlag),
    N,
    HL
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
    Load8(Destination8, Source8),
    Load16(Destination16, Source16),

    Inc8(Destination8),
    Inc16(Destination16),

    Dec8(Destination8),
    Dec16(Destination16),
    LoadAndInc,
    //LoadAndIncR,
    LoadAndDec,
    //LoadAndDecR,
    XOR(Destination8),
    JR(JrArgs),
    JP(JpArgs),
    LoadFF00(LoadFF00Targets, LoadFF00Targets),
    Call(CallArgs),
    Pop(registers::Registers16),
    Push(registers::Registers16),
    Ret(RetArgs),
    Compare(Source8),
    Halt,
    PrefixCB,

    // CB extras
    BIT(u8, Destination8),
    RL(Destination8)
}

fn load_to_memory(
    registers:&mut registers::Registers,
    mmu:&mut mmu::MMU,
    rm: &registers::Registers16,
    rv: &registers::Registers8,
) {
    let m = registers.get16(rm);
    let v = registers.get8(rv);
    mmu.set(m, v);
}

fn push_stack(
    registers:&mut registers::Registers,
    mmu:&mut mmu::MMU,
    r: &registers::Registers16
) {
    let sp = registers.get16(&registers::Registers16::SP);
    let v = registers.get16(r);
    let (vh, vl) = bytes::split_u16(v);
    mmu.set(sp - 1, vh);
    mmu.set(sp - 2, vl);
    registers.set16(&registers::Registers16::SP, sp-2);
}

fn pop_stack(
    registers:&mut registers::Registers,
    mmu:&mut mmu::MMU,
    r: &registers::Registers16
) {
    let sp = registers.get16(&registers::Registers16::SP);
    let v = mmu.get16(sp);
    registers.set16(r, v);
    registers.set16(&registers::Registers16::SP, sp+2);
}

impl Op {
    pub fn args(&self) -> u8 {
        match self {
            Op::NotImplemented => 0,
            Op::NOP => 0,
            Op::Halt => 0,
            Op::PrefixCB => 0,
            Op::Load8(_, Source8::N) => 1,
            Op::Load8(_, _) => 0,
            Op::Load16(_, Source16::N) => 2,
            Op::Load16(_, _) => 0,
            Op::Inc8(_) => 0,
            Op::Inc16(_) => 0,
            Op::Dec8(_) => 0,
            Op::Dec16(_) => 0,
            Op::LoadAndInc => 0,
            Op::LoadAndDec => 0,
            Op::XOR(_) => 0,
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

            // cb instructions
            Op::BIT(_, _) => 0,
            Op::RL(_) => 0,
        }
    }

    pub fn call(&self, registers:&mut registers::Registers, mmu:&mut mmu::MMU, args:Vec<u8>) -> u8 {
        match self {
            Op::NotImplemented => panic!("NotImplemented Instruction"),
            Op::NOP => 4,
            Op::Halt => 4,
            Op::PrefixCB => 4,
            Op::Load8(Destination8::R(r1), Source8::R(r2)) => {
                let v = registers.get8(r2);
                registers.set8(r1, v);
                println!("Load8({:?}, {:?}) {:?}={:X}", r1, r2, r1, registers.get8(r1));
                4
            },
            Op::Load8(Destination8::R(r1), Source8::N) => {
                registers.set8(r1, args[0]);
                4
            },
            Op::Load8(Destination8::R(r1), Source8::Mem(r2)) => {
                let v = mmu.get(registers.get16(r2));
                registers.set8(r1, v);
                8
            },
            Op::Load8(Destination8::Mem(r1),Source8::R(r2)) => {
                load_to_memory(registers, mmu, r1, r2);
                8
            },
            Op::Load8(_,_) => panic!("Not Implemented"),

            Op::Load16(Destination16::R(r1), Source16::R(r2)) => {
                let v = registers.get16(r2);
                registers.set16(r1, v);
                12
            },
            Op::Load16(Destination16::R(r1), Source16::N) => {
                registers.set16(r1, bytes::combine_little(args[0], args[1]));
                12
            },
            Op::Load16(Destination16::Mem(_), _) => {
                panic!("Not Implemented");
                //20
            },
            Op::Load16(_, _) => panic!("Not Implemented"),

            Op::LoadFF00(LoadFF00Targets::C, LoadFF00Targets::A)=> {
                let c = registers.get8(&registers::Registers8::C) as u16;
                let a = registers.get8(&registers::Registers8::A);
                println!("Load FF00+{:X} with {:X}", c, a);
                mmu.set(c + 0xFF00, a);
                8
            },
            Op::LoadFF00(LoadFF00Targets::A, LoadFF00Targets::C)=> {
                let c = registers.get8(&registers::Registers8::C) as u16;
                let v = mmu.get(c + 0xFF00);
                registers.set8(&registers::Registers8::A, v);
                8
            },
            Op::LoadFF00(LoadFF00Targets::A, LoadFF00Targets::N)=> {
                let ma = args[0] as u16;
                let v = mmu.get(ma + 0xFF00);
                registers.set8(&registers::Registers8::A, v);
                8
            },
            Op::LoadFF00(LoadFF00Targets::N, LoadFF00Targets::A)=> {
                let a = registers.get8(&registers::Registers8::A);
                let ma = args[0] as u16;
                println!("Load FF00+{:X} with {:X}", ma, a);
                mmu.set(ma + 0xFF00, a);
                8
            },
            Op::LoadFF00(_, _)=> panic!("invalid loadFF00 inputs"),

            Op::LoadAndDec => {
                load_to_memory(registers, mmu, &registers::Registers16::HL, &registers::Registers8::A);
                registers.dec_hl();
                8
            },
            Op::LoadAndInc => {
                load_to_memory(registers, mmu, &registers::Registers16::HL, &registers::Registers8::A);
                registers.inc_hl();
                8
            },

            Op::JR(JrArgs::CheckFlag(CheckFlag::NZ)) => {
                if mmu.get_flag(device::flags::Flag::Z) {
                    println!("JR: flag set!");
                    12
                } else {
                    println!("JR: flag unset!");
                    let v = args[0] as i8;
                    let pc = registers.get16(&registers::Registers16::PC);
                    let out = bytes::add_unsigned_signed(pc, v);
                    println!("JR: PC=({}{})={}", pc, v, out); 
                    registers.set16(&registers::Registers16::PC, out);
                    16
                }
            },

            Op::JR(_) => panic!("Not Implemented"),
            Op::JP(_) => panic!("Not Implemented"),
            Op::Call(CallArgs::N) => {
                push_stack(registers, mmu, &registers::Registers16::PC);
                registers.set16(&registers::Registers16::PC, bytes::combine_little(args[0], args[1]));
                println!(
                    "Call - SP:{:X} PC{:X}",
                    registers.get16(&registers::Registers16::SP),
                    registers.get16(&registers::Registers16::PC)
                );
                24
            },
            Op::Call(_) => panic!("Not Implemented"),
            Op::Push(r) => {
                push_stack(registers, mmu, r);
                16
            },
            Op::Pop(r) => {
                pop_stack(registers, mmu, r);
                12
            },
            Op::Ret(RetArgs::Null) => {
                let sp = registers.get16(&registers::Registers16::SP);
                let v = mmu.get16(sp);
                registers.set16(&registers::Registers16::PC, v);
                registers.set16(&registers::Registers16::SP, sp+2);
                16
            },
            Op::Ret(RetArgs::CheckFlag(_)) => panic!("Not Implemented"),


            // ALU Codes
            Op::Inc8(Destination8::R(r)) => {
                let v = registers.get8(r);
                let n = v.wrapping_add(1);

                mmu.set_flag(device::flags::Flag::N, false);
                mmu.set_flag(device::flags::Flag::Z, n == 0);
                mmu.set_flag(device::flags::Flag::H, v & 0x0F == 0x0F);

                registers.set8(r, n);
                4
            },
            Op::Inc8(Destination8::Mem(r)) => {
                let a = registers.get16(r);
                let v = mmu.get(a);
                let n = v.wrapping_add(1);

                mmu.set_flag(device::flags::Flag::N, false);
                mmu.set_flag(device::flags::Flag::Z, n == 0);
                mmu.set_flag(device::flags::Flag::H, v & 0x0F == 0x0F);

                mmu.set(a, n);
                12
            },
            Op::Inc16(Destination16::R(r)) => {
                let v = registers.get16(r);
                let n = v.wrapping_add(1);

                mmu.set_flag(device::flags::Flag::N, false);
                mmu.set_flag(device::flags::Flag::Z, n == 0);
                mmu.set_flag(device::flags::Flag::H, v & 0x00FF == 0x00FF);

                println!("Inc16: Incrementing {:?} to {:X}", r, n);

                registers.set16(r, n);
                8
            },
            Op::Inc16(Destination16::Mem(_)) => panic!("Not Implemented"),

            Op::Dec8(Destination8::R(r)) => {
                let v = registers.get8(r);
                let n = v.wrapping_sub(1);

                mmu.set_flag(device::flags::Flag::N, true);
                mmu.set_flag(device::flags::Flag::Z, n == 0);
                mmu.set_flag(device::flags::Flag::H, v & 0x0F == 0x0F);

                registers.set8(r, n);
                4
            },
            Op::Dec8(Destination8::Mem(r)) => {
                let a = registers.get16(r);
                let v = mmu.get(a);
                let n = v.wrapping_sub(1);

                mmu.set_flag(device::flags::Flag::N, true);
                mmu.set_flag(device::flags::Flag::Z, n == 0);
                mmu.set_flag(device::flags::Flag::H, v & 0x0F == 0x0F);

                mmu.set(a, n);
                12
            },
            Op::Dec16(Destination16::R(r)) => {
                let v = registers.get16(r);
                let n = v.wrapping_sub(1);

                mmu.set_flag(device::flags::Flag::N, false);
                mmu.set_flag(device::flags::Flag::Z, n == 0);
                mmu.set_flag(device::flags::Flag::H, v & 0x00FF == 0x00FF);

                registers.set16(r, v + 1);
                8
            },
            Op::Dec16(_) => panic!("Not Implemented"),

            Op::Compare(Source8::N) => {
                let a = registers.get8(&registers::Registers8::A);
                let v = args[0];

                println!("\tCompare: A {:X} to V {:X}", a, v);

                mmu.set_flag(device::flags::Flag::N, true);
                mmu.set_flag(device::flags::Flag::Z, a == v);
                mmu.set_flag(device::flags::Flag::H, (0x0F & v) > (0x0F & a));
                mmu.set_flag(device::flags::Flag::C, v > a);
                8
            },
            Op::Compare(Source8::R(_)) => panic!("Not Implemented"),
            Op::Compare(Source8::Mem(_)) => panic!("Not Implemented"),
            Op::XOR(Destination8::R(r)) => {
                let v = registers.get8(r);
                let a = registers.get8(&registers::Registers8::A);
                let n = a ^ v;

                mmu.set_flag(device::flags::Flag::N, n == 0);
                mmu.set_flag(device::flags::Flag::Z, false);
                mmu.set_flag(device::flags::Flag::H, false);
                mmu.set_flag(device::flags::Flag::C, false);

                registers.set8(&registers::Registers8::A, n);
                4
            },
            Op::XOR(Destination8::Mem(_)) => panic!("Not Implemented"),


            // End ALU Codes



            // Cb instructions
            Op::BIT(location, Destination8::R(r)) => {
                let v = registers.get8(r);
                println!("Bit: {}, {:b}", v, v);
                if bytes::check_bit(v, *location) {
                    mmu.set_flag(device::flags::Flag::Z, false);
                } else {
                    mmu.set_flag(device::flags::Flag::Z, true);
                }
                8
            },
            Op::BIT(_, Destination8::Mem(_)) => panic!("Not Implemented"),
            Op::RL(Destination8::R(r)) => {
                let v = registers.get8(r);
                let c = mmu.get_flag(device::flags::Flag::C);

                println!("IN: {:b}", v);

                let out = if c {
                    (v << 1) | 0x0001
                } else {
                    v << 1
                };
                println!("OUT: {:b}", out);

                mmu.set_flag(device::flags::Flag::C, bytes::check_bit(v, 7));

                registers.set8(r, out);
                println!("RL{:?}, {:?}={:X}, flags={}", r, r, registers.get8(r), mmu.get_flag(device::flags::Flag::C));
                8
            }
            Op::RL(Destination8::Mem(_)) => panic!("Not Implemented"),
        }
    }
}

pub struct Instructions {
    instructions: Vec<Op>,
    cb_instructions: Vec<Op>
}

impl Instructions {
    pub fn get(&self, opcode:u8) -> &Op {
        let o = opcode as usize;
        &self.instructions[o]
    }

    pub fn get_cb(&self, opcode:u8) -> &Op {
        let o = opcode as usize;
        &self.cb_instructions[o]
    }
}

pub fn new() -> Instructions {
    let mut instructions = vec![Op::NotImplemented;256];

    instructions[0x0000] = Op::NOP;
    instructions[0x0005] = Op::Dec8(Destination8::R(registers::Registers8::B));
    instructions[0x0006] = Op::Load8(Destination8::R(registers::Registers8::B), Source8::N);
    instructions[0x000C] = Op::Inc8(Destination8::R(registers::Registers8::C));
    instructions[0x000E] = Op::Load8(Destination8::R(registers::Registers8::C), Source8::N);
    instructions[0x0011] = Op::Load16(Destination16::R(registers::Registers16::DE), Source16::N);
    instructions[0x0013] = Op::Inc16(Destination16::R(registers::Registers16::DE));
    instructions[0x0017] = Op::RL(Destination8::R(registers::Registers8::A));
    instructions[0x003E] = Op::Load8(Destination8::R(registers::Registers8::A), Source8::N);
    instructions[0x001A] = Op::Load8(Destination8::R(registers::Registers8::A), Source8::Mem(registers::Registers16::DE));
    instructions[0x0021] = Op::Load16(Destination16::R(registers::Registers16::HL), Source16::N);
    instructions[0x0022] = Op::LoadAndInc;
    instructions[0x0023] = Op::Inc8(Destination8::Mem(registers::Registers16::HL));
    instructions[0x004F] = Op::Load8(Destination8::R(registers::Registers8::C), Source8::R(registers::Registers8::A));
    instructions[0x0031] = Op::Load16(Destination16::R(registers::Registers16::SP), Source16::N);
    instructions[0x0032] = Op::LoadAndDec;
    instructions[0x0020] = Op::JR(JrArgs::CheckFlag(CheckFlag::NZ));
    instructions[0x00AF] = Op::XOR(Destination8::R(registers::Registers8::A));
    instructions[0x00C5] = Op::Push(registers::Registers16::BC);
    instructions[0x00C1] = Op::Pop(registers::Registers16::BC);
    instructions[0x00CB] = Op::PrefixCB;
    instructions[0x00CD] = Op::Call(CallArgs::N);
    instructions[0x00C9] = Op::Ret(RetArgs::Null);
    instructions[0x00E2] = Op::LoadFF00(LoadFF00Targets::C, LoadFF00Targets::A);
    instructions[0x00E0] = Op::LoadFF00(LoadFF00Targets::N, LoadFF00Targets::A);
    instructions[0x00FE] = Op::Compare(Source8::N);
    instructions[0x0076] = Op::Halt;
    instructions[0x0077] = Op::Load8(Destination8::Mem(registers::Registers16::HL), Source8::R(registers::Registers8::A));
    instructions[0x007B] = Op::Load8(Destination8::R(registers::Registers8::A), Source8::R(registers::Registers8::E));

    let mut cb_instructions = vec![Op::NotImplemented;256];
    cb_instructions[0x007C] = Op::BIT(7, Destination8::R(registers::Registers8::H));
    cb_instructions[0x0011] = Op::RL(Destination8::R(registers::Registers8::C));

    Instructions {
        instructions: instructions,
        cb_instructions: cb_instructions
    }
}
