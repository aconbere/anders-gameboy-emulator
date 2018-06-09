use ::memory;
use ::registers;

pub struct Instruction {
    operation: fn(&registers::Registers, &memory::RAM, Vec<u8>),
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
    pub fn call(&self, registers:&registers::Registers, memory:&memory::RAM, args:Vec<u8>) {
        let op = &self.operation;
        op(registers, memory, args)
    }
}

pub struct Instructions {
    instructions: Vec<Instruction>
}

impl Instructions {
    pub fn get(&self, opcode:u8) -> &Instruction {
        let o = opcode as usize;
        &self.instructions[o]
    }
}

mod operations {
    use ::memory;
    use ::registers;
    pub fn nop(_:&registers::Registers, _:&memory::RAM, _:Vec<u8>) {}
    pub fn ld_sp(_:&registers::Registers, _:&memory::RAM, args:Vec<u8>) {
        println!("LD SP: {:?}", args)
    }
    // fn ld_bc_d16(_:&registers::Registers, _:&memory::RAM) {
    // 
    // }

}

pub fn new() -> Instructions {
    let nop = Instruction {
        operation: operations::nop,
        args: 1,
        label: String::from("NOP"),
    };

    let ld_sp = Instruction {
        operation: operations::ld_sp,
        args: 3,
        label: String::from("LD SP"),
    };

    let mut instructions = vec![nop;256];

    instructions[0x0031] = ld_sp;

    Instructions {
        instructions: instructions
    }
}
