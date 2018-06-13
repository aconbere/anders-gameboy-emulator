mod bytes;
mod registers;
mod instructions;
mod program;
mod cpu;
mod mmu;
mod device;

fn main() {
    let mut registers = registers::new();
    let instructions = instructions::new();
    let mut mmu = mmu::new();
    let mut cpu = cpu::new(&mut registers, &instructions, &mut mmu);
    cpu.run();
    //cpu.dump_map();
}
