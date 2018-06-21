use device::Device;

pub struct RamBank {
    storage: [u8;4096]
}

impl Device for RamBank {
    fn get(&self, a:u16) -> u8 {
        self.storage[a as usize]
    }

    fn set(&mut self, a:u16, v:u8) {
        self.storage[a as usize] = v;
    }
}

pub fn new() -> RamBank {
    RamBank{
        storage: [0;4096]
    }
}
