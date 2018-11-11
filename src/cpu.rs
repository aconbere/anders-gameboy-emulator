use config;
use instructions;
use mmu;
use registers;
use registers::Registers16;
use registers::Flag;

#[derive(PartialEq)]
pub enum State {
    Halted,
    Running,
    Prefix,
}

pub struct CPU {
    state: State,
    log_instructions: bool,
    log_register_states: bool,
}

pub struct Context {
    pc: u16,
    cb: bool,
    opcode: u8,
    instruction: instructions::Op,
    args: Vec<u8>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            pc: 0,
            cb: false,
            opcode: 0,
            instruction: instructions::Op::NotImplemented,
            args: vec!()
        }
    }
}

fn log_register_states(context: &Context, registers: &registers::Registers) {
    let pc = registers.get16(&Registers16::PC);
    let af = registers.get16(&Registers16::AF);
    let bc = registers.get16(&Registers16::BC);
    let de = registers.get16(&Registers16::DE);
    let hl = registers.get16(&Registers16::HL);
    let sp = registers.get16(&Registers16::SP);

    println!("{:04X}{:04X}{:04X}{:04X}{:04X}{:04X}", pc, af, bc, de, hl, sp);
}

fn log_context(context: &Context, registers: &registers::Registers) {
    let pc_f = format!("{:04X}", context.pc);
    let cb_f = if context.cb { "CB-" } else { "" };
    let opcode_f = format!("{}{:04X}", cb_f, context.opcode);
    let instruction_f = format!("{:?}", context.instruction);
    let args_f = format!("{:04X?}", context.args);

    let af = registers.get16(&Registers16::AF);
    let bc = registers.get16(&Registers16::BC);
    let de = registers.get16(&Registers16::DE);
    let hl = registers.get16(&Registers16::HL);
    let pc = registers.get16(&Registers16::PC);
    let sp = registers.get16(&Registers16::SP);

    let f_z = if registers.get_flag(Flag::Z) { "Z" } else { "-" };
    let f_n = if registers.get_flag(Flag::N) { "N" } else { "-" };
    let f_h = if registers.get_flag(Flag::H) { "H" } else { "-" };
    let f_c = if registers.get_flag(Flag::C) { "C" } else { "-" };

    println!("pc: {:<4} | {:<4} | {:<20} | {:<10}", pc_f, opcode_f, instruction_f, args_f);
    println!("AF: {:04X} BC: {:04X} DE: {:04X} HL: {:04X} PC: {:04X} SP {:04X}", af, bc, de, hl, pc, sp);
    println!("[{}{}{}{}]", f_z, f_n, f_h, f_c);
}

impl CPU {
    pub fn set_log_instructions(&mut self, state: bool) {
        self.log_instructions = state;
    }

    pub fn tick(
        &mut self,
        instructions: &instructions::Instructions,
        registers: &mut registers::Registers,
        mmu: &mut mmu::MMU,
    ) -> u8 {
        match self.state {
            State::Running => {
                let mut context = Context::new();
                let instruction = self.fetch(&mut context, instructions, registers, mmu, false);
                self.execute(&mut context, &instruction, registers, mmu)
            }
            State::Prefix => {
                let mut context = Context::new();
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
        let pc = registers.get16(&Registers16::PC);
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
                log_context(&context, &registers);
                // panic!("Not Implemented");
                0
            }
            _ => {
                let mut args = Vec::new();
                for _ in 0..instruction.args() {
                    let next = registers.get16(&Registers16::PC);
                    args.push(mmu.get(next));
                    registers.inc_pc()
                }

                let cycles = instruction.call(&mut registers, mmu, &args);

                context.instruction = *instruction;
                context.args = args;
                context.cb = self.state == State::Prefix;
                if self.log_instructions {
                    log_context(&context, &registers);
                }
                if self.log_register_states {
                    log_register_states(&context, &registers);
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
        log_instructions: config.debug.log_instructions,
        log_register_states: config.debug.log_register_states,
    }
}
