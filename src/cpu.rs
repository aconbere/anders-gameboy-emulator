use ::registers;
use ::instructions;
use ::mmu;

#[derive(PartialEq)]
pub enum State {
    Halted,
    Running,
    Prefix,
}

pub struct CPU {
    state: State
}

impl CPU {
    pub fn tick(
        &mut self,
        instructions: &instructions::Instructions,
        registers: &mut registers::Registers,
        mmu:&mut mmu::MMU
    ) -> u8 {
        println!("TICK");
        match self.state {
            State::Running => {
                let instruction = self.fetch(instructions, registers, mmu, false);
                self.execute(&instruction, registers, mmu)
            },
            State::Prefix => {
                self.state = State::Running;
                let instruction = self.fetch(instructions, registers, mmu, true);
                self.execute(&instruction, registers, mmu)
            },
            State::Halted => 0,
        }
    }

    fn fetch(
        &mut self,
        instructions: &instructions::Instructions,
        registers: &mut registers::Registers,
        mmu:&mut mmu::MMU,
        prefix:bool
    ) -> instructions::Op {
        let pc = registers.get16(&registers::Registers16::PC);
        println!("\tpc: {}", pc);

        let opcode = mmu.get(pc);
        registers.inc_pc();
        println!("\topcode: {:X}", opcode);

        if prefix {
            *instructions.get_cb(opcode)
        } else {
            *instructions.get(opcode)
        }
    }

    pub fn execute(
        &mut self,
        instruction: &instructions::Op,
        mut registers: &mut registers::Registers,
        mmu:&mut mmu::MMU
    ) -> u8 {

        match instruction {
            instructions::Op::PrefixCB => {
                self.state = State::Prefix;
                0
            },
            instructions::Op::Halt => {
                println!("HALTING!");
                self.state = State::Halted;
                0
            },
            _ => {
                println!("\tinstruction: {:?}", instruction);

                let mut args = Vec::new();
                for _ in 0..instruction.args() {
                    let next = registers.get16(&registers::Registers16::PC);
                    args.push(mmu.get(next));
                    registers.inc_pc()
                }

                println!("\tcalling instruction: {:?} with args: {:X?}", instruction, args);

                instruction.call(&mut registers, mmu, args)
            }
        }

    }
}

pub fn new() -> CPU {
    CPU {
        state: State::Running,
    }
}
