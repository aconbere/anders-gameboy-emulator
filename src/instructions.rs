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
    pub fn ld_sp(registers:&mut registers::Registers, _:&mut memory::RAM, args:Vec<u8>) {
        println!("LD SP: {:?}", args);
        registers.set_sp(bytes::combine_little(args[0], args[1]))
    }
    pub fn ld_hl(registers:&mut registers::Registers, _:&mut memory::RAM, args:Vec<u8>) {
        println!("LD HL: {:?}", args);
        registers.set_h(args[1]);
        registers.set_l(args[0]);
    }
    pub fn ldd_hl(registers:&mut registers::Registers, memory:&mut memory::RAM, _:Vec<u8>) {
        let a = registers.get_a();
        let hl = registers.get_hl();
        println!("LDD HL - A: {} | HL: {}", a, hl);
        memory.set(hl, a);
        registers.dec_hl();
    }
    pub fn xor_a(registers:&mut registers::Registers, _:&mut memory::RAM, args:Vec<u8>) {
        println!("XOR A: {:?}", args);
        let a = registers.get_a();
        registers.set_a(a ^ a)
    }
    pub fn bit_7_h(_:&mut registers::Registers, _:&mut memory::RAM, args:Vec<u8>) {
        println!("BIT 7,h: {:?}", args)
    }
}

pub fn new() -> Instructions {
    let nop = Instruction {
        operation: operations::nop,
        args: 0,
        label: String::from("NOP"),
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

    let mut instructions = vec![nop;256];

    instructions[0x0031] = ld_sp;
    instructions[0x0032] = ldd_hl;
    instructions[0x00AF] = xor_a;
    instructions[0x0021] = ld_hl;

    let cb_nop = Instruction {
        operation: operations::nop,
        args: 0,
        label: String::from("CB_NOP"),
    };

    let bit_7_h = Instruction {
        operation: operations::bit_7_h,
        args: 1,
        label: String::from("LDD HL"),
    };


    let mut cb_instructions = vec![cb_nop;256];
    cb_instructions[0x007C] = bit_7_h;

    Instructions {
        instructions: instructions,
        cb_instructions: cb_instructions
    }
}
