use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Shade {
    White,
    LightGrey,
    DarkGrey,
    Black,
}

fn get_shade(i: u8) -> Shade {
    match i {
        0 => Shade::White,
        1 => Shade::LightGrey,
        2 => Shade::DarkGrey,
        3 => Shade::Black,
        _ => panic!("invalid shade index: {}", i),
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
    storage: u8,
    shades: [Shade;4],
}

impl fmt::Debug for Palette {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Palette: [{:?},{:?},{:?},{:?}]", self.shades[0], self.shades[1], self.shades[2], self.shades[3])
    }
}

impl Palette {
    pub fn get(&self) -> u8 {
        self.storage
    }

    pub fn set(&mut self, v: u8) {
        self.storage = v;

        /* we take our mask 00000011 in binary and we check what the value is
         * at for those bits in storage. Then we shift the mask over to check
         * the next two bytes, and shift the result back to get back to the
         * numeric result
         */
        let mask = 0x03;

        self.shades = [
            get_shade(v & mask),
            get_shade((v & (mask << 2)) >> 2),
            get_shade((v & (mask << 4)) >> 4),
            get_shade((v & (mask << 6)) >> 6),
        ];
    }

    pub fn map_shades(&self, i: u8) -> Shade {
        match i {
            0 => self.shades[0],
            1 => self.shades[1],
            2 => self.shades[2],
            3 => self.shades[3],
            _ => panic!("invalid shade index: {}", i),
        }
    }
}

pub fn new() -> Palette {
    Palette {
        storage: 0,
        shades: [Shade::White;4],
    }
}
