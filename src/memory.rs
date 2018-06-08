use std::ops::Range;

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

pub fn init() -> RAM {
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
