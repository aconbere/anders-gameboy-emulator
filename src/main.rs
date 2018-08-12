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
mod config;
mod repl;

fn main() {
    let matches = clap_app!(anders_gameboy_emulator =>
        (version: "0.1")
        (author: "Anders Conbere <anders@conbere.org>")
        (about: "Emulates a gameboy")
        (@arg BOOT_ROM: --boot_rom +takes_value +required "The file of the boot rom to load")
        (@arg GAME_ROM: --game_rom +takes_value +required "The file of the game rom to load")
        (@subcommand debug =>
            (@arg FRAME_COUNT: --frame_count "Print frame count to display.")
            (@arg LOG_INSTRUCTIONS: --log_instructions "Print each instruction to stdout.")
            (@arg BREAK_POINT_FRAME: --break_point_frame +takes_value "Frame to pause instruction at.")
            (@arg BREAK_POINT_PC: --break_point_pc +takes_value "Frame to pause instruction at.")
            (@arg REPL: --repl "Boots the emulator into debug mode.")
        )
    ).get_matches();

    let debug = match matches.subcommand_matches("debug") {
        Some(debug_matches) => config::new_debug(
            debug_matches.is_present("FRAME_COUNT"),
            debug_matches.is_present("LOG_INSTRUCTIONS"),
            debug_matches.value_of("BREAK_POINT_FRAME"),
            debug_matches.value_of("BREAK_POINT_PC"),
            debug_matches.is_present("REPL"),
        ).unwrap(),

        None => config::debug_default(),
    };

    let config = config::new(
        matches.value_of("BOOT_ROM").unwrap(),
        matches.value_of("GAME_ROM").unwrap(),
        debug,
    ).unwrap();

    let mut gameboy = gameboy::new(&config);
    let mut display = display::new(&config);

    display.start(&mut gameboy);
}
