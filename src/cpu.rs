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
        match self.state {
            State::Running =>
                self.sub_tick(instructions, registers, mmu, false),
            State::Prefix => {
                self.state = State::Running;
                self.sub_tick(instructions, registers, mmu, true)
            },
            State::Halted => 0,
        }
    }

    pub fn sub_tick(
        &mut self,
        instructions: &instructions::Instructions,
        mut registers: &mut registers::Registers,
        mmu:&mut mmu::MMU,
        prefix:bool
    ) -> u8 {
        println!("TICK: Prefix: {}", prefix);

        let pc = registers.get16(&registers::Registers16::PC);
        println!("\tpc: {}", pc);

        let opcode = mmu.get(pc);
        registers.inc_pc();
        println!("\topcode: {:X}", opcode);

        let instruction = if prefix {
            instructions.get_cb(opcode)
        } else {
            instructions.get(opcode)
        };

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
