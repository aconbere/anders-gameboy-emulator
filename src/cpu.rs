use ::registers;
use ::instructions;
use ::mmu;

#[derive(PartialEq)]
pub enum State {
    Halted,
    Running,
    Prefix,
}

pub struct CPU <'a> {
    registers: &'a mut registers::Registers,
    instructions: &'a instructions::Instructions,
    state: State
}

impl <'a> CPU <'a> {
    // pub fn next_frame(&mut self) {
    //     println!("FRAME");
    //     while self.cycles <= 70244 {
    //         self.cycles += self.tick() as u32;
    //     }
    //     self.cycles -= 70244
    // }

    pub fn tick(&mut self, mmu:&mut mmu::MMU) -> u8 {
        match self.state {
            State::Running => self.sub_tick(mmu, false),
            State::Prefix => {
                self.state = State::Running;
                self.sub_tick(mmu, true)
            },
            State::Halted => 0,
        }
    }

    pub fn sub_tick(&mut self, mmu:&mut mmu::MMU, prefix:bool) -> u8 {
        println!("TICK: Prefix: {}", prefix);

        let pc = self.registers.get16(&registers::Registers16::PC);
        println!("\tpc: {}", pc);

        let opcode = mmu.get(pc);
        self.registers.inc_pc();
        println!("\topcode: {:X}", opcode);

        let instruction = if prefix {
            self.instructions.get_cb(opcode)
        } else {
            self.instructions.get(opcode)
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
                    let next = self.registers.get16(&registers::Registers16::PC);
                    args.push(mmu.get(next));
                    self.registers.inc_pc()
                }

                println!("\tcalling instruction: {:?} with args: {:X?}", instruction, args);

                instruction.call(&mut self.registers, mmu, args)
            }
        }

    }
}

pub fn new<'a>(
    registers:&'a mut registers::Registers,
    instructions:&'a instructions::Instructions,
) -> CPU <'a> {
    CPU {
        registers:registers,
        instructions:instructions,
        state: State::Running,
    }
}
