use ::registers;
use ::instructions;
use ::memory;
use ::program;

pub struct CPU {
    registers: registers::Registers,
    instructions: instructions::Instructions,
    memory: memory::RAM,
    program: program::Program
}

impl CPU {
    pub fn run(&mut self) {
        let pc = self.registers.get_pc();
        println!("pc: {}", pc);
        let opcode = self.memory.get(pc);
        println!("opcode: {}", opcode);
        let instruction = self.instructions.get(opcode);
        println!("instruction: {}", instruction.label);

        let mut args = vec![0; instruction.args as usize];
        for _ in 1..instruction.args {
            let next = pc+1;
            self.registers.set_pc(next);
            args.push(self.memory.get(next))
        }

        instruction.call(&self.registers, &self.memory, args);
    }
}

pub fn new(
    registers:registers::Registers,
    instructions:instructions::Instructions,
    memory:memory::RAM,
    program:program::Program
) -> CPU {
    CPU {
        registers:registers,
        instructions:instructions,
        memory:memory,
        program:program
    }
}

