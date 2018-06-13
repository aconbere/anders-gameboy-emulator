use ::registers;
use ::instructions;
use ::mmu;

pub struct CPU <'a> {
    registers: &'a mut registers::Registers,
    instructions: &'a instructions::Instructions,
    mmu: &'a mut mmu::MMU<'a>,
}

impl <'a> CPU <'a> {
    pub fn run(&mut self) {
        for i in 0..100 {
            self.next()
        }
    }
    pub fn next(&mut self) {
        println!("TICK");

        let pc = self.registers.get16(&registers::Registers16::PC);
        println!("\tpc: {}", pc);

        let opcode = self.mmu.get(pc);
        self.registers.inc_pc();
        println!("\topcode: {:X}", opcode);

        let instruction = if opcode == 0x00CB {
            let pc = self.registers.get16(&registers::Registers16::PC);
            let opcode = self.mmu.get(pc);
            self.registers.inc_pc();
            self.instructions.get_cb(opcode)
        } else {
            self.instructions.get(opcode)
        };

        println!("\tinstruction: {:?}", instruction);

        let mut args = Vec::new();
        for _ in 0..instruction.args() {
            let next = self.registers.get16(&registers::Registers16::PC);
            args.push(self.mmu.get(next));
            self.registers.inc_pc()
        }

        println!("\tcalling instruction: {:?} with args: {:X?}", instruction, args);

        instruction.call(&mut self.registers, &mut self.mmu, args);
    }
}

pub fn new<'a>(
    registers:&'a mut registers::Registers,
    instructions:&'a instructions::Instructions,
    mmu:&'a mut mmu::MMU<'a>,
) -> CPU <'a> {
    CPU {
        registers:registers,
        instructions:instructions,
        mmu:mmu,
    }
}
