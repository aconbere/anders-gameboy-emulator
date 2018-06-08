use ::memory;
use ::registers;

pub struct Instruction {
    label: String,
    operation: Box<Fn(registers::Registers, memory::RAM)>
}

pub fn init() -> Vec<Instruction> {
    let mut instructions = Vec::with_capacity(256);
    instructions[0x0000] = Instruction {
        label: String::from("NOP"),
        operation: Box::new(|_:registers::Registers, _:memory::RAM|{})
    };
    return instructions
}
