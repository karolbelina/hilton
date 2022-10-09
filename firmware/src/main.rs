#![no_std]
#![no_main]

use atmega_hal::{clock::MHz16, delay::Delay, pins, Peripherals};
use avr_hal_generic::port::PinOps;
use avr_hal_generic::prelude::*;

mod canvas;
mod lcd;
mod panic;

use self::canvas::*;
use self::lcd::*;

#[atmega_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = pins!(dp);
    let mut delay = Delay::<MHz16>::new();

    let mut canvas = Canvas::new_uninit(
        Lcd10168::builder()
            .reset(pins.pc1)
            .chip_enable(pins.pc2)
            .data_command(pins.pc3)
            .data_in(pins.pc4)
            .clock(pins.pc5)
            .build(),
    )
    .init();

    let mut ball = Ball::new(FIELD_SIZE / 2, Vec2::new(1, 1));

    loop {
        ball.render(&mut canvas);
        ball.update();
        delay.delay_ms(16u16);
    }
}

const FIELD_SIZE: Vec2<isize> = Vec2::new(84, 48);

struct Ball {
    position: Vec2<isize>,
    velocity: Vec2<isize>,
}

impl Ball {
    const SIZE: Vec2<isize> = Vec2::new(8, 8);

    pub fn new(position: Vec2<isize>, velocity: Vec2<isize>) -> Self {
        Self { position, velocity }
    }

    pub fn update(&mut self) {
        self.position += self.velocity;

        if self.position.x == 0 || self.position.x + Ball::SIZE.x == FIELD_SIZE.x {
            self.velocity.x *= -1;
        }

        if self.position.y == 0 || self.position.y + Ball::SIZE.y == FIELD_SIZE.y {
            self.velocity.y *= -1;
        }
    }

    fn render<RST: PinOps, SCE: PinOps, DC: PinOps, DIN: PinOps, CLK: PinOps>(
        &self,
        canvas: &mut Canvas<RST, SCE, DC, DIN, CLK>,
    ) {
        const CORNERS: [Vec2<isize>; 4] = [
            Vec2::new(0, 0),
            Vec2::new(Ball::SIZE.x - 1, 0),
            Vec2::new(0, Ball::SIZE.y - 1),
            Vec2::new(Ball::SIZE.x - 1, Ball::SIZE.y - 1),
        ];

        canvas.clear();

        canvas.draw_rect(self.position, Self::SIZE, Color::On);
        // Remove corner pixels
        for corner in CORNERS {
            canvas.set_pixel_color(self.position + corner, Color::Off);
        }

        canvas.render()
    }
}
