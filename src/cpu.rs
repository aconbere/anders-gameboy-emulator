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
    mmu: &'a mut mmu::MMU,
    cycles: u32,
    state: State
}

impl <'a> CPU <'a> {
    pub fn run(&mut self) {
        loop {
            self.next_frame()
        }
    }

    pub fn next_frame(&mut self) {
        println!("FRAME");
        while self.cycles <= 70244 {
            match self.state {
                State::Running => {
                    self.cycles += self.next(false) as u32
                },
                State::Prefix => {
                    self.cycles += self.next(true) as u32;
                    self.state = State::Running
                },
                State::Halted => {

                }
            }
        }
        self.cycles -= 70244
    }

    pub fn next(&mut self, prefix:bool) -> u8 {
        println!("TICK: Prefix: {}", prefix);

        let pc = self.registers.get16(&registers::Registers16::PC);
        println!("\tpc: {}", pc);

        let opcode = self.mmu.get(pc);
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
                    args.push(self.mmu.get(next));
                    self.registers.inc_pc()
                }

                println!("\tcalling instruction: {:?} with args: {:X?}", instruction, args);

                instruction.call(&mut self.registers, &mut self.mmu, args)
            }
        }

    }
}

pub fn new<'a>(
    registers:&'a mut registers::Registers,
    instructions:&'a instructions::Instructions,
    mmu:&'a mut mmu::MMU,
) -> CPU <'a> {
    CPU {
        registers:registers,
        instructions:instructions,
        mmu:mmu,
        cycles:0,
        state: State::Running,
    }
}
