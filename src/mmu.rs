/* Memory space mappings
 * http://gameboy.mongenel.com/dmg/asmmemmap.html
 */

use bytes;
use device;
use device::Device;
use device::boot_rom;
use device::cartridge;
use device::tile_map;
use device::tile_data;

pub struct MMU {
    boot_rom_loaded: bool,

    pub boot_rom: boot_rom::BootRom,
    pub cartridge: cartridge::Cartridge,
    pub tile_map_1: tile_map::TileMap,
    pub tile_map_2: tile_map::TileMap,
    pub tile_data_1: tile_data::TileData,
    pub tile_data_2: tile_data::TileData,
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
    pub fn get16(&self, address: u16) -> u16 {
        let mh = self.get(address);
        let ml = self.get(address + 1);
        bytes::combine_little(mh, ml)
    }

    pub fn get(&self, address: u16) -> u8 {
        let k = device::get_kind(address);

        match k {
            device::Kind::RestartAndInterrupt => {
                if self.boot_rom_loaded {
                    self.boot_rom.get(address)
                } else {
                    self.cartridge.get(address)
                }
            },
            device::Kind::CartridgeHeader
            | device::Kind::CartridgeROMBank0
            | device::Kind::CartridgeROMBank1 => self.cartridge.get(address),

            device::Kind::TileData1 => self.tile_data_1.get(address - 0x8000),
            device::Kind::TileData2 => self.tile_data_2.get(address - 0x8800),
            device::Kind::TileMap1 => self.tile_map_1.get(address - 0x9800),
            device::Kind::TileMap2 => self.tile_map_2.get(address - 0x9C00),

            device::Kind::CartridgeRAM => self.cartridge_ram.get(address),
            device::Kind::InternalRAMBank0 => self.internal_ram_bank_0.get(address - 0xC000),
            device::Kind::InternalRAMBank1 => self.internal_ram_bank_1.get(address - 0xD000),
            device::Kind::EchoRAM => self.echo_ram.get(address),
            device::Kind::ObjectAttributeMemory => self.object_attribute_memory.get(address),
            device::Kind::UnusableMemory => self.unusable_memory.get(address),
            device::Kind::HardwareIORegisters => self.hardware_io.get(address - 0xFF00),
            device::Kind::ZeroPage => self.zero_page.get(address - 0xFF80),
            device::Kind::InterruptEnableFlag => self.interrupt_enable.get(address),
        }
    }

    pub fn set(&mut self, address: u16, v: u8) {
        let k = device::get_kind(address);

        match k {
            device::Kind::RestartAndInterrupt => {
                if self.boot_rom_loaded {
                    println!("can't write to boot rom address: {:X} value: {:X}", address, v)
                } else {
                    println!("can't write to cartridge rom address: {:X} value: {:X}", address, v)
                }
            },
            device::Kind::CartridgeHeader => println!("can't write to cartridge header address: {:X}, value: {:X}", address, v),
            device::Kind::CartridgeROMBank0
            | device::Kind::CartridgeROMBank1 => self.cartridge.set(address, v),

            device::Kind::TileData1 => self.tile_data_1.set(address - 0x8000, v),
            device::Kind::TileData2 => self.tile_data_2.set(address - 0x8800, v),
            device::Kind::TileMap1 => self.tile_map_1.set(address - 0x9800, v),
            device::Kind::TileMap2 => self.tile_map_2.set(address - 0x9C00, v),

            device::Kind::CartridgeRAM => self.cartridge_ram.set(address, v),
            device::Kind::InternalRAMBank0 => self.internal_ram_bank_0.set(address - 0xC000, v),
            device::Kind::InternalRAMBank1 => self.internal_ram_bank_1.set(address - 0xD000, v),
            device::Kind::EchoRAM => self.echo_ram.set(address, v),
            device::Kind::ObjectAttributeMemory => self.object_attribute_memory.set(address, v),
            device::Kind::UnusableMemory => self.unusable_memory.set(address, v),
            device::Kind::HardwareIORegisters => match address {
                0xFF50 => self.boot_rom_loaded = false,
                _ => self.hardware_io.set(address - 0xFF00, v),
            },
            device::Kind::ZeroPage => self.zero_page.set(address - 0xFF80, v),
            device::Kind::InterruptEnableFlag => self.interrupt_enable.set(address, v),
        }
    }
}

pub fn new(boot_rom: boot_rom::BootRom, cartridge: cartridge::Cartridge) -> MMU {
    MMU {
        boot_rom_loaded: true,
        boot_rom: boot_rom,
        cartridge: cartridge,
        tile_map_1: tile_map::new(),
        tile_map_2: tile_map::new(),
        tile_data_1: tile_data::new(tile_data::TileDataKind::Bottom),
        tile_data_2: tile_data::new(tile_data::TileDataKind::Top),
        cartridge_ram: device::not_implemented::NotImplemented {},
        internal_ram_bank_0: device::ram_bank::new(),
        internal_ram_bank_1: device::ram_bank::new(),
        echo_ram: device::not_implemented::NotImplemented {},
        object_attribute_memory: device::not_implemented::NotImplemented {},
        unusable_memory: device::not_implemented::NotImplemented {},
        hardware_io: device::hardware_io::new(),
        zero_page: device::zero_page::new(),
        interrupt_enable: device::interrupt::new_enabled(),
    }
}
