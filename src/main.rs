extern crate sdl2;

mod bytes;
mod registers;
mod instructions;
mod cpu;
mod gpu;
mod mmu;
mod device;
mod display;

fn main() {
    let mut registers = registers::new();
    let instructions = instructions::new();

    let mut mmu = mmu::new();

    let mut cpu = cpu::new();
    let mut gpu = gpu::new();

    loop {
        let m = &mut mmu;
        let r = &mut registers;

        let cycles = cpu.tick(&instructions, r, m);

        if m.hardware_io.get_lcd_control_flag(device::hardware_io::LCDControlFlag::LCDDisplayEnable) {
            gpu.tick(m, cycles);
        }

        if r.get_interrupts_enabled() {
            let requested = m.hardware_io.get_requested_interrupts();
            let enabled = m.interrupt_enable.get_enabled_interrupts();
            for i in device::interrupt::flags(enabled, requested) {
                println!("Saw interrupt: {:?}", i);
            }
        }
    }
}
