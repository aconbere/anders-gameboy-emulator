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

pub struct LCDControlRegister {
    storage: u8
}

impl LCDControlRegister {
    pub fn set_flag(&mut self, f: LCDControlFlag, t:bool) {
        if t {
            self.storage = bytes::set_bit(self.storage, f.get_index());
        } else {
            self.storage = bytes::clear_bit(self.storage, f.get_index());
        }
    }

    pub fn get_flag(&self, f: LCDControlFlag) -> bool {
        let i = f.get_index();
        bytes::check_bit(self.storage, i)
    }

    pub fn get(&self) -> u8 {
        self.storage
    }

    pub fn set(&mut self, v:u8) {
        self.storage = v
    }
}

pub enum LCDModes {
    HBlank,
    VBlank,
    Searching,
    Transfer,
}

pub enum LCDStatusFlag {
    LYCoincidence,
    OAM,
    VBlank,
    HBlank,
    Coincidence,
    Mode(LCDModes),
}

impl LCDStatusFlag {
    pub fn get_index(&self) -> u8 {
        match self {
            LCDStatusFlag::LYCoincidence => 6, 
            LCDStatusFlag::OAM => 5, 
            LCDStatusFlag::VBlank => 4, 
            LCDStatusFlag::HBlank => 3, 
            LCDStatusFlag::Coincidence => 2, 
            LCDStatusFlag::Mode(LCDModes::HBlank) => 1, 
            LCDStatusFlag::Mode(LCDModes::VBlank) => 1, 
            LCDStatusFlag::Mode(LCDModes::Searching) => 0, 
            LCDStatusFlag::Mode(LCDModes::Transfer) => 0, 
        }
    }
}


pub struct LCDStatusRegister {
    storage: u8
}

impl LCDStatusRegister {
    pub fn set_flag(&mut self, f: LCDStatusFlag, t:bool) {
        if t {
            self.storage = bytes::set_bit(self.storage, f.get_index());
        } else {
            self.storage = bytes::clear_bit(self.storage, f.get_index());
        }
    }

    pub fn get_flag(&self, f: LCDStatusFlag) -> bool {
        let i = f.get_index();
        bytes::check_bit(self.storage, i)
    }

    pub fn get(&self) -> u8 {
        self.storage
    }

    pub fn set(&mut self, v:u8) {
        self.storage = v
    }
}

pub struct LCDLineCount {
    storage: u8
}

impl LCDLineCount {
    pub fn get(&self) -> u8 {
        self.storage
    }

    // all writes reset this register
    pub fn set(&mut self, _:u8) {
        self.storage = 0;
    }

    pub fn inc(&mut self) {
        let n = self.storage + 1;
        println!("new line!: {}", n);
        if n >= 154 {
            self.storage = 0;
        } else {
            self.storage = n;
        }
    }
}



pub struct HardwareIO {
    pub lcd_control_register: LCDControlRegister,
    pub lcd_status_register: LCDStatusRegister,
    pub lcd_line_count: LCDLineCount,
    pub storage: [u8; 128],
}

pub fn new() -> HardwareIO {
    HardwareIO {
        lcd_control_register: LCDControlRegister{storage: 0},
        lcd_status_register: LCDStatusRegister{storage: 0},
        lcd_line_count: LCDLineCount{storage: 0},
        storage: [0;128],
    }
}

impl Device for HardwareIO {
    fn get(&self, a:u16) -> u8 {
        match a {
            0x0040 => self.lcd_control_register.get(),
            0x0041 => self.lcd_status_register.get(),
            0x0044 => self.lcd_line_count.get(),
            _ => self.storage[a as usize],
        }
    }

    fn set(&mut self, a:u16, v:u8) {
        match a {
            0x0040 => self.lcd_control_register.set(v),
            0x0041 => self.lcd_status_register.set(v),
            0x0044 => panic!("lcd line count is R/O"),
            _ => self.storage[a as usize] = v,
        }
    }
}

impl HardwareIO {
    pub fn get_requested_interrupts(&self) -> u8 {
        self.storage[0x0F]
    }
}
