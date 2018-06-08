mod registers;
mod instructions;
mod memory;
mod program;

struct CPU {
    registers: registers::Registers,
    instructions: instructions::Instructions,
    memory: memory::Memory,
    program: program::Program
}

impl CPU {
    pub fn next() {


    }
}



pub fn init(
    registers:registers::Registers,
    instructions:instructions::Instructions,
    memory:memory::Memory,
    program:program::Program
) -> CPU {
    CPU {
        registers:registers,
        instructions:instructions,
        memory:memory,
        program:program
    }
}

