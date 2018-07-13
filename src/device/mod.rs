pub mod cartridge;
pub mod hardware_io;
pub mod interrupt;
pub mod not_implemented;
pub mod ram_bank;
pub mod tile_data;
pub mod tile_map;
pub mod zero_page;
pub mod boot_rom;

#[derive(Debug)]
pub enum Kind {
    RestartAndInterrupt,
    CartridgeHeader,
    CartridgeROMBank0,
    CartridgeROMBank1,
    TileMap1,
    TileMap2,
    TileData1,
    TileData2,
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

pub fn get_kind(address: u16) -> Kind {
    match address {
        0x0000...0x00FF => Kind::RestartAndInterrupt,

        // Cartridge
        0x00FF...0x014F => Kind::CartridgeHeader,
        0x014F...0x3FFF => Kind::CartridgeROMBank0,
        0x3FFF...0x7FFF => Kind::CartridgeROMBank1,

        // video ram
        // Because TileData is actually segmented into three sections with splits in between
        // the two sections this probably needs to be a single device.
        0x8000...0x8FFF => Kind::TileData1,
        0x8800...0x97FF => Kind::TileData2,
        0x9800...0x9BFF => Kind::TileMap1,
        0x9C00...0x9FFF => Kind::TileMap2,

        0xA000...0xBFFF => Kind::CartridgeRAM,

        // Internal Ram
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
    fn get(&self, a: u16) -> u8;
    fn set(&mut self, a: u16, v: u8);
}
