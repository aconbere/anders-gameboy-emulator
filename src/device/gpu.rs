use device::Device;

/*
    Modes
        Line: 456
            VRAM: 172
            OAM: 80
            Hblank: 204
        VBlank: 4560
*/


pub enum Mode {
    OAM,
    VRAM,
    HBlank,
    VBlank,
}

pub struct GPU {
    pub storage: [u8; 8192],
    mode: Mode,
    mode_clock: u16,
    lines: u16,
}

pub fn new() -> GPU{
    GPU {
        storage: [0;8192],
        mode: Mode::OAM,
        mode_clock: 0,
        lines: 0,
    }
}

impl Device for GPU {
    fn get(&self, a:u16) -> u8 {
        self.storage[a as usize]
    }

    fn set(&mut self, a:u16, v:u8) {
        self.storage[a as usize] = v;
    }
}


impl GPU {
    pub fn get_mode(&self) -> Mode {
        if self.lines >= 144 {
            Mode::VBlank
        } else {
            match self.mode_clock {
                0...80 => Mode::OAM,
                80...252 => Mode::VRAM,
                252...456 => Mode::HBlank,
                _ => panic!("invalid mode_clock value")
            }
        }
    }

    // the mode clock should be set by the timings from the previous cpu instruction
    pub fn tick(&mut self, cycles:u8) {
        self.mode_clock += cycles as u16;
        match self.get_mode() {
            Mode::OAM => {
                println!("GPU: OAM Mode");
            },
            Mode::VRAM => {
                println!("GPU: VRAM Mode");
            },
            Mode::HBlank => {
                println!("GPU: HBLANK Mode");
                self.mode_clock = 0;
                self.lines += 1;
            },
            Mode::VBlank => {
                println!("GPU: VBLANK Mode");
                self.lines = 0;
            },
        }
    }
}
