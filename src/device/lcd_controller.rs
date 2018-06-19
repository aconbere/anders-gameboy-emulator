pub struct LcdController {
    pub registers: u8;
}

impl Device for LcdController {
    pub fn get(&self) -> u8 {
        self.registers
    }

    pub fn set(&mut self, v: u8) {
        self.registers = v;
    }
}

impl LcdController {
}

