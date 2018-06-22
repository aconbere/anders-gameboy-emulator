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
    mode: Mode,
}

pub fn new() -> GPU {
    GPU {
        mode_clock: 0,
        lines: 0,
        mode: Mode::OAM,
    }
}

impl GPU {
    pub fn tick(&mut self, mmu:&mut mmu::MMU, cycles:u8) {
        self.mode_clock += cycles as u32;

        match self.mode {
            Mode::OAM => {
                println!("GPU: OAM Mode");
                if self.mode_clock >= 80 {
                    self.mode = Mode::VRAM;
                }
            },
            Mode::VRAM => {
                println!("GPU: VRAM Mode");
                if self.mode_clock >= 252 {
                    self.mode = Mode::HBlank;
                }
            },
            Mode::HBlank => {
                println!("GPU: HBLANK Mode");
                if self.mode_clock >= 456 {
                    self.lines += 1;
                    self.mode_clock -= 456;

                    if self.lines == 144 {
                        self.lines = 0;
                        self.mode = Mode::VBlank;
                    } else {
                        self.mode = Mode::OAM;
                    }
                }
            },
            Mode::VBlank => {
                println!("GPU: VBLANK Mode");
                if self.mode_clock >= 4560 {
                    self.mode_clock -= 4560;
                    self.mode = Mode::OAM;
                }
            },
        }
    }
}
