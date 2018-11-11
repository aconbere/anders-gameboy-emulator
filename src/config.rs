use std::fs::File;
use std::path::Path;
use std::io;

#[derive(Debug, Copy, Clone)]
pub struct Debug {
    pub frame_count: bool,
    pub log_instructions: bool,
    pub log_register_states: bool,
    pub break_point_pc: Option<u16>,
    pub break_point_frame: Option<u32>,
    pub repl: bool,
}

pub fn new_debug(
    frame_count:bool,
    log_instructions:bool,
    log_register_states:bool,
    break_point_frame: Option<&str>,
    break_point_pc: Option<&str>,
    repl: bool,
) -> Result<Debug, String> {
    let bk_pc = break_point_pc.map(|r| u16::from_str_radix(r, 16).unwrap());
    let bk_frame = break_point_frame.map(|r| u32::from_str_radix(r, 16).unwrap());

    Ok(Debug {
            frame_count: frame_count,
            log_instructions: log_instructions,
            log_register_states: log_register_states,
            break_point_pc: bk_pc,
            break_point_frame: bk_frame,
            repl: repl,
        })
}

pub fn debug_default() -> Debug {
    Debug {
        frame_count: false,
        log_instructions: false,
        log_register_states: false,
        break_point_pc: None,
        break_point_frame: None,
        repl: false,
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub boot_rom: String,
    pub game_rom: String,
    pub debug: Debug,
}

fn read_file(path: &String) -> io::Result<File> {
    File::open(Path::new(path))
}

impl Config {
    pub fn read_boot_rom(&self) -> io::Result<File> {
        read_file(&self.boot_rom)
    }

    pub fn read_game_rom(&self) -> io::Result<File> {
        read_file(&self.game_rom)
    }
}

pub fn new(
    boot_rom_path:&str,
    game_rom_path:&str,
    debug: Debug
) -> Result<Config, String> {
    if !Path::new(boot_rom_path).exists() {
        return Err(format!("Boot rom path does not exist: {}", boot_rom_path));
    }

    if !Path::new(game_rom_path).exists() {
        return Err(format!("Game rom path does not exist: {}", game_rom_path));
    }

    Ok(Config {
        boot_rom: String::from(boot_rom_path),
        game_rom: String::from(game_rom_path),
        debug: debug,
    })
}
