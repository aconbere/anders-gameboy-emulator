/* Registers
 * A, B, C, D, E, F, H, L
 */

pub struct Registers {
    A:u8,
    B:u8,
    C:u8,
    D:u8,
    E:u8,
    F:u8,
    H:u8,
    L:u8,
    SP:u16,
    PC:u16,
}

impl Registers {
    pub fn getA(&self) -> u8 { self.A }
    pub fn getB(&self) -> u8 { self.B }
    pub fn getC(&self) -> u8 { self.C }
    pub fn getD(&self) -> u8 { self.D }
    pub fn getE(&self) -> u8 { self.E }
    pub fn getF(&self) -> u8 { self.F }
    pub fn getH(&self) -> u8 { self.H }
    pub fn getL(&self) -> u8 { self.L }
    pub fn getAF(&self) -> u16 { joinRegisters(self.A, self.B) }
    pub fn getBC(&self) -> u16 { joinRegisters(self.B, self.C) }
    pub fn getDE(&self) -> u16 { joinRegisters(self.D, self.E) }
    pub fn getHL(&self) -> u16 { joinRegisters(self.H, self.L) }

    pub fn setA(&self, n:u8) { self.A = n }
}

pub fn init() -> Registers {
    return Registers{ A:0, B:0, C:0, D:0, E:0, F:0, H:0, L:0, SP:0xFFFE, PC:0x0100 }
}

fn joinRegisters(a1:u8, b1:u8) -> u16 {
    let a2 = a1 as u16;
    let b2 = b1 as u16;
    match a2.checked_shl(8) {
        Some(s) =>  s | b2,
        None => panic!("Invalid shift results")
    }
}
