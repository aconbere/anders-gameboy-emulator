use sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::path::Path;

use config;
use framebuffer;
use gameboy;
use palette;

struct RateLimiter {
    fps: u32,
    last_ticks: u32,
}

fn new_rate_limiter() -> RateLimiter {
    RateLimiter {
        fps: 60,
        last_ticks: 0,
    }
}

impl RateLimiter {
    pub fn limit(&mut self, timer: &mut sdl2::TimerSubsystem) {
        let ticks = timer.ticks();
        let adjusted_ticks = ticks - self.last_ticks;
        if adjusted_ticks < 1000 / self.fps {
            timer.delay((1000 / self.fps) - adjusted_ticks);
        }
        self.last_ticks = ticks;
    }
}

enum State {
    Running,
    Paused,
    TileData,
    TileMap,
}

pub struct Display {
    frame_count: u32,
    state: State,
    config: config::Config,
}

pub fn new(config: &config::Config) -> Display {
    Display {
        frame_count: 0,
        state: State::Running,
        config: config.clone(),
    }
}

impl Display {
    fn toggle_paused(&mut self) {
        match self.state {
            State::Running => self.state = State::Paused,
            _ => self.state = State::Running,
        }
    }

    fn draw(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        framebuffer: &framebuffer::Framebuffer,
        scale: u32,
    ) {
        for x in 0..160 {
            for y in 0..144 {
                let i = (y * 160) + x;
                match framebuffer[i] {
                    palette::Shade::White => canvas.set_draw_color(Color::RGBA(255, 255, 255, 255)),
                    palette::Shade::LightGrey => {
                        canvas.set_draw_color(Color::RGBA(211, 211, 211, 255))
                    }
                    palette::Shade::DarkGrey => {
                        canvas.set_draw_color(Color::RGBA(169, 169, 169, 255))
                    }
                    palette::Shade::Black => canvas.set_draw_color(Color::RGBA(0, 0, 0, 255)),
                }
                canvas
                    .fill_rect(Rect::new(
                        (x as u32 * scale) as i32,
                        (y as u32 * scale) as i32,
                        scale,
                        scale,
                    ))
                    .unwrap();
            }
        }
    }

    pub fn start(&mut self, gameboy: &mut gameboy::Gameboy) {
        let sdl_context = sdl2::init().unwrap();
        let ttf_context = sdl2::ttf::init().unwrap();
        let mut timer = sdl_context.timer().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let scale: u32 = 4;

        let window = video_subsystem
            .window("Gameboy", 160 * scale, 144 * scale)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().software().build().unwrap();

        let texture_creator = canvas.texture_creator();
        let font = ttf_context
            .load_font(
                Path::new("./fonts/Roboto-Black.ttf"),
                128,
            )
            .unwrap();
        let target = Rect::new(10, 10, 80, 20);

        canvas.clear();
        canvas.present();

        let mut events = sdl_context.event_pump().unwrap();
        let mut framebuffer: framebuffer::Framebuffer = [palette::Shade::White; 23040];
        let mut rendered_tile_data = false;
        let mut rendered_tile_map = false;
        let mut rate_limiter = new_rate_limiter();

        let paused_text = {
            let surface = font.render(&format!("Paused"))
                .blended(Color::RGBA(255, 0, 0, 255))
                .unwrap();
            texture_creator
                .create_texture_from_surface(&surface)
                .unwrap()
        };

        'mainloop: loop {
            match self.state {
                State::Running => {
                    gameboy.next_frame(&mut framebuffer);
                    self.draw(&mut canvas, &framebuffer, scale);
                    if self.config.debug.frame_count {
                        let surface = font.render(&format!("F:{}", self.frame_count))
                            .blended(Color::RGBA(255, 0, 0, 255))
                            .unwrap();
                        let texture = texture_creator
                            .create_texture_from_surface(&surface)
                            .unwrap();
                        canvas.copy(&texture, None, Some(target)).unwrap();
                    }
                    canvas.present();
                    self.frame_count += 1;
                    match self.config.debug.break_point_frame {
                        Some(bk) => {
                            if bk == self.frame_count {
                                self.state = State::Paused
                            }
                        },
                        _ => {}
                    }
                }
                State::TileData => {
                    if !rendered_tile_data {
                        gameboy.render_tile_data(&mut framebuffer);
                        self.draw(&mut canvas, &framebuffer, scale);
                        let surface = font.render(&format!("Tile Data"))
                            .blended(Color::RGBA(255, 0, 0, 255))
                            .unwrap();
                        let texture = texture_creator
                            .create_texture_from_surface(&surface)
                            .unwrap();
                        canvas.copy(&texture, None, Some(target)).unwrap();
                        rendered_tile_data = true;
                        canvas.present();
                    }
                }
                State::TileMap => {
                    if !rendered_tile_map {
                        canvas.clear();
                        let (tm1, tm2) = gameboy.get_tile_maps();
                        println!("TileMap1:\n{:?}", tm1);
                        println!("TileMap2:\n{:?}", tm2);
                        let surface = font.render(&format!("Tile Map"))
                            .blended(Color::RGBA(255, 0, 0, 255))
                            .unwrap();
                        let texture = texture_creator
                            .create_texture_from_surface(&surface)
                            .unwrap();
                        canvas.copy(&texture, None, Some(target)).unwrap();
                        canvas.present();
                        rendered_tile_map = true;
                    }
                }
                State::Paused => {
                    canvas.clear();
                    self.draw(&mut canvas, &framebuffer, scale);
                    canvas.copy(&paused_text, None, Some(target)).unwrap();
                    canvas.present();
                }
            }

            rate_limiter.limit(&mut timer);

            for event in events.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Option::Some(Keycode::Escape),
                        ..
                    } => break 'mainloop,
                    Event::KeyDown {
                        keycode: Option::Some(Keycode::Space),
                        ..
                    } => self.toggle_paused(),
                    Event::KeyDown {
                        keycode: Option::Some(Keycode::D),
                        ..
                    } => self.state = State::TileData,
                    Event::KeyDown {
                        keycode: Option::Some(Keycode::M),
                        ..
                    } => self.state = State::TileMap,
                    Event::KeyDown {
                        keycode: Option::Some(Keycode::Right),
                        ..
                    } => {
                        rendered_tile_data = false;
                        rendered_tile_map = false;
                    }
                    _ => {}
                }
            }
        }
    }
}
