use bytes;


pub struct TileMap {
    storage: [u8;1024] 
}

impl TileMap {
    pub fn get(&self, a:u16) -> u8 {
        self.storage[a as usize]
    }
    pub fn set(&mut self, a:u16, v:u8) {
        self.storage[a as usize] = v;
    }
}

pub struct TileData {
    storage: [u8;4096] 
}

impl TileData {
    pub fn get(&self, a:u16) -> u8 {
        self.storage[a as usize]
    }
    pub fn set(&mut self, a:u16, v:u8) {
        self.storage[a as usize] = v;
    }
}


impl TileData {
    pub fn get_tile(&self, index:u8) -> Tile {
        let offset = (index as u16 * 16) as usize;
        let mut arr = [0;16];
        arr.clone_from_slice(&self.storage[offset..offset+16]);
        Tile {
            storage: arr
        }
    }
}


pub fn new_tile_map() -> TileMap {
    TileMap {
        storage: [0;1024]
    }
}

pub fn new_tile_data() -> TileData {
    TileData {
        storage: [0;4096]
    }
}

#[derive(Debug)]
pub struct Tile {
    storage: [u8;16]
}

/*
 * Tiles are 8x8 pixels and layed out where every two bits defines a pixel. They are
 * 16 bytes and with every two bytes defining a row. Oddly pixel are aligned vertically
 * in these two byte rows... for example
 *
 * [0,1,0,0,1,1,1,0]
 * [1,0,0,0,1,0,1,1]
 *  
 * results in a row of pixels:
 * [2,1,0,0,3,1,3.2]
 */
impl Tile {
    pub fn get_pixel(&self, x:u8, y:u8) -> u8 {
        let y_offset = y * 2;
        bytes::get_bit(self.storage[y_offset as usize], x) + bytes::get_bit(self.storage[(y_offset + 1) as usize], x)
    }
}
