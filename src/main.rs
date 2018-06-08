mod memory;
mod registers;
mod instructions;
mod program;

fn main() {
    let memory = memory::init();
    let _registers = registers::init();
    let _instructions = instructions::init();

    test_memory(memory)
}

fn test_memory(mut memory:memory::RAM) {
    memory::dump_map(&memory);
    memory.set(0x0000, 12);
    memory::dump_map(&memory);
}


