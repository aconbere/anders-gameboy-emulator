/* Memory space mappings
 * http://gameboy.mongenel.com/dmg/asmmemmap.html
 */

use device;
use bytes;
use device::Device;

pub struct MMU<'a> {
    pub restart_and_interrupt: &'a mut device::restart_and_interrupt::RestartAndInterrupt,
    pub cartridge: &'a mut device::cartridge::Cartridge,
    pub video_ram: &'a mut device::vram::VRam,
    pub cartridge_ram: &'a mut device::not_implemented::NotImplemented,
    pub internal_ram_bank_0: &'a mut device::not_implemented::NotImplemented,
    pub internal_ram_bank_1: &'a mut device::not_implemented::NotImplemented,
    pub echo_ram: &'a mut device::not_implemented::NotImplemented,
    pub object_attribute_memory: &'a mut device::not_implemented::NotImplemented,
    pub unusable_memory: &'a mut device::not_implemented::NotImplemented,
    pub hardware_io: &'a mut device::hardware_io::HardwareIO,
    pub zero_page: &'a mut device::zero_page::ZeroPage,
    pub interrupt_enable_flag: &'a mut device::flags::Flags,
}

impl <'a> MMU <'a> {
    pub fn set_flag(&mut self, f:device::flags::Flag, v:bool) {
        if v {
            self.interrupt_enable_flag.set_flag(f);
        } else {
            self.interrupt_enable_flag.clear_flag(f);
        }
    }

    pub fn get_flag(&self, f:device::flags::Flag) -> bool {
        self.interrupt_enable_flag.get_flag(f)
    }


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
            device::Kind::InterruptEnableFlag => self.interrupt_enable_flag.get(address),
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
            device::Kind::InternalRAMBank0 => self.internal_ram_bank_0.set(address, v),
            device::Kind::InternalRAMBank1 => self.internal_ram_bank_1.set(address, v),
            device::Kind::EchoRAM => self.echo_ram.set(address, v),
            device::Kind::ObjectAttributeMemory => self.object_attribute_memory.set(address, v),
            device::Kind::UnusableMemory => self.unusable_memory.set(address, v),
            device::Kind::HardwareIORegisters => self.hardware_io.set(address - 0xFF00, v),
            device::Kind::ZeroPage => self.zero_page.set(address - 0xFF80, v),
            device::Kind::InterruptEnableFlag => self.interrupt_enable_flag.set(address, v),
        }
    }
}
