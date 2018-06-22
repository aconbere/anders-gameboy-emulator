use std::fs::File;
use std::io::Read;

use device::Device;

pub struct Cartridge {
    pub storage: [u8;32512]
}

impl Device for Cartridge {
    fn get(&self, a:u16) -> u8 {
        self.storage[a as usize]
    }

    fn set(&mut self, a:u16, v:u8) {
        self.storage[a as usize] = v;
    }
}

pub fn load_from_file(filename:String) -> Cartridge {
    let mut f = File::open(filename).unwrap();
    let mut c = Cartridge { storage: [0;32512] };
    f.read(&mut c.storage).unwrap();
    c
}
