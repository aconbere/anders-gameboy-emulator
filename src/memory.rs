use std::ops::Range;

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
]

pub struct RAM {
    storage: [u8; 65536]
}

impl RAM {
    pub fn get(&self, address:u16) -> u8 {
        let a = address as usize;
        self.storage[a]
    }

    pub fn set(&mut self, address:u16, v:u8) {
        let a = address as usize;
        self.storage[a] = v
    }
}

pub fn new() -> RAM {
    RAM {
        storage: [0; 65536]
    }
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

struct Space {
    range: Range<u16>,
    description: String
}

/* Memory space mappings
 * http://gameboy.mongenel.com/dmg/asmmemmap.html
 */
fn lookup_space(k: Kind) -> Space {
    match k {
        Kind::RestartAndInterrupt => Space {
            range: 0x0000..0x00FF,
            description: String::from("Restart and Interupt")
        },
        Kind::CartridgeHeader => Space {
            range: 0x0100..0x014F,
            description: String::from("Cartridge Header")
        },
        Kind::CartridgeROMBank0 => Space {
            range: 0x0150..0x3FFF,
            description: String::from("Cartridge ROM Bank 0")
        },
        Kind::CartridgeROMBankSwitchable => Space {
            range: 0x4000..0x7FFF,
            description: String::from("Cartridge ROM Bank Switchable")
        },
        Kind::CharacterRAM => Space {
            range: 0x8000..0x97FF,
            description: String::from("Character RAM")
        },
        Kind::BackgroundMapData1 => Space {
            range: 0x9800..0x9BFF,
            description: String::from("Background Map Data 1")
        },
        Kind::BackgroundMapData2 => Space {
            range: 0x9C00..0x9FFF,
            description: String::from("Background Map Data 2")
        },
        Kind::CartridgeRAM => Space {
            range: 0xA000..0xBFFF,
            description: String::from("Cartridge RAM")
        },
        Kind::InternalRAMBank0 => Space {
            range: 0xC000..0xCFFF,
            description: String::from("Internal RAM Bank 0")
        },
        Kind::InternalRAMBankSwitchable => Space {
            range: 0xD000..0xDFFF,
            description: String::from("Internal Ram Bank Switchable")
        },
        Kind::EchoRAM => Space {
            range:0xE000..0xFDFF,
            description: String::from("Echo RAM")
        },
        Kind::ObjectAttributeMemory => Space {
            range: 0xFE00..0xFE9F,
            description: String::from("Object Attribute Memory")
        },
        Kind::UnusableMemory => Space {
            range: 0xFEA0..0xFEFF,
            description: String::from("Unusable Memory")
        },
        Kind::HardwareIORegisters => Space {
            range: 0xFF00..0xFF7F,
            description: String::from("Hardware IO Registers")
        },
        Kind::ZeroPage => Space {
            range: 0xFF80..0xFFFE,
            description: String::from("Zero Page")
        },
        Kind::InterruptEnableFlag => Space {
            range: 0x0000..0x00FF,
            description: String::from("Interupt Enable Flag")
        },
    }
}

pub fn dump_space(memory:&RAM, kind:Kind) {
    let space = lookup_space(kind);

    print!("{}: \n", space.description);

    for i in space.range {
        print!("{}", memory.get(i))
    }
    print!("\n\n")
}

pub fn dump_map(memory:&RAM) {
    dump_space(memory, Kind::RestartAndInterrupt);
    dump_space(memory, Kind::CartridgeHeader);
    dump_space(memory, Kind::CartridgeROMBank0);
    dump_space(memory, Kind::CartridgeROMBankSwitchable);
    dump_space(memory, Kind::CharacterRAM);
    dump_space(memory, Kind::BackgroundMapData1);
    dump_space(memory, Kind::BackgroundMapData2);
    dump_space(memory, Kind::CartridgeRAM);
    dump_space(memory, Kind::InternalRAMBank0);
    dump_space(memory, Kind::InternalRAMBankSwitchable);
    dump_space(memory, Kind::EchoRAM);
    dump_space(memory, Kind::ObjectAttributeMemory);
    dump_space(memory, Kind::UnusableMemory);
    dump_space(memory, Kind::HardwareIORegisters);
    dump_space(memory, Kind::ZeroPage);
    dump_space(memory, Kind::InterruptEnableFlag);
}

