use ::bytes;

pub struct Registers {
    a:u8,
    b:u8,
    c:u8,
    d:u8,
    e:u8,
    f:u8,
    h:u8,
    l:u8,
    sp:u16,
    pc:u16,
}

impl Registers {
    pub fn get_a(&self) -> u8 { self.a }
    pub fn get_b(&self) -> u8 { self.b }
    pub fn get_c(&self) -> u8 { self.c }
    pub fn get_d(&self) -> u8 { self.d }
    pub fn get_e(&self) -> u8 { self.e }
    pub fn get_f(&self) -> u8 { self.f }
    pub fn get_h(&self) -> u8 { self.h }
    pub fn get_l(&self) -> u8 { self.l }
    pub fn get_af(&self) -> u16 { bytes::combine_big(self.a, self.b) }
    pub fn get_bc(&self) -> u16 { bytes::combine_big(self.b, self.c) }
    pub fn get_de(&self) -> u16 { bytes::combine_big(self.d, self.e) }
    pub fn get_hl(&self) -> u16 { bytes::combine_big(self.h, self.l) }
    pub fn get_pc(&self) -> u16 { self.pc }
    pub fn get_sp(&self) -> u16 { self.sp }

    pub fn set_a(&mut self, n:u8) { self.a = n }
    pub fn set_h(&mut self, n:u8) { self.h = n }
    pub fn set_l(&mut self, n:u8) { self.l = n }
    pub fn set_pc(&mut self, n:u16) { self.pc = n }
    pub fn set_hl(&mut self, n:u16) { 
        let (high, low) = bytes::split_u16(n);
        self.h = low;
        self.l = high;
    }
    pub fn set_sp(&mut self, n:u16) { self.sp = n }

    pub fn inc_pc(&mut self) { self.pc = self.pc + 1 }

    pub fn dec_hl(&mut self) {
        let hl = self.get_hl();
        self.set_hl(hl - 1);
    }
}

pub fn new() -> Registers {
    // return Registers{ a:0, b:0, c:0, d:0, e:0, f:0, h:0, l:0, sp:0xFFFE, pc:0x0100 }
    return Registers{ a:0, b:0, c:0, d:0, e:0, f:0, h:0, l:0, sp:0xFFFE, pc:0x0000 }
}

