mod memory;
mod registers;
mod instructions;
mod program;
mod cpu;

fn main() {
    let registers = registers::new();
    let instructions = instructions::new();
    let memory = memory::new();
    let program = [0;512];
    let mut cpu = cpu::new(registers, instructions, memory, program);
    cpu.run()
}
