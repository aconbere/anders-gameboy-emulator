use bytes;

#[derive(Debug)]
pub struct Tile {
    pub storage: [u8; 16],
}

/*
 * Tiles are 8x8 pixels and layed out where every two bits defines a pixel. They are
 * 16 bytes and with every two bytes defining a row. Oddly pixels are aligned vertically
 * in these two byte rows... for example
 *
 * [0,1,0,0,1,1,1,0]
 * [1,0,0,0,1,0,1,1]
 *
 * results in a row of pixels:
 * [2,1,0,0,3,1,3.2]
 */
impl Tile {
    pub fn get_pixel(&self, x: u8, y: u8) -> u8 {
        let y_offset = y * 2;
        let top_byte = self.storage[y_offset as usize];
        let bottom_byte = self.storage[(y_offset + 1) as usize];
        let tb = bytes::check_bit(top_byte, 7 - x);
        let bb = bytes::check_bit(bottom_byte, 7 - x);
        bytes::add_bool(tb, bb)
    }

    pub fn is_zero(&self) -> bool {
        for i in self.storage.iter() {
            if *i != 0 {
                return true;
            }
        }
        false
    }
}

