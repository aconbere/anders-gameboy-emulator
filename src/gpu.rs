use mmu;
use device;
use device::hardware_io::LCDControlFlag;

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

fn render_line(mmu: &mmu::MMU) -> [device::hardware_io::Shade;160] {
    // get our y-offset, this wont change per scan line
    let y_offset = mmu.hardware_io.lcd_line_count.get() + mmu.hardware_io.lcd_scroll_position_y;

    /* Fetch the currently active palette
     */
    let pallet = mmu.hardware_io.background_palette.get_shades();

    let mut line:[device::hardware_io::Shade;160] = [device::hardware_io::Shade::White;160];

    let mut i = 0;
    // for each pixel in a line
    while i < 160 {
        /* The Tile Map is a 32x32 array where every byte is a reference to where in the tile data
         * to pull tile data from.
         * 
         * So for every line we're on, we jump forward 32 tiles. We take the current x scroll
         * position and add to it where we are in rendering this line (i). Then we want to find
         * which tile we would land on, so divide our current index (i) by 32 to find which of the
         * 32 tiles we are on.
         *
         * We'll later need to know which index in the tile we're at so record that while we're at
         * it.
         */

        /* x offset tells us which pixel in the line we're on. we have to take this and map it into
         * which tile it would be
         */
        let x_offset = mmu.hardware_io.lcd_scroll_position_x + i;

        /* The first tile in a line might start at any pixel in the tile from 0 to 8
         * later tiles will always start at 0
         */
        let tile_index_y = (y_offset / 32) + (y_offset % 32);
        let tile_index_x = if i == 0 {
            (x_offset / 32) + (x_offset % 32)
        } else {
            0
        };


        /* to get where in the tile data to look, first we firgure out which tile map is enabled.
         * then we look up there to find the data in the tile map index
         */
        let tile_map_index = (y_offset * 32) + (x_offset / 32);
        let tile_data_index = if mmu.hardware_io.lcd_control_register.get_flag(LCDControlFlag::TileMapSelect) {
            mmu.tile_map_1.get(tile_map_index);
        } else {
            mmu.tile_map_2.get(tile_map_index);
        };

        /* We check what tile data set is enabled and use the tile data index found previously to
         * fetch a tile.
         */
        let tile_data = if mmu.hardware_io.lcd_control_register.get_flag(LCDControlFlag::TileDataSelect) {
            mmu.tile_data_1.get_tile(tile_data_index);
        } else {
            mmu.tile_data_2.get_tile(tile_data_index);
        };

        /* Once we have a tile, we need to actually index into the tile at the right location
         */

        /* For each pixel in the tile render the pixel. Now... of course this can't be simple.
         *
         * that are then mapped through a pallet
         */
        for tx in tile_index_x..8 {
            let pixel = tile_data.get_pixel(tx, tile_index_y);
            // get the pallet for the tile
            // render the RGB values into a struct
            line[i as usize] = device::hardware_io::get_shade(pixel);
            i+=1;
        }
    }

    return line;
}

impl GPU {
    pub fn new_frame_available(&self) -> bool {
        self.frame_available
    }

    pub fn tick(&mut self, mmu:&mut mmu::MMU, cycles:u8) {
        self.mode_clock += cycles as u32;
        let framebuffer = [0;184320];

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
                    let line = render_line(&*mmu);
                    println!("LINE: {:?}", line);
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
