use std::fmt;

#[derive(Copy)]
pub struct TileMap {
    storage: [u8; 1024],
}

impl TileMap {
    pub fn get(&self, a: u16) -> u8 {
        self.storage[a as usize]
    }

    pub fn set(&mut self, a: u16, v: u8) {
        self.storage[a as usize] = v;
    }
}

impl Clone for TileMap {
    fn clone(&self) -> TileMap {
        *self
    }
}

impl fmt::Debug for TileMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut i = 0;
        for _ in 0..31 {
            for _ in 0..31 {
                let _ = write!(f, "{:X},", self.storage[i]);
                i += 1;
            }
            let _ = write!(f, "\n");
        }
        write!(f, "")
    }
}

pub fn new() -> TileMap {
    TileMap { storage: [0; 1024] }
}
