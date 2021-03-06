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

/* Represents the gameboy device. Owns all the components needed to get it working.
 * However the actual loop is controlled by the display since SDL2 wants to own the
 * main game loop.
 *
 * The main function to understand is `next_frame` which will fill a framebuffer with
 * palette data to be rendered to screen by the display.
 * */
pub struct Gameboy {
    registers: registers::Registers,
    instructions: instructions::Instructions,
    cycle_count: u32,
    mmu: mmu::MMU,
    cpu: cpu::CPU,
    gpu: gpu::GPU,
}

impl Gameboy {
    pub fn get_pc(&self) -> u16 {
        self.registers.get16(&registers::Registers16::PC)
    }

    pub fn set_log_instructions(&mut self, state: bool) {
        self.cpu.set_log_instructions(state);
    }

    /* Executes an instruction (which returns the number of cycles it took) when the cycle count
     * exceeds 70244 it returns true to signal that a new frame is available. Gameboy frame timings
     * are based on cycles and 70244 is the number of frames a gameboy takes to render a full
     * frame.
     *
     * This function takes as its input a `framebuffer` which is an array of palette::Shades how to
     * render a shade is up to the display.
     *
     * Returns true if a frame is ready.
     */
    pub fn next_instruction(&mut self, framebuffer: &mut framebuffer::Framebuffer) -> bool {
        let cycles = self.cpu.tick(&self.instructions, &mut self.registers, &mut self.mmu);

        if self.mmu.hardware_io
            .lcd_control_register
            .get_flag(device::hardware_io::LCDControlFlag::LCDDisplayEnable)
        {
            self.gpu.tick(&mut self.mmu, cycles, framebuffer);
        }

        if self.registers.get_interrupts_enabled() {
            let enabled = self.mmu.interrupt_enable.get_enabled_interrupts();
            let interrupts = self.mmu.hardware_io.interrupts.get_interrupts(enabled);

            for i in interrupts {
                self.registers.set_interrupts_enabled(false);
                interrupt::handle_interrupt(&mut self.registers, &mut self.mmu, i);
            }
        }

        self.cycle_count += cycles as u32;

        if self.cycle_count >= 70244 {
            /* if we crossed 70244 we want to loop back around
             */
            self.cycle_count -= 70244;
            true
        } else {
            false
        }
    }

    /* Debug functions: */

    /* Looks through tile_data_1 and renders each of the tiles to screen. Useful to debug what tile
     * data is loaded.
     */
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

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufReader, BufRead};
    use std::path::Path;
    use registers;
    use config;
    use std::iter::Map;
    use palette;
    use framebuffer;

    #[derive(Debug, PartialEq)]
    struct State {
        pc: u16,
        af: u16,
        bc: u16,
        de: u16,
        hl: u16,
        sp: u16,
    }

    fn registers_to_state(r:&registers::Registers) -> State {
        State {
            pc: r.get16(&registers::Registers16::PC),
            af: r.get16(&registers::Registers16::AF),
            bc: r.get16(&registers::Registers16::BC),
            de: r.get16(&registers::Registers16::DE),
            hl: r.get16(&registers::Registers16::HL),
            sp: r.get16(&registers::Registers16::SP),
        }
    }

    fn parse_u16_hex(s:&str) -> u16 {
        u16::from_str_radix(s, 16).unwrap()
    }

    fn parse_state_string(s:String) -> State {
        State {
            pc: parse_u16_hex(&s[0..4]),
            af: parse_u16_hex(&s[4..8]),
            bc: parse_u16_hex(&s[8..12]),
            de: parse_u16_hex(&s[12..16]),
            hl: parse_u16_hex(&s[16..20]),
            sp: parse_u16_hex(&s[20..24]),
        }
    }


    fn read_state_file(filename:&Path) -> Vec<State> {
        let f = File::open(filename).unwrap();

        BufReader::new(f).lines().map(|l|
            parse_state_string(l.unwrap())
        ).collect::<Vec<State>>()
    }

    #[test]
    fn boot_rom_states_are_exact() {
        let states = read_state_file(Path::new("./tests/state_files/boot_rom_states"));

        let config = config::Config {
            boot_rom: String::from("../gb_test_roms/DMG_ROM.bin"),
            game_rom: String::from("./tests/cpu_instrs_01_special.gb"),
            debug: config::debug_default(),
        };

        let mut gameboy = super::new(&config);
        let mut framebuffer: framebuffer::Framebuffer = [palette::Shade::White; 23040];

        gameboy.next_instruction(&mut framebuffer);

        for s in states {
            let register_state = registers_to_state(&gameboy.registers);
            assert_eq!(s, register_state);
            gameboy.next_instruction(&mut framebuffer);
        }
    }

    #[test]
    fn cpu_instr_06_ld_r_r() {
        let states = read_state_file(Path::new("./tests/state_files/06_ld_r_r.test"));

        let config = config::Config {
            boot_rom: String::from("../gb_test_roms/DMG_ROM.bin"),
            game_rom: String::from("./tests/cpu_instrs_06_ld_r_r.gb"),
            debug: config::debug_default(),
        };

        let mut gameboy = super::new(&config);
        let mut framebuffer: framebuffer::Framebuffer = [palette::Shade::White; 23040];

        while gameboy.get_pc() < 0x0100 {
            gameboy.next_instruction(&mut framebuffer);
        }

        for s in states {
            println!("State: {:?}", s);
            let register_state = registers_to_state(&gameboy.registers);
            assert_eq!(s, register_state);
            gameboy.next_instruction(&mut framebuffer);
        }
    }
}

