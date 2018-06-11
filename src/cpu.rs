use ::registers;
use ::instructions;
use ::memory;
use ::program;

pub struct CPU <'a> {
    registers: &'a mut registers::Registers,
    instructions: &'a instructions::Instructions,
    memory: &'a mut memory::RAM,
    program: &'a program::Program
}

impl <'a> CPU <'a> {
    pub fn run(&mut self) {
        for i in 0..2000 {
            self.next()
        }
    }
    pub fn next(&mut self) {
        println!("TICK");

        let pc = self.registers.get16(registers::Registers16::PC);
        println!("\tpc: {}", pc);

        let opcode = self.memory.get(pc);
        self.registers.inc_pc();
        println!("\topcode: {:X}", opcode);

        let instruction = if opcode == 0x00CB {
            println!("found cb opcode");
            let pc = self.registers.get16(registers::Registers16::PC);
            let opcode = self.memory.get(pc);
            self.registers.inc_pc();
            self.instructions.get_cb(opcode)
        } else {
            self.instructions.get(opcode)
        };
        println!("\tinstruction: {}", instruction.label);

        let mut args = Vec::new();
        for _ in 0..instruction.args {
            let next = self.registers.get16(registers::Registers16::PC);
            args.push(self.memory.get(next));
            self.registers.inc_pc()
        }
        println!("\tcalling instruction: {} with args: {:X?}", instruction.label, args);

        instruction.call(&mut self.registers, &mut self.memory, args);
    }

    pub fn dump_map(&mut self) {
        self.memory.dump_map()
    }
}

pub fn new<'a>(
    registers:&'a mut registers::Registers,
    instructions:&'a instructions::Instructions,
    memory:&'a mut memory::RAM,
    program:&'a program::Program
) -> CPU <'a> {
    CPU {
        registers:registers,
        instructions:instructions,
        memory:memory,
        program:program
    }
}

