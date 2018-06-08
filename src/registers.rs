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
    pub fn get_af(&self) -> u16 { join_registers(self.a, self.b) }
    pub fn get_bc(&self) -> u16 { join_registers(self.b, self.c) }
    pub fn get_de(&self) -> u16 { join_registers(self.d, self.e) }
    pub fn get_hl(&self) -> u16 { join_registers(self.h, self.l) }
    pub fn get_pc(&self) -> u16 { self.pc }
    pub fn get_sp(&self) -> u16 { self.sp }

    pub fn set_a(&mut self, n:u8) { self.a = n }
    pub fn set_pc(&mut self, n:u16) { self.pc = n }
}

pub fn new() -> Registers {
    // return Registers{ a:0, b:0, c:0, d:0, e:0, f:0, h:0, l:0, sp:0xFFFE, pc:0x0100 }
    return Registers{ a:0, b:0, c:0, d:0, e:0, f:0, h:0, l:0, sp:0xFFFE, pc:0x0000 }
}

fn join_registers(a1:u8, b1:u8) -> u16 {
    let a2 = a1 as u16;
    let b2 = b1 as u16;
    match a2.checked_shl(8) {
        Some(s) =>  s | b2,
        None => panic!("Invalid shift results")
    }
}
