use device::Device;
use device::boot_rom;

enum States {
    Booting,
    Running,
}

pub struct RestartAndInterrupt {
    boot_rom: boot_rom::BootRom,
    storage: [u8; 256],
    state: States, 
}

impl Device for RestartAndInterrupt {
    fn get(&self, a:u16) -> u8 {
        match self.state {
            States::Booting => self.boot_rom.get(a),
            States::Running => self.storage[a as usize]
        }
    }

    fn set(&mut self, a:u16, v:u8) {
        match self.state {
            States::Booting => panic!("can't mangle the bootrom!"),
            States::Running => self.storage[a as usize] = v,
        }
    }
}

pub fn new(boot_rom:boot_rom::BootRom) -> RestartAndInterrupt {
    RestartAndInterrupt {
        storage: [0;256],
        boot_rom: boot_rom,
        state: States::Booting,
    }
}
