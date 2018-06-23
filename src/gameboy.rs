use registers;
use mmu;
use cpu;
use gpu;
use instructions;
use device;

use std::{thread, time};

pub struct Gameboy {
    registers: registers::Registers,
    instructions: instructions::Instructions,
    mmu: mmu::MMU,
    cpu: cpu::CPU,
    gpu: gpu::GPU,
}

impl Gameboy {
    pub fn next_frame(&mut self) {
        loop {
            let m = &mut self.mmu;
            let r = &mut self.registers;

            let cycles = self.cpu.tick(&self.instructions, r, m);

            if m.hardware_io.lcd_control_register.get_flag(device::hardware_io::LCDControlFlag::LCDDisplayEnable) {
                self.gpu.tick(m, cycles);
            }

            if r.get_interrupts_enabled() {
                let requested = m.hardware_io.get_requested_interrupts();
                let enabled = m.interrupt_enable.get_enabled_interrupts();
                for i in device::interrupt::flags(enabled, requested) {
                    println!("Saw interrupt: {:?}", i);
                }
            }
            if self.gpu.new_frame_available() {
                println!("new frame available: sleeping");
                thread::sleep(time::Duration::from_secs(1));
                break;
            }
        }
    }
}

pub fn new() -> Gameboy {
    Gameboy {
        registers: registers::new(),
        instructions: instructions::new(),
        mmu: mmu::new(),
        cpu: cpu::new(),
        gpu: gpu::new(),
    }
}
