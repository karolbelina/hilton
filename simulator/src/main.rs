use hilton_simulate::Simulator;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;
use std::{env, process, thread};

const PIXEL_WIDTH: u32 = 5;
const PIXEL_HEIGHT: u32 = 6;
const HORIZONTAL_PADDING: u32 = 16 * PIXEL_WIDTH;
const VERTICAL_PADDING: u32 = 8 * PIXEL_HEIGHT;

pub fn main() -> Result<(), String> {
    let program_path = env::args().skip(1).next().expect("Path");

    let mut sim = Simulator::atmega328p(program_path);
    // let led = sim.led(sim.pins().pb0());
    let lcd = sim.lcd_10168(
        sim.pins().pc1(),
        sim.pins().pc2(),
        sim.pins().pc3(),
        sim.pins().pc4(),
        sim.pins().pc5(),
    );
    sim.start();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let (display_width, display_height) = lcd.display_size();
    let window = video_subsystem
        .window(
            "Hilton",
            display_width as u32 * PIXEL_WIDTH + HORIZONTAL_PADDING * 2,
            display_height as u32 * PIXEL_HEIGHT + VERTICAL_PADDING * 2,
        )
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => process::exit(0),

                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        for y in 0..display_height {
            for x in 0..display_width {
                if lcd.is_pixel_on_at(x, y) {
                    let x = x * PIXEL_WIDTH + HORIZONTAL_PADDING;
                    let y = y * PIXEL_HEIGHT + VERTICAL_PADDING;
                    canvas.set_draw_color(Color::RGB(64, 64, 64));
                    canvas
                        .fill_rect(Rect::new(x as i32, y as i32, PIXEL_WIDTH, PIXEL_HEIGHT))
                        .unwrap();
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    canvas
                        .fill_rect(Rect::new(
                            x as i32,
                            y as i32,
                            PIXEL_WIDTH - 1,
                            PIXEL_HEIGHT - 1,
                        ))
                        .unwrap();
                }
            }
        }

        // canvas.set_draw_color(Color::RGB(if led.is_on() { 255 } else { 64 }, 0, 0));
        // canvas.fill_rect(Rect::new(10, 10, 50, 50)).unwrap();

        canvas.present();

        thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
}
