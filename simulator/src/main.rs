use hilton_simulate::Simulator;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;
use std::{env, process, thread};

const LCD_WIDTH: u32 = 84;
const LCD_HEIGHT: u32 = 48;
const PIXEL_SIZE: u32 = 4;

pub fn main() -> Result<(), String> {
    let program_path = env::args().skip(1).next().expect("Path");

    let mut sim = Simulator::atmega328p(program_path);
    let led = sim.led(sim.pins().pb0());
    sim.start();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Hilton", LCD_WIDTH * PIXEL_SIZE, LCD_HEIGHT * PIXEL_SIZE)
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

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(if led.is_on() { 255 } else { 64 }, 0, 0));
        canvas.fill_rect(Rect::new(10, 10, 50, 50)).unwrap();

        canvas.present();

        thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
}
