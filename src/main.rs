extern crate sdl2;

mod bytes;
mod registers;
mod instructions;
mod cpu;
mod gpu;
mod mmu;
mod device;
mod display;
mod gameboy;

fn main() {
    let mut gameboy = gameboy::new();

    loop {
        gameboy.next_frame();
    }
}
