use ::memory;
use ::registers;

pub struct Instruction {
    operation: fn(&mut registers::Registers, &mut memory::RAM, Vec<u8>),
    pub args: u8,
    pub label: String
}

impl Clone for Instruction {
    fn clone(&self) -> Self { 
        Instruction {
            operation:self.operation,
            args:self.args,
            label:self.label.clone()
        }
    }
}

impl Instruction {
    pub fn call(&self, registers:&mut registers::Registers, memory:&mut memory::RAM, args:Vec<u8>) {
        let op = &self.operation;
        op(registers, memory, args)
    }
}

pub struct Instructions {
    instructions: Vec<Instruction>,
    cb_instructions: Vec<Instruction>
}

impl Instructions {
    pub fn get(&self, opcode:u8) -> &Instruction {
        let o = opcode as usize;
        &self.instructions[o]
    }

    pub fn get_cb(&self, opcode:u8) -> &Instruction {
        let o = opcode as usize;
        &self.cb_instructions[o]
    }
}

mod operations {
    use ::memory;
    use ::registers;
    use ::bytes;

    pub fn nop(_:&mut registers::Registers, _:&mut memory::RAM, _:Vec<u8>) {}

    pub fn ld_a(registers:&mut registers::Registers, _:&mut memory::RAM, args:Vec<u8>) {
        ld_u8_into(registers, registers::Registers8::A, args[0])
    }

    pub fn ld_c(registers:&mut registers::Registers, _:&mut memory::RAM, args:Vec<u8>) {
        ld_u8_into(registers, registers::Registers8::C, args[0])
    }

    pub fn ld_sp(registers:&mut registers::Registers, _:&mut memory::RAM, args:Vec<u8>) {
        ld_u16_into(registers, registers::Registers16::SP, args)
    }

    pub fn ld_hl(registers:&mut registers::Registers, _:&mut memory::RAM, args:Vec<u8>) {
        ld_u16_into(registers, registers::Registers16::HL, args)
    }


    pub fn ldd_hl(registers:&mut registers::Registers, memory:&mut memory::RAM, _:Vec<u8>) {
        let a = registers.get8(registers::Registers8::A);
        let hl = registers.get16(registers::Registers16::HL);
        println!("LDD HL - A: {:X} | HL: {:X} {:b}", a, hl, hl);
        memory.set(hl, a);
        registers.dec_hl();
    }

    pub fn xor_a(registers:&mut registers::Registers, _:&mut memory::RAM, args:Vec<u8>) {
        println!("XOR A: {:?}", args);
        let a = registers.get8(registers::Registers8::A);
        registers.set8(registers::Registers8::A, a ^ a)
    }

    pub fn bit_7_h(registers:&mut registers::Registers, memory:&mut memory::RAM, args:Vec<u8>) {
        let h = registers.get8(registers::Registers8::H);
        println!("BIT 7,h: {:X} {:b}", h, h);
        if bytes::check_bit(h, 7) {
            memory.clear_flag(memory::Flag::Z);
        } else {
            println!("BIT 7,h; is zero");
            memory.set_flag(memory::Flag::Z);
        }
    }

    pub fn jr_nz(registers:&mut registers::Registers, memory:&mut memory::RAM, args:Vec<u8>) {
        println!("JR NZ,e: {:?}", args);
        if !memory.get_flag(memory::Flag::Z) {
            let v = args[0] as i8;
            let pc = registers.get16(registers::Registers16::PC);
            // println!("JR: {}, {}", v, pc);
            println!("JR: {}, {}, {}", v, pc, bytes::add_unsigned_signed(pc, v));
            registers.set16(registers::Registers16::PC, bytes::add_unsigned_signed(pc, v))
        }
    }

    fn ld_u8_into(registers:&mut registers::Registers, r:registers::Registers8, v:u8) {
        registers.set8(r, v)
    }

    fn ld_u16_into(registers:&mut registers::Registers, r:registers::Registers16, args:Vec<u8>) {
        let v = bytes::combine_little(args[0], args[1]);
        println!("ld_u16_into: loading {:X}", v);
        registers.set16(r, v)
    }
}

pub fn new() -> Instructions {
    let nop = Instruction {
        operation: operations::nop,
        args: 0,
        label: String::from("NOP"),
    };

    let ld_c = Instruction {
        operation: operations::ld_c,
        args: 1,
        label: String::from("LD C"),
    };

    let ld_a = Instruction {
        operation: operations::ld_a,
        args: 1,
        label: String::from("LD A"),
    };

    let ld_sp = Instruction {
        operation: operations::ld_sp,
        args: 2,
        label: String::from("LD SP"),
    };

    let xor_a = Instruction {
        operation: operations::xor_a,
        args: 0,
        label: String::from("XOR A"),
    };

    let ld_hl = Instruction {
        operation: operations::ld_hl,
        args: 2,
        label: String::from("LD HL"),
    };

    let ldd_hl = Instruction {
        operation: operations::ldd_hl,
        args: 0,
        label: String::from("LDD HL"),
    };

    let jr_nz = Instruction {
        operation: operations::jr_nz,
        args: 1,
        label: String::from("JR NZ"),
    };

    let mut instructions = vec![nop;256];

    instructions[0x000E] = ld_c;
    instructions[0x0031] = ld_sp;
    instructions[0x0032] = ldd_hl;
    instructions[0x003E] = ld_a;
    instructions[0x00AF] = xor_a;
    instructions[0x0020] = jr_nz;
    instructions[0x0021] = ld_hl;

    let cb_nop = Instruction {
        operation: operations::nop,
        args: 0,
        label: String::from("CB_NOP"),
    };

    let bit_7_h = Instruction {
        operation: operations::bit_7_h,
        args: 0,
        label: String::from("BIT 7 H"),
    };


    let mut cb_instructions = vec![cb_nop;256];
    cb_instructions[0x007C] = bit_7_h;

    Instructions {
        instructions: instructions,
        cb_instructions: cb_instructions
    }
}
