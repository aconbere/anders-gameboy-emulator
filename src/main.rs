mod memory;
mod registers;
mod instructions;
mod program;
mod cpu;

fn main() {
    let mut registers = registers::new();
    let instructions = instructions::new();
    let memory = memory::new();
    let program = [0;512];
    let mut cpu = cpu::new(&mut registers, &instructions, &memory, &program);
    cpu.run();
    memory.dump_map()

}
