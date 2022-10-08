#![no_std]
#![no_main]

use atmega_hal::{clock::MHz16, delay::Delay, pins, Peripherals};
use avr_hal_generic::port::PinOps;
use avr_hal_generic::prelude::*;
use panic_halt as _;

mod buf_display;
mod lcd;

use self::buf_display::BufDisplay;
use self::lcd::Lcd10168;

#[atmega_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = pins!(dp);
    let mut delay = Delay::<MHz16>::new();

    let mut buf_display = BufDisplay::new(
        Lcd10168::builder()
            .reset(pins.pc1)
            .chip_enable(pins.pc2)
            .data_command(pins.pc3)
            .data_in(pins.pc4)
            .clock(pins.pc5)
            .build(),
    )
    .init();

    let mut ball = Ball::new(0, 0, 1, 1);

    loop {
        ball.render(&mut buf_display);
        ball.update();
        delay.delay_ms(16u16);
    }
}

const FIELD_WIDTH: usize = 84;
const FIELD_HEIGHT: usize = 48;

struct Ball {
    x: usize,
    y: usize,
    v_x: isize,
    v_y: isize,
}

impl Ball {
    const WIDTH: usize = 8;
    const HEIGHT: usize = 8;

    pub fn new(x: usize, y: usize, v_x: isize, v_y: isize) -> Self {
        Self { x, y, v_x, v_y }
    }

    pub fn update(&mut self) {
        self.x = (self.x as isize + self.v_x) as usize;
        self.y = (self.y as isize + self.v_y) as usize;

        if self.x == 0 || self.x == FIELD_WIDTH - Self::WIDTH - 1 {
            self.v_x *= -1;
        }

        if self.y == 0 || self.y == FIELD_HEIGHT - Self::HEIGHT - 1 {
            self.v_y *= -1;
        }
    }

    fn render<RST: PinOps, SCE: PinOps, DC: PinOps, DIN: PinOps, CLK: PinOps>(
        &self,
        buf_display: &mut BufDisplay<RST, SCE, DC, DIN, CLK>,
    ) {
        buf_display.clear();

        buf_display.draw_rect(self.x, self.y, Self::WIDTH, Self::HEIGHT, true);
        // Remove corner pixels
        buf_display.set_pixel(self.x, self.y, false);
        buf_display.set_pixel(self.x + Self::WIDTH - 1, self.y, false);
        buf_display.set_pixel(self.x, self.y + Self::HEIGHT - 1, false);
        buf_display.set_pixel(self.x + Self::WIDTH - 1, self.y + Self::HEIGHT - 1, false);

        buf_display.render()
    }
}
