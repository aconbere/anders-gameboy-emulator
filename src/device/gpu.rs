use mmu;

pub enum Mode {
    OAM,
    VRAM,
    HBlank,
    VBlank,
}

pub struct GPU {
    mode_clock: u32,
    lines: u16,
}

pub fn new() -> GPU {
    GPU {
        mode_clock: 0,
        lines: 0,
    }
}

impl GPU {
    pub fn get_mode(&self) -> Mode {
        match self.mode_clock {
            0...80 => Mode::OAM,
            80...252 => Mode::VRAM,
            252...456 => Mode::HBlank,
            456...70224 => Mode::VBlank,
            _ => panic!("invalid mode clock"),
        }
    }

    // the mode clock should be set by the timings from the previous cpu instruction
    pub fn tick(&mut self, mmu:&mut mmu::MMU, cycles:u8) {
        self.mode_clock += cycles as u32;

        if self.mode_clock > 70224 {
            self.mode_clock -= 70224;
            self.lines = 0;
        }

        match self.get_mode() {
            Mode::OAM => {
                println!("GPU: OAM Mode");
            },
            Mode::VRAM => {
                println!("GPU: VRAM Mode");
            },
            Mode::HBlank => {
                println!("GPU: HBLANK Mode");
                self.lines += 1;
            },
            Mode::VBlank => {
                println!("GPU: VBLANK Mode");
            },
        }
    }
}
