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
    pub fn get_row(&self, y: u8) -> [u8;8] {
        let y_offset = y * 2;
        let top_byte = self.storage[y_offset as usize];
        let bottom_byte = self.storage[(y_offset + 1) as usize];

        let mut arr = [0;8];

        for i in 0..8 {
            let m = 0x01 << i;
            let tb = (top_byte & m) >> i;
            let bb = (bottom_byte & m) >> i;
            arr[i as usize] = tb + bb;
        }

        arr
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

