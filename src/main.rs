mod memory;
mod registers;
mod instructions;

fn main() {
    let mut memory = memory::init();
    let mut registers = registers::init();
    let instructions = instructions::init();

}

fn test_memory(memory:memory::Memory) {
    memory::dump_map(&memory);
    memory[0x0000] = 12;
    memory::dump_map(&memory);
}


