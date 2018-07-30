use device::Device;
use registers::Registers;
use registers::Registers16;
use mmu::MMU;
use instructions;

pub struct Interrupt {
    pub storage: u8
}

impl Interrupt {
    pub fn set(&mut self, v: u8) {
        self.storage = v;
    }

    pub fn get_interrupts(&self, enabled:u8) -> Vec<Flag> {
        let masked = enabled & self.storage;

        let mut flags = vec![];

        for i in 0..5 {
            if masked & (1 << i) != 0 {
                flags.push(FLAG_LOOKUP[i]);
            }
        }

        flags
    }
}

pub fn handle_interrupt(registers:&mut Registers, mmu:&mut MMU, f:Flag) {
    instructions::push_stack(registers, mmu, &Registers16::PC);
    println!("Handling Iterrupt: {:?}", f);

    match f {
        Flag::VBlank => {
            instructions::jump(registers, 0x0040);
        },
        Flag::LCDStat => {
            instructions::jump(registers, 0x0048);
        },
        Flag::Timer => {
            instructions::jump(registers, 0x0050);
        },
        Flag::Serial => {
            instructions::jump(registers, 0x0058);
        },
        Flag::Joypad => {
            instructions::jump(registers, 0x0060);
        },
    }

}

#[derive(Debug, Clone, Copy)]
pub enum Flag {
    VBlank,
    LCDStat,
    Timer,
    Serial,
    Joypad,
}

pub static FLAG_LOOKUP: [Flag; 5] = [
    Flag::VBlank,
    Flag::LCDStat,
    Flag::Timer,
    Flag::Serial,
    Flag::Joypad,
];

pub struct Enabled {
    f: u8,
}

impl Device for Enabled {
    fn get(&self, _: u16) -> u8 {
        self.f
    }

    fn set(&mut self, _: u16, v: u8) {
        self.f = v;
    }
}

impl Enabled {
    pub fn get_enabled_interrupts(&self) -> u8 {
        self.f
    }
}

pub fn new_enabled() -> Enabled {
    Enabled { f: 0 }
}

pub fn flags(enabled: u8, requested: u8) -> Vec<Flag> {
    let masked = enabled & requested;

    let mut flags = vec![];

    for i in 0..5 {
        if masked & (1 << i) != 0 {
            flags.push(FLAG_LOOKUP[i]);
        }
    }

    flags
}
