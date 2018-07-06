#[derive(Debug, Clone, Copy)]
pub enum Shade {
    White,
    LightGrey,
    DarkGrey,
    Black
}

pub fn get_shade(i:u8) -> Shade {
    match i {
        0 => Shade::White,
        1 => Shade::LightGrey,
        2 => Shade::DarkGrey,
        3 => Shade::Black,
        _ => panic!("invalid shade index"),
    }
}

/* A palette defines how to take tile data and turn it into
 * the color space of the gameboy (four shades). A palette
 * is broken into four, two bit segments
 *
 * [ 0-1, 2-3, 4-5, 6-7]
 *
 * each of those two bit segments can represent a shade [0-3]
 * between white and black.
 */
pub struct Palette {
    storage: u8
}

pub fn map_shade(shades:&[Shade;4], i:u8) -> Shade {
    match i {
        0 => shades[0],
        1 => shades[1],
        2 => shades[2],
        3 => shades[3],
        _ => panic!("invalid shade index"),
    }
}

impl Palette {
    pub fn get(&self) -> u8 {
        self.storage
    }

    pub fn set(&mut self, v:u8) {
        self.storage = v
    }

    pub fn get_shades(&self) -> [Shade;4] {
        println!("Palette: {:b}", self.storage);
        let mask = 0x03;

        /* we take our mask 00000011 in binary and we check what the value is
         * at for those bits in storage. Then we shift the mask over to check
         * the next two bytes, and shift the result back to get back to the
         * numeric result
         */
        return [
            get_shade(self.storage & mask),
            get_shade((self.storage & (mask << 2)) >> 2),
            get_shade((self.storage & (mask << 4)) >> 4),
            get_shade((self.storage & (mask << 6)) >> 6),
        ]
    }
}

pub fn new() -> Palette {
    Palette{storage:0}
}
