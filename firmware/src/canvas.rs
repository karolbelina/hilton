#![allow(dead_code)]

use crate::lcd::{self, Lcd10168};
use avr_hal_generic::port::PinOps;

mod chunk;
mod color;
mod draw;
mod vec2;

pub use self::chunk::*;
pub use self::color::*;
pub use self::vec2::*;

pub struct Canvas<RST, SCE, DC, DIN, CLK> {
    lcd: Lcd10168<RST, SCE, DC, DIN, CLK>,
    buffer: [Chunk; lcd::COLUMNS * lcd::ROWS],
}

impl<RST, SCE, DC, DIN, CLK> Canvas<RST, SCE, DC, DIN, CLK> {
    pub fn new_uninit(
        lcd: Lcd10168<RST, SCE, DC, DIN, CLK>,
    ) -> UninitCanvas<RST, SCE, DC, DIN, CLK> {
        UninitCanvas { lcd }
    }
}

pub struct UninitCanvas<RST, SCE, DC, DIN, CLK> {
    lcd: Lcd10168<RST, SCE, DC, DIN, CLK>,
}

impl<RST: PinOps, SCE: PinOps, DC: PinOps, DIN: PinOps, CLK: PinOps>
    UninitCanvas<RST, SCE, DC, DIN, CLK>
{
    pub fn init(self) -> Canvas<RST, SCE, DC, DIN, CLK> {
        let Self { mut lcd } = self;

        lcd.reset();
        lcd.function_set(
            lcd::ChipMode::Active,
            lcd::AddressingMode::Horizontal,
            lcd::InstructionSet::Extended,
        );
        // Safety: the instruction set has been set to extended on the line above
        unsafe {
            lcd.set_bias_voltage_coefficient(0);
            lcd.set_temperature_coefficient(0);
            lcd.set_operation_voltage(50);
        }
        lcd.function_set(
            lcd::ChipMode::Active,
            lcd::AddressingMode::Horizontal,
            lcd::InstructionSet::Basic,
        );
        // Safety: the instruction set has been set to basic on the line above
        unsafe { lcd.set_display_mode(lcd::DisplayMode::Normal) };

        Canvas {
            lcd,
            buffer: [Default::default(); lcd::COLUMNS * lcd::ROWS],
        }
    }
}

impl<RST, SCE, DC, DIN, CLK> Canvas<RST, SCE, DC, DIN, CLK> {
    #[inline]
    pub fn buffer_chunk(&mut self, Vec2 { x, y }: Vec2<usize>) -> &mut Chunk {
        &mut self.buffer[y / 8 * lcd::COLUMNS + x]
    }
}

impl<RST: PinOps, SCE: PinOps, DC: PinOps, DIN: PinOps, CLK: PinOps>
    Canvas<RST, SCE, DC, DIN, CLK>
{
    pub fn render(&mut self) {
        // Safety: we couldn't have acquired a `Canvas` without calling
        // `UninitCanvas::init` which sets the instruction set to basic
        unsafe {
            self.lcd.set_x_cursor(0);
            self.lcd.set_y_cursor(0);
        }

        for chunk in self.buffer {
            self.lcd.write_data(chunk.into());
        }
    }
}
