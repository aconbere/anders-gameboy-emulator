use std::fs::File;
use std::path::Path;

extern crate sdl2;
#[macro_use]
extern crate clap;

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
    let matches = clap_app!(anders_gameboy_emulator =>
        (version: "0.1")
        (author: "Anders Conbere <anders@conbere.org>")
        (about: "Emulates a gameboy")
        (@arg BOOT_ROM: --boot_rom +takes_value +required "The file of the boot rom to load")
        (@arg GAME_ROM: --game_rom +takes_value +required "The file of the game rom to load")
        // (@subcommand debug =>
        //     (@arg verbose: -v --verbose "Print debug information verbosely")
        // )
    ).get_matches();

    let mut boot_rom = File::open(Path::new(matches.value_of("BOOT_ROM").unwrap())).unwrap();
    let mut game_rom = File::open(Path::new(matches.value_of("GAME_ROM").unwrap())).unwrap();

    let mut gameboy = gameboy::new(&mut boot_rom, &mut game_rom);
    let mut display = display::new();

    display.start(&mut gameboy);
}
