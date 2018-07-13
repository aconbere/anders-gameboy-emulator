use std::fs::File;
use std::io::Read;

use device::Device;

#[derive(Debug)]
pub enum States {
    Booting,
    Running,
}

pub struct Cartridge {
    storage: [u8; 32767],
    state: States,
}

impl Device for Cartridge {
    fn get(&self, a: u16) -> u8 {
        self.storage[a as usize]
    }

    fn set(&mut self, a: u16, v: u8) {
        self.storage[a as usize] = v;
    }
}

impl Cartridge {
    pub fn set_state(&mut self, state: States) {
        self.state = state;
    }
}

pub fn new(game_rom: &mut File) -> Cartridge {
    let mut m = [0; 32767];
    game_rom.read(&mut m).unwrap();
    Cartridge {
        storage: m,
        state: States::Booting,
    }
}
