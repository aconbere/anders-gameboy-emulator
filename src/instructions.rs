mod memory;
mod registers;

struct Instruction {
    label: String,
    operation: Box<Fn(registers::Registers, memory::Memory)>
}

pub fn init() -> Vec<Instruction> {
    let mut instructions = Vec::with_capacity(256);
    instructions[0x0000] = Instruction {
        label: String::from("NOP"),
        operation: Box::new(|r:registers::Registers, m:memory::Memory|{})
    };
    return instructions
}
