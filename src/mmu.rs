/* Memory space mappings
 * http://gameboy.mongenel.com/dmg/asmmemmap.html
 */

use device;
use bytes;
use device::Device;

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


pub struct MMU {
    pub restart_and_interrupt: device::restart_and_interrupt::RestartAndInterrupt,
    pub cartridge: device::cartridge::Cartridge,
    pub video_ram: device::vram::VRam,
    pub cartridge_ram: device::not_implemented::NotImplemented,
    pub internal_ram_bank_0: device::ram_bank::RamBank,
    pub internal_ram_bank_1: device::ram_bank::RamBank,
    pub echo_ram: device::not_implemented::NotImplemented,
    pub object_attribute_memory: device::not_implemented::NotImplemented,
    pub unusable_memory: device::not_implemented::NotImplemented,
    pub hardware_io: device::hardware_io::HardwareIO,
    pub zero_page: device::zero_page::ZeroPage,
    pub interrupt_enable: device::interrupt::Enabled,
}

impl MMU {
    pub fn get16(&self, address:u16) -> u16 {
        let mh = self.get(address);
        let ml = self.get(address + 1);
        bytes::combine_little(mh, ml)
    }

    pub fn get(&self, address:u16) -> u8 {
        let k = device::get_kind(address);

        match k {
            device::Kind::RestartAndInterrupt => self.restart_and_interrupt.get(address),

            device::Kind::CartridgeHeader | device::Kind::CartridgeROMBank0 | device::Kind::CartridgeROMBank1 =>
                self.cartridge.get(address),

            device::Kind::CharacterRAM | device::Kind::BackgroundMapData1 | device::Kind::BackgroundMapData2 =>
                self.video_ram.get(address - 0x8000),

            device::Kind::CartridgeRAM => self.cartridge_ram.get(address),
            device::Kind::InternalRAMBank0 => self.internal_ram_bank_0.get(address),
            device::Kind::InternalRAMBank1 => self.internal_ram_bank_1.get(address),
            device::Kind::EchoRAM => self.echo_ram.get(address),
            device::Kind::ObjectAttributeMemory => self.object_attribute_memory.get(address),
            device::Kind::UnusableMemory => self.unusable_memory.get(address),
            device::Kind::HardwareIORegisters => self.hardware_io.get(address - 0xFF00),
            device::Kind::ZeroPage => self.zero_page.get(address - 0xFF80),
            device::Kind::InterruptEnableFlag => self.interrupt_enable.get(address),
        }
    }

    pub fn set(&mut self, address:u16, v:u8) {
        let k = device::get_kind(address);

        match k {
            device::Kind::RestartAndInterrupt => self.restart_and_interrupt.set(address, v),
            device::Kind::CartridgeHeader | device::Kind::CartridgeROMBank0 | device::Kind::CartridgeROMBank1 =>
                self.cartridge.set(address, v),

            device::Kind::CharacterRAM | device::Kind::BackgroundMapData1 | device::Kind::BackgroundMapData2 =>
                self.video_ram.set(address - 0x8000, v),

            device::Kind::CartridgeRAM => self.cartridge_ram.set(address, v),
            device::Kind::InternalRAMBank0 => self.internal_ram_bank_0.set(address - 0xC000, v),
            device::Kind::InternalRAMBank1 => self.internal_ram_bank_1.set(address - 0xD000, v),
            device::Kind::EchoRAM => self.echo_ram.set(address, v),
            device::Kind::ObjectAttributeMemory => self.object_attribute_memory.set(address, v),
            device::Kind::UnusableMemory => self.unusable_memory.set(address, v),
            device::Kind::HardwareIORegisters => self.hardware_io.set(address - 0xFF00, v),
            device::Kind::ZeroPage => self.zero_page.set(address - 0xFF80, v),
            device::Kind::InterruptEnableFlag => self.interrupt_enable.set(address, v),
        }
    }
}

pub fn new() -> MMU {
    let cartridge = device::cartridge::load_from_file(
        // String::from("/Users/anders/Projects/gb_test_roms/sheepitup.gb")
        String::from("/Users/anders/Projects/gb_test_roms/Mona_And_The_Witch_Hat.gb")
    );

    MMU {
        restart_and_interrupt: device::restart_and_interrupt::new(GBM_BOOT_ROM),
        cartridge: cartridge,
        video_ram: device::vram::new(),
        cartridge_ram: device::not_implemented::NotImplemented{},
        internal_ram_bank_0: device::ram_bank::new(),
        internal_ram_bank_1: device::ram_bank::new(),
        echo_ram: device::not_implemented::NotImplemented{},
        object_attribute_memory: device::not_implemented::NotImplemented{},
        unusable_memory: device::not_implemented::NotImplemented{},
        hardware_io: device::hardware_io::new(),
        zero_page: device::zero_page::new(),
        interrupt_enable: device::interrupt::new_enabled(),
    }
}
