use std::fs::File;
use std::io::Read;

use device::Device;

#[derive(Debug)]
pub enum States {
    Booting,
    Running,
}

pub struct Cartridge {
    cartridge: [u8; 32767],
    boot_rom: [u8; 256],
    state: States,
}

impl Device for Cartridge {
    fn get(&self, a: u16) -> u8 {
        match self.state {
            States::Booting => {
                if a < 256 {
                    self.boot_rom[a as usize]
                } else {
                    self.cartridge[a as usize]
                }
            }
            States::Running => {
                let v = self.cartridge[a as usize];
                v
            }
        }
    }

    fn set(&mut self, a: u16, v: u8) {
        match self.state {
            States::Booting => {
                if a < 256 {
                    panic!("can't mangle boot rom");
                } else {
                    self.cartridge[a as usize] = v;
                }
            }
            States::Running => self.cartridge[a as usize] = v,
        }
    }
}

impl Cartridge {
    pub fn set_state(&mut self, state: States) {
        self.state = state;
    }
}

pub fn load_cartridge(filename: String) -> [u8; 32767] {
    let mut f = File::open(filename).unwrap();
    let mut m = [0; 32767];
    f.read(&mut m).unwrap();
    m
}

pub fn load_boot_rom(filename: String) -> [u8; 256] {
    let mut f = File::open(filename).unwrap();
    let mut m = [0; 256];
    f.read(&mut m).unwrap();
    m
}

pub fn new(boot_rom: [u8; 256], cartridge: [u8; 32767]) -> Cartridge {
    Cartridge {
        boot_rom: boot_rom,
        cartridge: cartridge,
        state: States::Booting,
    }
}
