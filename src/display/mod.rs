use sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::ttf;

use config;
use framebuffer;
use gameboy;
use palette;
// use repl;

mod rate_limiter;
mod canvas;

enum RunningState {
    Frame,
    Instruction,
    Continuous,
}

enum State {
    Running(RunningState),
    Paused,
    TileData,
    TileMap,
}

/* Display 
 */
pub struct Display {
    frame_count: u32,
    state: State,
    config: config::Config,
    canvas: Canvas<Window>,
    scale: u32,
    sdl_context: sdl2::Sdl,
    timer: sdl2::TimerSubsystem,
}

pub fn new<'a, 'b>(config: &config::Config) -> Display {
    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();

    let scale = 4;

    let window = video_subsystem
        .window("Gameboy", 160 * scale, 144 * scale)
        .position_centered()
        .build()
        .unwrap();

    let canvas = window.into_canvas().software().build().unwrap();

    let timer = sdl_context.timer().unwrap();

    Display {
        frame_count: 0,
        state: State::Running(RunningState::Continuous),
        config: config.clone(),
        canvas: canvas,
        scale: scale,
        sdl_context: sdl_context,
        timer: timer,
    }
}

impl Display {
    fn handle_event(&mut self, event: Event) {
        match event {
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
                self.state = State::Running(RunningState::Frame)
            },
            Event::KeyDown {
                keycode: Option::Some(Keycode::Down),
                ..
            } => {
                self.state = State::Running(RunningState::Instruction)
            },
            _ => {}
        }
    }

    fn break_at_frame(&mut self) {
        match self.config.debug.break_point_frame {
            Some(bk) => {
                if bk == self.frame_count {
                    self.state = State::Paused
                }
            },
            _ => {}
        }
    }

    fn toggle_paused(&mut self) {
        match self.state {
            State::Paused => self.state = State::Running(RunningState::Continuous),
            _ => self.state = State::Paused,
        }
    }

    pub fn start(&mut self, gameboy: &mut gameboy::Gameboy) {
        let mut framebuffer: framebuffer::Framebuffer = [palette::Shade::White; 23040];

        let mut rate_limiter = rate_limiter::new(60);

        let ttf_context = ttf::init().unwrap();
        let debug_text = canvas::DebugText::new(&ttf_context);

        'mainloop: loop {
            match self.state {
                State::Running(RunningState::Instruction) => {
                    if gameboy.next_instruction(&mut framebuffer) {
                        self.frame_count += 1;
                    }
                    canvas::draw(&mut self.canvas, &framebuffer, self.scale);

                    if self.config.debug.frame_count {
                        debug_text.draw(&mut self.canvas, &format!("F:{}", self.frame_count))
                    }
                    self.canvas.present();
                    self.state = State::Paused;
                }
                State::Running(RunningState::Frame) => {
                    'frameloop1: loop {
                        if gameboy.next_instruction(&mut framebuffer) {
                            self.frame_count += 1;
                            break 'frameloop1
                        }
                    }

                    canvas::draw(&mut self.canvas, &framebuffer, self.scale);

                    if self.config.debug.frame_count {
                        debug_text.draw(&mut self.canvas, &format!("F:{}", self.frame_count))
                    }
                    self.canvas.present();
                    self.state = State::Paused;
                }
                State::Running(RunningState::Continuous) => {
                    'frameloop: loop {
                        if gameboy.next_instruction(&mut framebuffer) {
                            self.frame_count += 1;
                            break 'frameloop
                        }
                    }

                    canvas::draw(&mut self.canvas, &framebuffer, self.scale);

                    if self.config.debug.frame_count {
                        debug_text.draw(&mut self.canvas, &format!("F:{}", self.frame_count))
                    }
                    self.canvas.present();
                    self.break_at_frame();
                }
                State::TileData => {
                    gameboy.render_tile_data(&mut framebuffer);
                    canvas::draw(&mut self.canvas, &framebuffer, self.scale);
                    debug_text.draw(&mut self.canvas, "Tile Data");
                    self.canvas.present();
                }
                State::TileMap => {
                    self.canvas.clear();
                    let (tm1, tm2) = gameboy.get_tile_maps();
                    println!("TileMap1:\n{:?}", tm1);
                    println!("TileMap2:\n{:?}", tm2);
                    debug_text.draw(&mut self.canvas, "Tile Map");
                    self.canvas.present();
                }
                State::Paused => {
                    self.canvas.clear();
                    canvas::draw(&mut self.canvas, &framebuffer, self.scale);
                    debug_text.draw(&mut self.canvas, "Paused");
                    self.canvas.present();
                }
            }

            rate_limiter.limit(&mut self.timer);

            let mut events = self.sdl_context.event_pump().unwrap();

            for event in events.poll_iter() {
                match event {
                    Event::Quit { .. } | Event::KeyDown { keycode: Option::Some(Keycode::Escape), ..  } => {
                        break 'mainloop
                    },
                    _ => self.handle_event(event)
                }
            }
        }
    }
}
