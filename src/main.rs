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

fn test_memory(mut memory:memory::RAM) {
    memory::dump_map(&memory);
    memory.set(0x0000, 12);
    memory::dump_map(&memory);
}


