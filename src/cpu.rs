use config;
use instructions;
use mmu;
use registers;

#[derive(PartialEq)]
pub enum State {
    Halted,
    Running,
    Prefix,
}

pub struct CPU {
    state: State,
    config: config::Config,
}

pub struct Context {
    pc: u16,
    cb: bool,
    opcode: u8,
    instruction: instructions::Op,
    args: Vec<u8>,
}

pub fn new_context() -> Context {
    Context {
        pc: 0,
        cb: false,
        opcode: 0,
        instruction: instructions::Op::NotImplemented,
        args: vec!()
    }
}

fn log_context(context: &Context) {
    let pc_f = format!("{:X}", context.pc);
    let cb_f = if context.cb { "CB-" } else { "" };
    let opcode_f = format!("{}{:X}", cb_f, context.opcode);
    let instruction_f = format!("{:?}", context.instruction);
    let args_f = format!("{:X?}", context.args);

    println!("pc: {:<4} | {:<4} | {:<20} | {:<10}", pc_f, opcode_f, instruction_f, args_f);
}

impl CPU {
    pub fn tick(
        &mut self,
        instructions: &instructions::Instructions,
        registers: &mut registers::Registers,
        mmu: &mut mmu::MMU,
    ) -> u8 {
        match self.state {
            State::Running => {
                let mut context = new_context();
                let instruction = self.fetch(&mut context, instructions, registers, mmu, false);
                self.execute(&mut context, &instruction, registers, mmu)
            }
            State::Prefix => {
                let mut context = new_context();
                let instruction = self.fetch(&mut context, instructions, registers, mmu, true);
                self.execute(&mut context, &instruction, registers, mmu)
            }
            State::Halted => 0,
        }
    }

    fn fetch(
        &mut self,
        context: &mut Context,
        instructions: &instructions::Instructions,
        registers: &mut registers::Registers,
        mmu: &mut mmu::MMU,
        prefix: bool,
    ) -> instructions::Op {
        let pc = registers.get16(&registers::Registers16::PC);
        let opcode = mmu.get(pc);

        context.pc = pc;
        context.opcode = opcode;

        registers.inc_pc();

        if prefix {
            *instructions.get_cb(opcode)
        } else {
            *instructions.get(opcode)
        }
    }

    pub fn execute(
        &mut self,
        context: &mut Context,
        instruction: &instructions::Op,
        mut registers: &mut registers::Registers,
        mmu: &mut mmu::MMU,
    ) -> u8 {
        match instruction {
            instructions::Op::PrefixCB => {
                self.state = State::Prefix;
                0
            }
            instructions::Op::HALT => {
                self.state = State::Halted;
                0
            }
            instructions::Op::NotImplemented => {
                context.cb = self.state == State::Prefix;
                log_context(&context);
                // panic!("Not Implemented");
                0
            }
            _ => {
                let mut args = Vec::new();
                for _ in 0..instruction.args() {
                    let next = registers.get16(&registers::Registers16::PC);
                    args.push(mmu.get(next));
                    registers.inc_pc()
                }

                let cycles = instruction.call(&mut registers, mmu, &args);

                context.instruction = *instruction;
                context.args = args;
                context.cb = self.state == State::Prefix;
                if self.config.debug.log_instructions {
                    log_context(&context);
                }
                self.state = State::Running;
                cycles
            }
        }
    }
}

pub fn new(config: config::Config) -> CPU {
    CPU {
        state: State::Running,
        config: config,
    }
}
