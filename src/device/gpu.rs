use device::Device;

pub struct VRam {
    pub storage: [u8; 8192]
}

impl Device for VRam {
    fn get(&self, a:u16) -> u8 {
        self.storage[a as usize]
    }

    fn set(&mut self, a:u16, v:u8) {
        self.storage[a as usize] = v;
    }
}

