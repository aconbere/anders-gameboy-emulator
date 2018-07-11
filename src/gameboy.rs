use cpu;
use device;
use tile;
use framebuffer;
use gpu;
use instructions;
use mmu;
use palette;
use registers;

pub struct Gameboy {
    registers: registers::Registers,
    instructions: instructions::Instructions,
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
                let requested = m.hardware_io.get_requested_interrupts();
                let enabled = m.interrupt_enable.get_enabled_interrupts();
                for i in device::interrupt::flags(enabled, requested) {
                    println!("Saw interrupt: {:?}", i);
                }
            }

            if self.gpu.new_frame_available() {
                break;
            }
        }
    }

    pub fn render_tile(
        &self,
        framebuffer: &mut framebuffer::Framebuffer,
        tile: &tile::Tile,
        palette: &palette::Palette,
        tx: u8,
        ty: u8,
    ) {
        for y in 0..8 {
            for x in 0..8 {
                let i = (((ty + y) as u16 * 160) + (tx + x) as u16) as usize;

                framebuffer[i] = palette.map_shades(tile.get_pixel(x, y));
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
                self.render_tile(framebuffer, &tile, &self.mmu.hardware_io.background_palette, tx * 8, ty * 8);
            }
        }
    }

    pub fn get_tile_maps(&self) -> (device::tile_map::TileMap, device::tile_map::TileMap) {
        (self.mmu.tile_map_1.clone(), self.mmu.tile_map_2.clone())
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
