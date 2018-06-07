mod memory;
mod opcodes;

fn main() {
    let mut mem = memory::init();

    memory::dump_map(&mem);
    mem[0x0000] = 12;
    memory::dump_map(&mem);
}


