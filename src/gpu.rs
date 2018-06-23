use mmu;

#[derive(PartialEq)]
pub enum Mode {
    OAM,
    VRAM,
    HBlank,
    VBlank,
}

pub struct GPU {
    mode_clock: u32,
    frame_available: bool,
    pub mode: Mode,
}

pub fn new() -> GPU {
    GPU {
        mode_clock: 0,
        mode: Mode::OAM,
        frame_available: false,
    }
}

impl GPU {
    pub fn new_frame_available(&self) -> bool {
        self.frame_available
    }

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
                    self.mode_clock -= 456;

                    mmu.hardware_io.lcd_line_count.inc();

                    if mmu.hardware_io.lcd_line_count.get() == 144 {
                        self.frame_available = true;
                        self.mode = Mode::VBlank;
                    } else {
                        self.mode = Mode::OAM;
                    }
                }
            },
            Mode::VBlank => {
                println!("GPU: VBLANK Mode");
                self.frame_available = false;

                if self.mode_clock >= 456 {
                    self.mode_clock -= 456;
                    mmu.hardware_io.lcd_line_count.inc();
                }

                if mmu.hardware_io.lcd_line_count.get() == 153 {
                    println!("GPU: FRAME COMPLETE");
                    self.mode = Mode::OAM;
                }
            },
        }
    }
}

/* 
 * Bit 7: LCD Display Enable             (0=Off, 1=On)
 * Bit 6: Window Tile Map Display Select (0=0x9800-0x9BFF, 1=0x9C00-0x9FFF)
 * Bit 5: Window Display Enable          (0=Off, 1=On)
 * Bit 4: BG & Window Tile Data Select   (0=0x8800-0x97FF, 1=0x8000-0x8FFF)
 * Bit 3: BG Tile Map Display Select     (0=0x9800-0x9BFF, 1=0x9C00-0x9FFF)
 * Bit 2: OBJ (Sprite) Size              (0=8x8, 1=8x16)
 * Bit 1: OBJ (Sprite) Display Enable    (0=Off, 1=On)
 * Bit 0: BG Display                     (0=Off, 1=On)
*/
