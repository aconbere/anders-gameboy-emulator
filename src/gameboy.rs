use cpu;
use config;
use device;
use tile;
use framebuffer;
use gpu;
use instructions;
use mmu;
use palette;
use registers;

use device::boot_rom;
use device::cartridge;
use device::interrupt;

pub struct Gameboy {
    registers: registers::Registers,
    instructions: instructions::Instructions,
    cycle_count: u32,
    mmu: mmu::MMU,
    cpu: cpu::CPU,
    gpu: gpu::GPU,
}

impl Gameboy {
    pub fn next_frame(&mut self, framebuffer: &mut framebuffer::Framebuffer) {
        loop {
            let m = &mut self.mmu;
            let r = &mut self.registers;

            let cycles = self.cpu.tick(&self.instructions, r, m);

            if m.hardware_io
                .lcd_control_register
                .get_flag(device::hardware_io::LCDControlFlag::LCDDisplayEnable)
            {
                self.gpu.tick(m, cycles, framebuffer);
            }

            if r.get_interrupts_enabled() {
                let enabled = m.interrupt_enable.get_enabled_interrupts();
                let interrupts = m.hardware_io.interrupts.get_interrupts(enabled);

                for i in interrupts {
                    r.set_interrupts_enabled(false);
                    interrupt::handle_interrupt(r, m, i);
                }
            }

            self.cycle_count += cycles as u32;
            if self.cycle_count >= 70244 {
                self.cycle_count -= 70244;
                break;
            }

        }
    }

    /* Consider actually just using tile maps here!
     */

    pub fn render_tile(
        &self,
        framebuffer: &mut framebuffer::Framebuffer,
        tile: &tile::Tile,
        palette: &palette::Palette,
        tx: u8,
        ty: u8,
    ) {
        let tile_index_y = (ty as u32) * 8 * 160;
        let tile_index_x = (tx as u32) * 8;

        for y in 0..8 {
            let row = tile.get_row(y);
            for x in 0..8 {
                let pixel_index = (tile_index_y + tile_index_x) + (y as u32 * 160) + (x as u32);
                framebuffer[pixel_index as usize] = palette.map_shades(row[x as usize]);
            }
        }
    }

    pub fn render_tile_data(&self, framebuffer: &mut framebuffer::Framebuffer) {
        for ty in 0..18 {
            for tx in 0..20 {
                let i = (ty * 20) + tx;
                if i >= 192 {
                    return;
                }
                let tile = self.mmu.tile_data_1.get_tile(i);
                self.render_tile(framebuffer, &tile, &self.mmu.hardware_io.background_palette, tx, ty);
            }
        }
    }

    pub fn get_tile_maps(&self) -> (device::tile_map::TileMap, device::tile_map::TileMap) {
        (self.mmu.tile_map_1.clone(), self.mmu.tile_map_2.clone())
    }
}

pub fn new(config: &config::Config) -> Gameboy {
    let mut boot_rom = config.read_boot_rom().unwrap();
    let mut game_rom = config.read_game_rom().unwrap();

    Gameboy {
        registers: registers::new(),
        instructions: instructions::new(),
        cycle_count: 0,
        mmu: mmu::new(boot_rom::new(&mut boot_rom), cartridge::new(&mut game_rom)),
        cpu: cpu::new(config.clone()),
        gpu: gpu::new(),
    }
}
