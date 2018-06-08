use ::memory;
use ::registers;

pub struct Instruction {
    operation: Box<Fn(&registers::Registers, &memory::RAM, Vec<u8>)>,
    pub args: u8,
    pub label: String
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
    pub fn nop(_:&registers::Registers, _:&memory::RAM, args:Vec<u8>) {}
    // fn ld_bc_d16(_:&registers::Registers, _:&memory::RAM) {
    // 
    // }

}

pub fn new() -> Instructions {
    let nop = Instruction {
        operation: Box::new(operations::nop),
        args: 1,
        label: String::from("NOP"),
    };

    Instructions {
        instructions: vec![
            nop,
        ]
    }
}
