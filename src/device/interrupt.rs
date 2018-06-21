use device::Device;


#[derive(Debug, Clone, Copy)]
pub enum Flags {
    VBlank,
    LCDStat,
    Timer,
    Serial,
    Joypad,
}

pub static FLAG_LOOKUP: [Flags;5] = [
    Flags::VBlank, Flags::LCDStat, Flags::Timer, Flags::Serial, Flags::Joypad
];

pub struct Enabled {
    f: u8,
}

impl Device for Enabled {
    fn get(&self, _:u16) -> u8 {
        self.f
    }

    fn set(&mut self, _:u16, v:u8) {
        self.f = v;
    }
}

impl Enabled {
    pub fn get_enabled_interrupts(&self) -> u8 {
        self.f
    }
}

pub fn new_enabled() -> Enabled {
    Enabled {
        f:0,
    }
}


pub fn flags(enabled:u8, requested:u8) -> Vec<Flags> {
    let masked = enabled & requested;

    let mut flags = vec!();

    for i in 0..5 {
        if  masked & (1 << i) != 0 {
            flags.push(FLAG_LOOKUP[i]);
        }
    }

    flags
}
