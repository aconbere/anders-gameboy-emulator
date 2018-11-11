use std::fs::File;
use std::io::Read;
use device::Device;

pub struct BootRom {
    storage: [u8;256]
}

impl Device for BootRom {
    fn get(&self, a: u16) -> u8 {
        self.storage[a as usize]
    }

    fn set(&mut self, a: u16, v: u8) {
        self.storage[a as usize] = v;
    }
}


pub fn new (f: &mut File) -> BootRom {
    let mut m = [0; 256];
    f.read(&mut m).unwrap();
    BootRom{ storage: m }
}

#[cfg(test)]
pub fn zero() -> BootRom {
    BootRom{ storage: [0;256] }
}
