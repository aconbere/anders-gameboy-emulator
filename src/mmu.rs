pub static GBM_BOOT_ROM: &'static [u8] = &[
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

pub struct MMU {
    devices:Vec<device::Device>
}

impl <'a> MMU {
    pub fn get(&self, address:u16) -> u8 {
        let a = address as usize;
        self.storage[a]
    }

    pub fn set(&mut self, address:u16, v:u8) {
        let a = address as usize;
        self.storage[a] = v
    }

    fn set_device(&self, kind:device::Kind, d:device::Device) {
        self.devices[kind.get_index()] = d;
    }

    fn get_device(&self, kind:device::Kind) -> device::Device {
        self.devices[kind.get_index()];
    }
}

pub fn new() -> MMU {
    MMU {
        devices:vec![device::NotImplemented{};16]
    }
}

pub fn initialize(&mut mmu:MMU) {
    mmu.set_device(device::Kind::RestartAndInterrupt, device::GBMBootRom{})
}

mod device {
    use bytes;

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

    fn get_kind(address:u16) -> Device {
        match address {
            0x0000...0x00FF => Kind::RestartAndInterrupt,
            0x0100...0x014F => Kind::CartridgeHeader,
            0x0150...0x3FFF => Kind::CartridgeROMBank0,
            0x4000...0x7FFF => Kind::CartridgeROMBankSwitchable,
            0x8000...0x97FF => Kind::CharacterRAM,
            0x9800...0x9BFF => Kind::BackgroundMapData1,
            0x9C00...0x9FFF => Kind::BackgroundMapData2,
            0xA000...0xBFFF => Kind::CartridgeRAM,
            0xC000...0xCFFF => Kind::InternalRAMBank0,
            0xD000...0xDFFF => Kind::InternalRAMBankSwitchable,
            0xE000...0xFDFF => Kind::EchoRAM,
            0xFE00...0xFE9F => Kind::ObjectAttributeMemory,
            0xFEA0...0xFEFF => Kind::UnusableMemory,
            0xFF00...0xFF7F => Kind::HardwareIORegisters,
            0xFF80...0xFFFE => Kind::ZeroPage,
            0xFFFF...0xFFFF => Kind::InterruptEnableFlag,
        }
    }

    impl Kind {
        pub fn get_index(&self) -> u8 {
            match self {
                Kind::RestartAndInterrupt => 1,
                Kind::CartridgeHeader => 2,
                Kind::CartridgeROMBank0 => 3,
                Kind::CartridgeROMBankSwitchable => 4,
                Kind::CharacterRAM => 5,
                Kind::BackgroundMapData1 => 6,
                Kind::BackgroundMapData2 => 7,
                Kind::CartridgeRAM => 8,
                Kind::InternalRAMBank0 => 9,
                Kind::InternalRAMBankSwitchable => 10,
                Kind::EchoRAM => 11,
                Kind::ObjectAttributeMemory => 12,
                Kind::UnusableMemory => 13,
                Kind::HardwareIORegisters => 14,
                Kind::ZeroPage => 15,
                Kind::InterruptEnableFlag => 16
            }
        }
    }

    pub trait Device {
        fn get(&self, a:u16) -> u8;
        fn set(&self, a:u16, v:u8);
    }

    pub struct NotImplemented {}

    impl Device for NotImplemented {
        fn get(&self, a:u16) -> u8 {
            panic!("Not Implemented")
        }
        fn set(&self, a:u16, v:u8) {
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
                _ => 0
            }
        }
    }



    pub struct Flags {
        f: u8
    }

    impl Device for Flags {
        fn get(&self, a:u16) -> u8 {
            self.f
        }

        fn set(&self, a:u16, v:u8) {
            self.f = v
        }
    }

    impl Flags {
        pub fn get_flag(&mut self, f:Flags) -> bool {
            let i = f.get_index();
            return bytes::check_bit(self.f, i)
        }

        pub fn set_flag(&mut self, f:Flags) {
            let i = f.get_index();
            bytes::set_bit(self.f, i)
        }

        pub fn clear_flag(&self, f:Flag) {
            let i = self.get_index(f);
            bytes::clear_bit(self.f, i)
        }
    }

    pub struct GBMBootRom {}
}

/* Memory space mappings
 * http://gameboy.mongenel.com/dmg/asmmemmap.html
 */



