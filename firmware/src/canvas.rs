use crate::lcd::{self, Lcd10168};
use avr_hal_generic::port::PinOps;

pub struct Canvas<RST, SCE, DC, DIN, CLK> {
    lcd: Lcd10168<RST, SCE, DC, DIN, CLK>,
    buffer: [u8; lcd::COLUMNS * lcd::ROWS],
}

impl<RST, SCE, DC, DIN, CLK> Canvas<RST, SCE, DC, DIN, CLK> {
    pub fn new(lcd: Lcd10168<RST, SCE, DC, DIN, CLK>) -> UninitCanvas<RST, SCE, DC, DIN, CLK> {
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
            buffer: [0; lcd::COLUMNS * lcd::ROWS],
        }
    }
}

impl<RST, SCE, DC, DIN, CLK> Canvas<RST, SCE, DC, DIN, CLK> {
    #[inline]
    pub fn buffer_slice(&mut self, x: usize, y: usize) -> &mut u8 {
        &mut self.buffer[y / 8 * lcd::COLUMNS + x]
    }

    #[inline]
    fn slice_pixel_index(y: usize) -> usize {
        y % 8
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: bool) {
        let slice = self.buffer_slice(x, y);
        let i = Self::slice_pixel_index(y);

        if color {
            *slice |= 1 << i;
        } else {
            *slice &= !(1 << i);
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(0);
    }

    pub fn draw_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: bool) {
        for dy in 0..height {
            for dx in 0..width {
                self.set_pixel(x + dx, y + dy, color);
            }
        }
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

        for slice in self.buffer {
            self.lcd.write_data(slice);
        }
    }
}
