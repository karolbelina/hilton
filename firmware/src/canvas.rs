#![allow(dead_code)]

use crate::lcd::*;
use atmega_hal::port::Dynamic;
use avr_hal_generic::port::PinOps;

mod blit;
mod chunk;
mod color;
mod draw;
mod vec2;

pub use self::blit::*;
pub use self::chunk::*;
pub use self::color::*;
pub use self::draw::*;
pub use self::vec2::*;

pub struct Canvas<RST = Dynamic, SCE = Dynamic, DC = Dynamic, DIN = Dynamic, CLK = Dynamic> {
    lcd: Lcd10168<RST, SCE, DC, DIN, CLK>,
    buffer: [Chunk; Lcd10168::COLUMNS * Lcd10168::ROWS],
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
            ChipMode::Active,
            AddressingMode::Horizontal,
            InstructionSet::Extended,
        );
        // Safety: the instruction set has been set to extended on the line above
        unsafe {
            lcd.set_bias_voltage_coefficient(0);
            lcd.set_temperature_coefficient(0);
            lcd.set_operation_voltage(50);
        }
        lcd.function_set(
            ChipMode::Active,
            AddressingMode::Horizontal,
            InstructionSet::Basic,
        );
        // Safety: the instruction set has been set to basic on the line above
        unsafe { lcd.set_display_mode(DisplayMode::Normal) };

        Canvas {
            lcd,
            buffer: [Default::default(); Lcd10168::COLUMNS * Lcd10168::ROWS],
        }
    }
}

impl<RST, SCE, DC, DIN, CLK> Canvas<RST, SCE, DC, DIN, CLK> {
    pub fn chunk_at(&mut self, x: usize, y: usize) -> &mut Chunk {
        self.chunk_at_raw(x, y / 8)
    }

    pub fn chunk_at_raw(&mut self, column: usize, row: usize) -> &mut Chunk {
        &mut self.buffer[row * Lcd10168::COLUMNS + column]
    }

    pub fn clear(&mut self) {
        self.buffer.fill(Default::default());
    }
}

impl<RST: PinOps, SCE: PinOps, DC: PinOps, DIN: PinOps, CLK: PinOps>
    Canvas<RST, SCE, DC, DIN, CLK>
{
    pub fn write_frame(&mut self) {
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

impl<RST: PinOps, SCE: PinOps, DC: PinOps, DIN: PinOps, CLK: PinOps>
    Canvas<RST, SCE, DC, DIN, CLK>
{
    pub fn downgrade(
        self,
    ) -> Canvas<RST::Dynamic, SCE::Dynamic, DC::Dynamic, DIN::Dynamic, CLK::Dynamic> {
        Canvas {
            lcd: self.lcd.downgrade(),
            buffer: self.buffer,
        }
    }
}
