use device::Device;
use bytes;

pub enum LCDControlFlag {
    LCDDisplayEnable,
    WindowTileMapDisplaySelect,
    WindowDisplayEnable,
    BackgroundAndWindowTileDataSelect,
    BGTileMapDisplaySelect,
    ObjectSize,
    ObjectDisplayEnable,
    BackgroundDisplay,
}

impl LCDControlFlag {
    pub fn get_index(&self) -> u8 {
        match self {
            LCDControlFlag::LCDDisplayEnable => 7,
            LCDControlFlag::WindowTileMapDisplaySelect => 6,
            LCDControlFlag::WindowDisplayEnable => 5,
            LCDControlFlag::BackgroundAndWindowTileDataSelect => 4,
            LCDControlFlag::BGTileMapDisplaySelect => 3,
            LCDControlFlag::ObjectSize => 2,
            LCDControlFlag::ObjectDisplayEnable => 1,
            LCDControlFlag::BackgroundDisplay => 0,
        }
    }
}

pub enum LCDModes {
    HBlank,
    VBlank,
    Searching,
    Transfer
}

pub enum LCDStatusFlag {
    LYCoincidenceInterrupt,
    Mode2OAMInterrupt,
    Mode1VBlankInterrupt,
    Mode0HBlankInterrupt,
    CoincidenceFlag,
    Mode(LCDModes),
}

pub struct HardwareIO {
    pub storage: [u8; 128]
}

pub fn new() -> HardwareIO {
    HardwareIO {
        storage: [0;128],
    }
}

impl Device for HardwareIO {
    fn get(&self, a:u16) -> u8 {
        self.storage[a as usize]
    }

    fn set(&mut self, a:u16, v:u8) {
        self.storage[a as usize] = v;
    }
}

impl HardwareIO {
    pub fn set_lcd_control_flag(&mut self, f: LCDControlFlag, t:bool) {
        if t {
            self.storage[0x40] = bytes::set_bit(self.storage[0x40], f.get_index());
        } else {
            self.storage[0x40] = bytes::clear_bit(self.storage[0x40], f.get_index());
        }
    }

    pub fn get_lcd_control_flag(&self, f: LCDControlFlag) ->bool {
        let i = f.get_index();
        bytes::check_bit(self.storage[0x40], i)
    }

}
