use device::Device;

pub struct RestartAndInterrupt {
    pub storage: [u8; 256]
}

impl Device for RestartAndInterrupt {
    fn get(&self, a:u16) -> u8 {
        self.storage[a as usize]
    }

    fn set(&mut self, a:u16, v:u8) {
        self.storage[a as usize] = v;
    }
}

