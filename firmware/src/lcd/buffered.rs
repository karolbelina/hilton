use super::{AddressingMode, ChipMode, DisplayMode, InstructionSet, Lcd10168};
use crate::canvas::{Canvas, Color};
use atmega_hal::port::Dynamic;
use avr_hal_generic::port::PinOps;

mod chunk;

pub use self::chunk::*;

pub struct BufLcd10168<RST = Dynamic, SCE = Dynamic, DC = Dynamic, DIN = Dynamic, CLK = Dynamic> {
    lcd: Lcd10168<RST, SCE, DC, DIN, CLK>,
    buffer: [Chunk; Lcd10168::COLUMNS * Lcd10168::ROWS],
}

impl<RST: PinOps, SCE: PinOps, DC: PinOps, DIN: PinOps, CLK: PinOps>
    Lcd10168<RST, SCE, DC, DIN, CLK>
{
    pub fn into_buffered(self) -> UninitBufLcd10168<RST, SCE, DC, DIN, CLK> {
        UninitBufLcd10168 { lcd: self }
    }
}

pub struct UninitBufLcd10168<RST, SCE, DC, DIN, CLK> {
    lcd: Lcd10168<RST, SCE, DC, DIN, CLK>,
}

impl<RST: PinOps, SCE: PinOps, DC: PinOps, DIN: PinOps, CLK: PinOps>
    UninitBufLcd10168<RST, SCE, DC, DIN, CLK>
{
    pub fn init(self) -> BufLcd10168<RST, SCE, DC, DIN, CLK> {
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

        BufLcd10168 {
            lcd,
            buffer: [Default::default(); Lcd10168::COLUMNS * Lcd10168::ROWS],
        }
    }
}

impl<RST, SCE, DC, DIN, CLK> BufLcd10168<RST, SCE, DC, DIN, CLK> {
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

impl<RST: PinOps, SCE: PinOps, DC: PinOps, DIN: PinOps, CLK: PinOps> Canvas
    for BufLcd10168<RST, SCE, DC, DIN, CLK>
{
    fn blit_pixel(&mut self, x: isize, y: isize, color: Color) {
        let x = match usize::try_from(x) {
            Ok(x) => x,
            Err(_) => return,
        };
        let y = match usize::try_from(y) {
            Ok(y) => y,
            Err(_) => return,
        };

        let chunk = self.chunk_at(x, y);
        let mask = Chunk::bit(y % 8);

        match color {
            Color::On => *chunk |= mask,
            Color::Off => *chunk &= !mask,
        }
    }
}

impl<RST: PinOps, SCE: PinOps, DC: PinOps, DIN: PinOps, CLK: PinOps>
    BufLcd10168<RST, SCE, DC, DIN, CLK>
{
    pub fn write_frame(&mut self) {
        // Safety: we couldn't have acquired a `BufLcd10168` without calling
        // `UninitBufLcd10168::init` which sets the instruction set to basic
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
    BufLcd10168<RST, SCE, DC, DIN, CLK>
{
    pub fn downgrade(
        self,
    ) -> BufLcd10168<RST::Dynamic, SCE::Dynamic, DC::Dynamic, DIN::Dynamic, CLK::Dynamic> {
        BufLcd10168 {
            lcd: self.lcd.downgrade(),
            buffer: self.buffer,
        }
    }
}
