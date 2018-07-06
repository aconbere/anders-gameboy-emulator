use sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use gameboy;
use palette;
use framebuffer;

pub fn start(gameboy:&mut gameboy::Gameboy) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let scale:u32 = 4;

    let window = video_subsystem.window("rust-sdl2 demo: Cursor", 160 * scale, 144 * scale)
      .position_centered()
      .build()
      .unwrap();

    let mut canvas = window.into_canvas().software().build().unwrap();

    canvas.clear();
    canvas.present();

    let mut events = sdl_context.event_pump().unwrap();
    let mut framebuffer:framebuffer::Framebuffer = [palette::Shade::White;23040];

    'mainloop: loop {
        gameboy.next_frame(&mut framebuffer);
        canvas.clear();
        canvas.present();

        for x in 0..160 {
            for y in 0..144 {
                let i = (y * 160) + x;
                match framebuffer[i] {
                    palette::Shade::White =>
                        canvas.set_draw_color(Color::RGBA(255, 255, 255, 255)),
                    palette::Shade::LightGrey =>
                        canvas.set_draw_color(Color::RGBA(211, 211, 211, 255)),
                    palette::Shade::DarkGrey =>
                        canvas.set_draw_color(Color::RGBA(169, 169, 169, 255)),
                    palette::Shade::Black =>
                        canvas.set_draw_color(Color::RGBA(0, 0, 0, 255)),
                }
                canvas.fill_rect(
                    Rect::new((x as u32 * scale) as i32,
                              (y as u32 * scale) as i32,
                              scale,
                              scale)
                    ).unwrap();
            }
        }
        canvas.present();
        for event in events.poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyDown {keycode: Option::Some(Keycode::Escape), ..} =>
                    break 'mainloop,
                _ => {}
            }
        }
    }
}
