extern crate sdl2;

mod bytes;
mod cpu;
mod device;
mod tile;
mod display;
mod framebuffer;
mod gameboy;
mod gpu;
mod instructions;
mod mmu;
mod palette;
mod registers;

fn main() {
    let mut gameboy = gameboy::new();
    let mut display = display::new();

    display.start(&mut gameboy);
}
