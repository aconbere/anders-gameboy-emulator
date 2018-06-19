use bytes;
use device;

pub enum Flag {
    Z,
    C,
    N,
    H,
}

impl Flag {
    pub fn get_index(&self) -> u8  {
        match self {
            Flag::Z => 7,
            Flag::N => 6,
            Flag::H => 5,
            Flag::C => 4,
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
    pub fn get_flag(&self, f:Flag) -> bool {
        let i = f.get_index();
        bytes::check_bit(self.f, i)
    }

    pub fn set_flag(&mut self, f:Flag) {
        self.f = bytes::set_bit(self.f, f.get_index());
    }

    pub fn clear_flag(&mut self, f:Flag) {
        self.f = bytes::clear_bit(self.f, f.get_index());
    }

}

pub fn new () -> Flags {
    Flags{f: 0x0000}
}
