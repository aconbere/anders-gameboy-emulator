use device::Device;

pub struct HardwareIO {
    pub storage: [u8; 128]
}

impl Device for HardwareIO {
    fn get(&self, a:u16) -> u8 {
        self.storage[a as usize]
    }

    fn set(&mut self, a:u16, v:u8) {
        self.storage[a as usize] = v;
    }
}
