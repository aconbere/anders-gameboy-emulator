use bytes;
use device;

pub enum Flag {
    Z
}

impl Flag {
    pub fn get_index(&self) -> u8  {
        match self {
            Flag::Z => 7,
        }
    }
}

pub struct Flags {
    pub f: u8
}

impl device::Device for Flags {
    fn get(&self, _:u16) -> u8 {
        self.f
    }

    fn set(&mut self, _:u16, v:u8) {
        self.f = v
    }
}

impl Flags {
    pub fn get_flag(&mut self, f:Flag) -> bool {
        let i = f.get_index();
        return bytes::check_bit(self.f, i)
    }

    pub fn set_flag(&mut self, f:Flag) {
        let i = f.get_index();
        bytes::set_bit(self.f, i);
    }

    pub fn clear_flag(&self, f:Flag) {
        let i = f.get_index();
        bytes::clear_bit(self.f, i);
    }
}
