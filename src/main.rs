mod bytes;
mod registers;
mod instructions;
mod program;
mod cpu;
mod mmu;

fn main() {
    let mut registers = registers::new();
    let instructions = instructions::new();

    let mut mmu = mmu::new();
    memory::initialize(&mut memory);

    let program = [0;512];
    let mut cpu = cpu::new(&mut registers, &instructions, &mut memory, &program);
    cpu.run();
    //cpu.dump_map();
}
