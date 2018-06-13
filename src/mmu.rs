/* Memory space mappings
 * http://gameboy.mongenel.com/dmg/asmmemmap.html
 */

use mmu::device::Device;

pub static GBM_BOOT_ROM: [u8;256] = [
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

pub struct MMU <'a> {
    restart_and_interupt: device::RestartAndInterrupt<'a>,
    cartridge_header: device::NotImplemented,
    cartridge_rom_bank_0: device::NotImplemented,
    cartridge_rom_bank_1: device::NotImplemented,
    character_ram: device::NotImplemented,
    background_map_data_1: device::NotImplemented,
    background_map_data_2: device::NotImplemented,
    cartridge_ram: device::NotImplemented,
    internal_ram_bank_0: device::NotImplemented,
    internal_ram_bank_1: device::NotImplemented,
    echo_ram: device::NotImplemented,
    object_attribute_memory: device::NotImplemented,
    unusable_memory: device::NotImplemented,
    hardware_io_registers: device::NotImplemented,
    zero_page: device::NotImplemented,
    pub interupt_enable_flag: device::Flags,
}

impl <'a> MMU <'a> {
    pub fn get(&self, address:u16) -> u8 {
        let k = device::get_kind(address);

        match k {
            device::Kind::RestartAndInterrupt => self.restart_and_interupt.get(address),
            device::Kind::CartridgeHeader => self.cartridge_header.get(address),
            device::Kind::CartridgeROMBank0 => self.cartridge_header.get(address),
            device::Kind::CartridgeROMBank1 => self.cartridge_header.get(address),
            device::Kind::CharacterRAM => self.cartridge_header.get(address),
            device::Kind::BackgroundMapData1 => self.cartridge_header.get(address),
            device::Kind::BackgroundMapData2 => self.cartridge_header.get(address),
            device::Kind::CartridgeRAM => self.cartridge_header.get(address),
            device::Kind::InternalRAMBank0 => self.cartridge_header.get(address),
            device::Kind::InternalRAMBank1 => self.cartridge_header.get(address),
            device::Kind::EchoRAM => self.cartridge_header.get(address),
            device::Kind::ObjectAttributeMemory => self.cartridge_header.get(address),
            device::Kind::UnusableMemory => self.cartridge_header.get(address),
            device::Kind::HardwareIORegisters => self.cartridge_header.get(address),
            device::Kind::ZeroPage => self.cartridge_header.get(address),
            device::Kind::InterruptEnableFlag => self.cartridge_header.get(address),
        }
    }

    pub fn set(&mut self, address:u16, v:u8) {
        let k = device::get_kind(address);

        match k {
            device::Kind::RestartAndInterrupt => self.restart_and_interupt.set(address, v),
            device::Kind::CartridgeHeader => self.cartridge_header.set(address, v),
            device::Kind::CartridgeROMBank0 => self.cartridge_rom_bank_0.set(address, v),
            device::Kind::CartridgeROMBank1 => self.cartridge_rom_bank_1.set(address, v),
            device::Kind::CharacterRAM => self.character_ram.set(address, v),
            device::Kind::BackgroundMapData1 => self.background_map_data_1.set(address, v),
            device::Kind::BackgroundMapData2 => self.background_map_data_2.set(address, v),
            device::Kind::CartridgeRAM => self.cartridge_ram.set(address, v),
            device::Kind::InternalRAMBank0 => self.internal_ram_bank_0.set(address, v),
            device::Kind::InternalRAMBank1 => self.internal_ram_bank_1.set(address, v),
            device::Kind::EchoRAM => self.echo_ram.set(address, v),
            device::Kind::ObjectAttributeMemory => self.object_attribute_memory.set(address, v),
            device::Kind::UnusableMemory => self.unusable_memory.set(address, v),
            device::Kind::HardwareIORegisters => self.hardware_io_registers.set(address, v),
            device::Kind::ZeroPage => self.zero_page.set(address, v),
            device::Kind::InterruptEnableFlag => self.interupt_enable_flag.set(address, v),
        }
    }
}

pub fn new<'a> () -> MMU <'a> {
    MMU {
        restart_and_interupt: device::RestartAndInterrupt{ storage: GBM_BOOT_ROM },
        cartridge_header: device::NotImplemented{},
        cartridge_rom_bank_0: device::NotImplemented{},
        cartridge_rom_bank_1: device::NotImplemented{},
        character_ram: device::NotImplemented{},
        background_map_data_1: device::NotImplemented{},
        background_map_data_2: device::NotImplemented{},
        cartridge_ram: device::NotImplemented{},
        internal_ram_bank_0: device::NotImplemented{},
        internal_ram_bank_1: device::NotImplemented{},
        echo_ram: device::NotImplemented{},
        object_attribute_memory: device::NotImplemented{},
        unusable_memory: device::NotImplemented{},
        hardware_io_registers: device::NotImplemented{},
        zero_page: device::NotImplemented{},
        interupt_enable_flag: device::Flags{f:0x0000},
    }
}

pub mod device {
    use bytes;

    pub enum Kind {
        RestartAndInterrupt,
        CartridgeHeader,
        CartridgeROMBank0,
        CartridgeROMBank1,
        CharacterRAM,
        BackgroundMapData1,
        BackgroundMapData2,
        CartridgeRAM,
        InternalRAMBank0,
        InternalRAMBank1,
        EchoRAM,
        ObjectAttributeMemory,
        UnusableMemory,
        HardwareIORegisters,
        ZeroPage,
        InterruptEnableFlag,
    }

    pub fn get_kind(address:u16) -> Kind {
        match address {
            0x0000...0x00FF => Kind::RestartAndInterrupt,
            0x0100...0x014F => Kind::CartridgeHeader,
            0x0150...0x3FFF => Kind::CartridgeROMBank0,
            0x4000...0x7FFF => Kind::CartridgeROMBank1,
            0x8000...0x97FF => Kind::CharacterRAM,
            0x9800...0x9BFF => Kind::BackgroundMapData1,
            0x9C00...0x9FFF => Kind::BackgroundMapData2,
            0xA000...0xBFFF => Kind::CartridgeRAM,
            0xC000...0xCFFF => Kind::InternalRAMBank0,
            0xD000...0xDFFF => Kind::InternalRAMBank1,
            0xE000...0xFDFF => Kind::EchoRAM,
            0xFE00...0xFE9F => Kind::ObjectAttributeMemory,
            0xFEA0...0xFEFF => Kind::UnusableMemory,
            0xFF00...0xFF7F => Kind::HardwareIORegisters,
            0xFF80...0xFFFE => Kind::ZeroPage,
            0xFFFF...0xFFFF => Kind::InterruptEnableFlag,
            _ => panic!("WTF: address: {:X} isn't covered", address),
        }
    }

    pub trait Device {
        fn get(&self, a:u16) -> u8;
        fn set(&mut self, a:u16, v:u8);
    }

    pub struct NotImplemented {
    }

    impl Device for NotImplemented {
        fn get(&self, _:u16) -> u8 {
            panic!("Not Implemented")
        }

        fn set(&mut self, _:u16, _:u8) {
            panic!("Not Implemented")
        }
    }

    pub enum Flag {
        Z
    }

    impl Flag {
        pub fn get_index(&self) -> u8  {
            match self {
                Flag::Z => 7,
            }
        }
    }

    pub struct Flags {
        pub f: u8
    }

    impl Device for Flags {
        fn get(&self, _:u16) -> u8 {
            self.f
        }

        fn set(&mut self, _:u16, v:u8) {
            self.f = v
        }
    }

    impl Flags {
        pub fn get_flag(&mut self, f:Flag) -> bool {
            let i = f.get_index();
            return bytes::check_bit(self.f, i)
        }

        pub fn set_flag(&mut self, f:Flag) {
            let i = f.get_index();
            bytes::set_bit(self.f, i);
        }

        pub fn clear_flag(&self, f:Flag) {
            let i = f.get_index();
            bytes::clear_bit(self.f, i);
        }
    }

    pub struct RestartAndInterrupt<'a> {
        storage: &'a[u8; 256]
    }

    impl <'a> Device for RestartAndInterrupt <'a>{
        fn get(&self, a:u16) -> u8 {
            self.storage[a as usize]
        }

        fn set(&mut self, a:u16, v:u8) {
            self.storage[a as usize] = v;
        }
    }

}
