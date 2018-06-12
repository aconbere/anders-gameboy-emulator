use ::memory;
use ::registers;
use ::bytes;

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
}

#[derive(Debug, Clone, Copy)]
pub enum JrArgs {
    Z,
    NZ,
    C,
    NC,
    N
}

#[derive(Debug, Clone, Copy)]
pub enum JpArgs {
    Z,
    NZ,
    C,
    NC,
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
pub enum Op {
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

    // CB extras
    BIT(u8, Destination8),

}

fn load_to_memory(
    registers:&mut registers::Registers,
    memory:&mut memory::RAM,
    rm: &registers::Registers16,
    rv: &registers::Registers8,
) {
    let m = registers.get16(rm);
    let v = registers.get8(rv);
    memory.set(m, v);
}

impl Op {
    pub fn args(&self) -> u8 {
        match self {
            Op::NOP => 0,
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
            Op::BIT(_, _) => 0,
            Op::JR(_) => 1,
            Op::JP(_) => 2,
            Op::LoadFF00(_, LoadFF00Targets::N) => 1,
            Op::LoadFF00(LoadFF00Targets::N, _) => 1,
            Op::LoadFF00(_, _) => 0,
        }
    }

    pub fn call(&self, registers:&mut registers::Registers, memory:&mut memory::RAM, args:Vec<u8>) {
        match self {
            Op::NOP => {},
            Op::Load8(Destination8::R(r1), Source8::R(r2)) => {
                let v = registers.get8(r2);
                registers.set8(r1, v);
            },
            Op::Load8(Destination8::R(r1), Source8::N) => {
                registers.set8(r1, args[0]);
            },
            Op::Load8(Destination8::R(r1), Source8::Mem(r2)) => {
                let v = memory.get(registers.get16(r2));
                registers.set8(r1, v);
            },
            Op::Load8(Destination8::Mem(r1),Source8::R(r2)) => {
                load_to_memory(registers, memory, r1, r2);
            },
            Op::Load8(_, _) => panic!("invalid args to load8"),
            Op::Load16(Destination16::R(r1), Source16::R(r2)) => {
                let v = registers.get16(r2);
                registers.set16(r1, v);
            },
            Op::Load16(Destination16::R(r1), Source16::N) => {
                registers.set16(r1, bytes::combine_little(args[0], args[1]));
            },
            Op::LoadFF00(LoadFF00Targets::C, LoadFF00Targets::A)=> {
                let c = registers.get8(&registers::Registers8::C) as u16;
                let a = registers.get8(&registers::Registers8::A);
                memory.set(c + 0xFF00, a);
            },
            Op::LoadFF00(LoadFF00Targets::A, LoadFF00Targets::C)=> {
                let c = registers.get8(&registers::Registers8::C) as u16;
                let v = memory.get(c + 0xFF00);
                registers.set8(&registers::Registers8::A, v);
            },
            Op::LoadFF00(LoadFF00Targets::A, LoadFF00Targets::N)=> {
                let ma = args[0] as u16;
                let v = memory.get(ma + 0xFF00);
                registers.set8(&registers::Registers8::A, v);
            },
            Op::LoadFF00(LoadFF00Targets::N, LoadFF00Targets::A)=> {
                let a = registers.get8(&registers::Registers8::A);
                let ma = args[0] as u16;
                memory.set(ma + 0xFF00, a);
            },
            Op::LoadFF00(_, _)=> panic!("invalid loadFF00 inputs"),

            Op::Inc8(Destination8::R(r)) => {
                let v = registers.get8(r);
                registers.set8(r, v + 1);
            },
            Op::Inc8(Destination8::Mem(r)) => {
                let a = registers.get16(r);
                let v = memory.get(a);
                memory.set(a, v + 1);
            },
            Op::Inc16(Destination16::R(r)) => {
                let v = registers.get16(r);
                registers.set16(r, v + 1);
            },

            Op::Dec8(Destination8::R(r)) => {
                let v = registers.get8(r);
                registers.set8(r, v - 1);
            },
            Op::Dec8(Destination8::Mem(r)) => {
                let a = registers.get16(r);
                let v = memory.get(a);
                memory.set(a, v -1 );
            },
            Op::Dec16(Destination16::R(r)) => {
                let v = registers.get16(r);
                registers.set16(r, v - 1);
            }

            Op::LoadAndDec => {
                load_to_memory(registers, memory, &registers::Registers16::HL, &registers::Registers8::A);
                registers.dec_hl();
            },
            Op::LoadAndInc => {
                load_to_memory(registers, memory, &registers::Registers16::HL, &registers::Registers8::A);
                registers.inc_hl();
            },

            Op::XOR(Destination8::R(r)) => {
                let v = registers.get8(r);
                let a = registers.get8(&registers::Registers8::A);
                registers.set8(&registers::Registers8::A, a ^ v)
            },
            Op::XOR(Destination8::Mem(_)) => {},

            Op::BIT(location, Destination8::R(r)) => {
                let v = registers.get8(r);
                if bytes::check_bit(v, *location) {
                    memory.clear_flag(memory::Flag::Z);
                } else {
                    memory.set_flag(memory::Flag::Z);
                }
            },
            Op::BIT(_, Destination8::Mem(_)) => {},

            Op::JR(JrArgs::NZ) => {
                if !memory.get_flag(memory::Flag::Z) {
                    let v = args[0] as i8;
                    let pc = registers.get16(&registers::Registers16::PC);
                    registers.set16(&registers::Registers16::PC, bytes::add_unsigned_signed(pc, v))
                }
            },
            Op::JR(_) => { },

            Op::JP(_) => { },
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
    let mut instructions = vec![Op::NOP;256];

    instructions[0x000C] = Op::Inc8(Destination8::R(registers::Registers8::C));
    instructions[0x000E] = Op::Load8(Destination8::R(registers::Registers8::C), Source8::N);
    instructions[0x0011] = Op::Load16(Destination16::R(registers::Registers16::DE), Source16::N);
    instructions[0x003E] = Op::Load8(Destination8::R(registers::Registers8::A), Source8::N);
    instructions[0x001A] = Op::Load8(Destination8::R(registers::Registers8::A), Source8::Mem(registers::Registers16::DE));
    instructions[0x0031] = Op::Load16(Destination16::R(registers::Registers16::SP), Source16::N);
    instructions[0x0032] = Op::LoadAndDec;
    instructions[0x0020] = Op::JR(JrArgs::NZ);
    instructions[0x0021] = Op::Load16(Destination16::R(registers::Registers16::HL), Source16::N);
    instructions[0x00AF] = Op::XOR(Destination8::R(registers::Registers8::A));
    instructions[0x00E2] = Op::LoadFF00(LoadFF00Targets::C, LoadFF00Targets::A);
    instructions[0x00E0] = Op::LoadFF00(LoadFF00Targets::N, LoadFF00Targets::A);
    instructions[0x0077] = Op::Load8(Destination8::Mem(registers::Registers16::HL), Source8::R(registers::Registers8::A));

    let mut cb_instructions = vec![Op::NOP;256];
    cb_instructions[0x007C] = Op::BIT(7, Destination8::R(registers::Registers8::H));

    Instructions {
        instructions: instructions,
        cb_instructions: cb_instructions
    }
}
