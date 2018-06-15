use ::bytes;

#[derive(Debug, Clone, Copy)]
pub enum Registers8 {
    A,B,C,D,E,F,H,L
}

#[derive(Debug, Clone, Copy)]
pub enum Registers16 {
    AF,BC,DE,HL,PC,SP
}

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
    pub fn get8(&self, r:&Registers8) -> u8 {
        match r {
            Registers8::A => self.a,
            Registers8::B => self.b,
            Registers8::C => self.c,
            Registers8::D => self.d,
            Registers8::E => self.e,
            Registers8::F => self.f,
            Registers8::H => self.h,
            Registers8::L => self.l
        }
    }
    pub fn get16(&self, r:&Registers16) -> u16 {
        match r {
            // Registers16::AF => bytes::combine_little(self.a, self.b),
            // Registers16::BC => bytes::combine_little(self.b, self.c),
            // Registers16::DE => bytes::combine_little(self.d, self.e),
            // Registers16::HL => bytes::combine_little(self.h, self.l),

            Registers16::AF => bytes::combine_little(self.b, self.f),
            Registers16::BC => bytes::combine_little(self.c, self.b),
            Registers16::DE => bytes::combine_little(self.e, self.d),
            Registers16::HL => bytes::combine_little(self.l, self.h),
            Registers16::PC => self.pc,
            Registers16::SP => self.sp
        }
    }

    pub fn set8(&mut self, r:&Registers8, v:u8) {
        match r {
            Registers8::A => self.a = v,
            Registers8::B => self.b = v,
            Registers8::C => self.c = v,
            Registers8::D => self.d = v,
            Registers8::E => self.e = v,
            Registers8::F => self.f = v,
            Registers8::H => self.h = v,
            Registers8::L => self.l = v
        }
    }

    pub fn set16(&mut self, r:&Registers16, v:u16) {
        match r {
            // Registers16::AF => self.set_combined(&Registers8::A, &Registers8::F, v),
            // Registers16::BC => self.set_combined(&Registers8::B, &Registers8::C, v),
            // Registers16::DE => self.set_combined(&Registers8::D, &Registers8::E, v),
            // Registers16::HL => self.set_combined(&Registers8::H, &Registers8::L, v),

            Registers16::AF => self.set_combined(&Registers8::F, &Registers8::A, v),
            Registers16::BC => self.set_combined(&Registers8::C, &Registers8::B, v),
            Registers16::DE => self.set_combined(&Registers8::E, &Registers8::D, v),
            Registers16::HL => self.set_combined(&Registers8::L, &Registers8::H, v),
            Registers16::PC => self.pc = v,
            Registers16::SP => self.sp = v
        }
    }

    pub fn inc_pc(&mut self) { self.pc = self.pc + 1 }

    pub fn dec_hl(&mut self) {
        let hl = self.get16(&Registers16::HL);
        self.set16(&Registers16::HL, hl - 1)
    }

    pub fn inc_hl(&mut self) {
        let hl = self.get16(&Registers16::HL) + 1;
        self.set16(&Registers16::HL, hl)
    }

    fn set_combined(&mut self, r1:&Registers8, r2:&Registers8, v:u16) {
        let (high, low) = bytes::split_u16(v);
        self.set8(r1, low);
        self.set8(r2, high);
    }
}

pub fn new() -> Registers {
    // return Registers{ a:0, b:0, c:0, d:0, e:0, f:0, h:0, l:0, sp:0xFFFE, pc:0x0100 }
    return Registers{ a:0, b:0, c:0, d:0, e:0, f:0, h:0, l:0, sp:0xFFFE, pc:0x0000 }
}

