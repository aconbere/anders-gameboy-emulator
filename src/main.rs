extern crate sdl2;

mod bytes;
mod registers;
mod instructions;
mod cpu;
mod mmu;
mod device;
mod display;

fn main() {
    let mut registers = registers::new();
    let instructions = instructions::new();
    let cartridge = device::cartridge::load_from_file(String::from("/Users/anders/Projects/gb_test_roms/sheepitup.gb"));
    let mut mmu = mmu::new(cartridge);
    let mut cpu = cpu::new(&mut registers, &instructions, &mut mmu);
    cpu.run();

    // let display = display::start(cpu);
    // cpu.dump_map();
}
