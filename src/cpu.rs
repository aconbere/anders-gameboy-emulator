use ::registers;
use ::instructions;
use ::memory;
use ::program;

pub struct CPU <'a> {
    registers: &'a mut registers::Registers,
    instructions: &'a instructions::Instructions,
    memory: &'a memory::RAM,
    program: &'a program::Program
}

impl <'a> CPU <'a> {
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
        println!("calling instruction: {} with args: {:?}", instruction.label, args);

        instruction.call(&self.registers, &self.memory, args);
    }
}

pub fn new<'a>(
    registers:&'a mut registers::Registers,
    instructions:&'a instructions::Instructions,
    memory:&'a memory::RAM,
    program:&'a program::Program
) -> CPU <'a> {
    CPU {
        registers:registers,
        instructions:instructions,
        memory:memory,
        program:program
    }
}

