mod bytes;
mod memory;
mod registers;
mod instructions;
mod program;
mod cpu;

fn main() {
    let mut registers = registers::new();
    let instructions = instructions::new();
    let mut memory = memory::new();
    memory::initialize(&mut memory);
    memory.set(0x00FF, 12);
    memory.set_space(memory::Kind::InterruptEnableFlag, &[13]);
    let program = [0;512];
    let mut cpu = cpu::new(&mut registers, &instructions, &mut memory, &program);
    cpu.run();
    //cpu.dump_map();
}
