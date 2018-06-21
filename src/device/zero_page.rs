use device::Device;

pub struct ZeroPage {
    pub storage: [u8; 127]
}

impl Device for ZeroPage {
    fn get(&self, a:u16) -> u8 {
        self.storage[a as usize]
    }

    fn set(&mut self, a:u16, v:u8) {
        self.storage[a as usize] = v;
    }
}

pub fn new() -> ZeroPage {
    ZeroPage {
        storage: [0;127]
    }
}
