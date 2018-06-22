use std::fs::File;
use std::io::Read;

use device::Device;

pub struct BootRom {
    pub storage: [u8;256]
}

impl Device for BootRom {
    fn get(&self, a:u16) -> u8 {
        self.storage[a as usize]
    }

    fn set(&mut self, a:u16, v:u8) {
        self.storage[a as usize] = v;
    }
}

pub fn load_from_file(filename:String) -> BootRom {
    let mut f = File::open(filename).unwrap();
    // let mut buf=[0u8;16384];
    let mut c = BootRom { storage: [0;256] };
    f.read(&mut c.storage).unwrap();
    c
}
