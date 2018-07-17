use tile;

pub enum TileDataKind {
    Top,
    Bottom,
}

pub struct TileData {
    storage: [u8; 4096],
    kind: TileDataKind,
}

impl TileData {
    pub fn get(&self, a: u16) -> u8 {
        self.storage[a as usize]
    }

    pub fn set(&mut self, a: u16, v: u8) {
        self.storage[a as usize] = v
    }
}

impl TileData {
    pub fn get_tile(&self, index: u8) -> tile::Tile {
        let offset = (index as u16 * 16) as usize;
        let mut arr = [0; 16];

        arr.clone_from_slice(&self.storage[offset..offset + 16]);
        tile::Tile { storage: arr }
    }
}

pub fn new(kind:TileDataKind) -> TileData {
    TileData {
        storage: [0; 4096],
        kind: kind,
    }
}
