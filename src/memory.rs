use std::ops::Range;
use bytes;

pub static GBC_BOOT_ROM: &'static [u8] = &[
    0x31, 0xFE, 0xFF, 0xAF, 0x21, 0xFF, 0x9F, 0x32, 0xCB, 0x7C, 0x20, 0xFB, 0x21, 0x26, 0xFF, 0x0E,
    0x11, 0x3E, 0x80, 0x32, 0xE2, 0x0C, 0x3E, 0xF3, 0xE2, 0x32, 0x3E, 0x77, 0x77, 0x3E, 0xFC, 0xE0,
    0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1A, 0xCD, 0x95, 0x00, 0xCD, 0x96, 0x00, 0x13, 0x7B,
    0xFE, 0x34, 0x20, 0xF3, 0x11, 0xD8, 0x00, 0x06, 0x08, 0x1A, 0x13, 0x22, 0x23, 0x05, 0x20, 0xF9,
    0x3E, 0x19, 0xEA, 0x10, 0x99, 0x21, 0x2F, 0x99, 0x0E, 0x0C, 0x3D, 0x28, 0x08, 0x32, 0x0D, 0x20,
    0xF9, 0x2E, 0x0F, 0x18, 0xF3, 0x67, 0x3E, 0x64, 0x57, 0xE0, 0x42, 0x3E, 0x91, 0xE0, 0x40, 0x04,
    0x1E, 0x02, 0x0E, 0x0C, 0xF0, 0x44, 0xFE, 0x90, 0x20, 0xFA, 0x0D, 0x20, 0xF7, 0x1D, 0x20, 0xF2,
    0x0E, 0x13, 0x24, 0x7C, 0x1E, 0x83, 0xFE, 0x62, 0x28, 0x06, 0x1E, 0xC1, 0xFE, 0x64, 0x20, 0x06,
    0x7B, 0xE2, 0x0C, 0x3E, 0x87, 0xE2, 0xF0, 0x42, 0x90, 0xE0, 0x42, 0x15, 0x20, 0xD2, 0x05, 0x20,
    0x4F, 0x16, 0x20, 0x18, 0xCB, 0x4F, 0x06, 0x04, 0xC5, 0xCB, 0x11, 0x17, 0xC1, 0xCB, 0x11, 0x17,
    0x05, 0x20, 0xF5, 0x22, 0x23, 0x22, 0x23, 0xC9, 0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B,
    0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E,
    0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC,
    0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E, 0x3C, 0x42, 0xB9, 0xA5, 0xB9, 0xA5, 0x42, 0x3C,
    0x21, 0x04, 0x01, 0x11, 0xA8, 0x00, 0x1A, 0x13, 0xBE, 0x00, 0x00, 0x23, 0x7D, 0xFE, 0x34, 0x20,
    0xF5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xFB, 0x86, 0x00, 0x00, 0x3E, 0x01, 0xE0, 0x50
];

pub struct RAM {
    storage: [u8; 65536]
}

pub enum Flag {
    Z
}

impl <'a> RAM {
    pub fn get(&self, address:u16) -> u8 {
        let a = address as usize;
        self.storage[a]
    }

    pub fn set(&mut self, address:u16, v:u8) {
        let a = address as usize;
        self.storage[a] = v
    }

    fn get_flag_i(&self, n:u8) -> bool {
        let f = self.get(0xFFFF);
        bytes::check_bit(f, n)
    }

    fn set_flag_i(&mut self, n:u8) {
        let f = self.get(0xFFFF);
        self.set(0xFFFF, bytes::set_bit(f, n))
    }

    pub fn set_flag(&mut self, f:Flag) {
        match f {
            Flag::Z => self.set_flag_i(7)
        }
    }

    pub fn get_flag(&mut self, f:Flag) -> bool {
        match f {
            Flag::Z => self.get_flag_i(7)
        }
    }

    pub fn clear_flag(&self, f:Flag) {
        match f {
            Flag::Z => {}
        }
    }

    pub fn set_space(&mut self, kind: Kind, v: &[u8]) {
        let r = get_address_range(kind);
        match self.storage.get_mut(r) {
            Some(s) => s.clone_from_slice(v),
            None => panic!("WTF")
        }
    }

    pub fn load_boot_rom(&mut self, rom:&[u8]) {
        self.set_space(Kind::RestartAndInterrupt, rom)
    }

    pub fn get_space(&self, kind: Kind) -> &[u8] {
        let r = get_address_range(kind);
        &self.storage[r.start..r.end]
    }

    pub fn dump_space(&self, kind:Kind) {
        let space = self.get_space(kind);

        for i in space {
            print!("{:X},", i)
        }
        print!("\n\n")
    }

    pub fn dump_map(&mut self) {
        println!("RestartAndInterrupt");
        self.dump_space(Kind::RestartAndInterrupt);
        println!("CartridgeHeader");
        self.dump_space(Kind::CartridgeHeader);
        println!("CartridgeROMBank0");
        self.dump_space(Kind::CartridgeROMBank0);
        println!("CartridgeROMBankSwitchable");
        self.dump_space(Kind::CartridgeROMBankSwitchable);
        println!("CharacterRAM");
        self.dump_space(Kind::CharacterRAM);
        println!("BackgroundMapData1");
        self.dump_space(Kind::BackgroundMapData1);
        println!("BackgroundMapData2");
        self.dump_space(Kind::BackgroundMapData2);
        println!("CartridgeRAM");
        self.dump_space(Kind::CartridgeRAM);
        println!("InternalRAMBank0");
        self.dump_space(Kind::InternalRAMBank0);
        println!("InternalRAMBankSwitchable");
        self.dump_space(Kind::InternalRAMBankSwitchable);
        println!("EchoRAM");
        self.dump_space(Kind::EchoRAM);
        println!("ObjectAttributeMemory");
        self.dump_space(Kind::ObjectAttributeMemory);
        println!("UnusableMemory");
        self.dump_space(Kind::UnusableMemory);
        println!("HardwareIORegisters");
        self.dump_space(Kind::HardwareIORegisters);
        println!("ZeroPage");
        self.dump_space(Kind::ZeroPage);
        println!("InterruptEnableFlag");
        self.dump_space(Kind::InterruptEnableFlag);
    }

}

pub fn new() -> RAM {
    RAM {
        storage: [0; 65536]
    }
}

pub fn initialize(r:&mut RAM) {
    r.load_boot_rom(GBC_BOOT_ROM)
}

pub enum Kind {
    RestartAndInterrupt,
    CartridgeHeader,
    CartridgeROMBank0,
    CartridgeROMBankSwitchable,
    CharacterRAM,
    BackgroundMapData1,
    BackgroundMapData2,
    CartridgeRAM,
    InternalRAMBank0,
    InternalRAMBankSwitchable,
    EchoRAM,
    ObjectAttributeMemory,
    UnusableMemory,
    HardwareIORegisters,
    ZeroPage,
    InterruptEnableFlag,
}

pub fn get_address_range(k:Kind) -> Range<usize> {
    match k {
        Kind::RestartAndInterrupt => 0x0000..0x0100,
        Kind::CartridgeHeader => 0x0100..0x0150,
        Kind::CartridgeROMBank0 => 0x0150..0x4000,
        Kind::CartridgeROMBankSwitchable => 0x4000..0x8000,
        Kind::CharacterRAM => 0x8000..0x9800,
        Kind::BackgroundMapData1 => 0x9800..0x9C00,
        Kind::BackgroundMapData2 => 0x9C00..0xA000,
        Kind::CartridgeRAM => 0xA000..0xC000,
        Kind::InternalRAMBank0 => 0xC000..0xD000,
        Kind::InternalRAMBankSwitchable => 0xD000..0xE000,
        Kind::EchoRAM => 0xE000..0xFE00,
        Kind::ObjectAttributeMemory => 0xFE00..0xFEA0,
        Kind::UnusableMemory => 0xFEA0..0xFF00,
        Kind::HardwareIORegisters => 0xFF00..0xFF80,
        Kind::ZeroPage => 0xFF80..0xFFFF,
        Kind::InterruptEnableFlag => 0xFFFF..0x10000,
    }
}

/* Memory space mappings
 * http://gameboy.mongenel.com/dmg/asmmemmap.html
 */


