#![allow(dead_code)]

use atmega_hal::{clock::MHz16, delay::Delay};
use avr_hal_generic::port::{mode::Output, Pin, PinOps};
use avr_hal_generic::prelude::*;

mod builder;
mod commands;

pub use self::commands::*;

const COLUMNS: u32 = 84;
const ROWS: u32 = 6;

/// Provides a basic interface for the LCD-10168 chip. See the
/// [full specification](https://www.sparkfun.com/datasheets/LCD/Monochrome/Nokia5110.pdf)
/// of the chip for more details.
pub struct Lcd10168<RST, SCE, DC, DIN, CLK> {
    rst: Pin<Output, RST>,
    sce: Pin<Output, SCE>,
    dc: Pin<Output, DC>,
    din: Pin<Output, DIN>,
    clk: Pin<Output, CLK>,

    delay: Delay<MHz16>,
}

impl<RST, SCE, DC, DIN, CLK> Lcd10168<RST, SCE, DC, DIN, CLK> {
    fn new(
        rst: Pin<Output, RST>,
        sce: Pin<Output, SCE>,
        dc: Pin<Output, DC>,
        din: Pin<Output, DIN>,
        clk: Pin<Output, CLK>,
    ) -> Self {
        Self {
            rst,
            sce,
            dc,
            din,
            clk,
            delay: Delay::<MHz16>::new(),
        }
    }
}

macro_rules! command {
    ($type:expr => $($arg:expr),*) => {
        (1 << $type)$(| $arg as u8)*
    };
}

impl<RST, SCE, DC, DIN, CLK> Lcd10168<RST, SCE, DC, DIN, CLK>
where
    RST: PinOps,
    SCE: PinOps,
    DC: PinOps,
    DIN: PinOps,
    CLK: PinOps,
{
    /// Executes a reset sequence of the chip.
    pub fn reset(&mut self) {
        self.rst.set_high();
        self.sce.set_high();
        self.delay.delay_ms(10u16);
        self.rst.set_low();
        self.delay.delay_ms(70u16);
        self.rst.set_high();
        self.sce.set_low();
    }

    /// Sends a command for controlling the chip mode (active/power down), the addressing
    /// mode (horizontal/vertical), and the instruction set (basic/extended) simultaneously.
    pub fn function_set(&mut self, chip_mode: ChipMode, addressing_mode: AddressingMode, instruction_set: InstructionSet) {
        self.write_command(command!(5 => chip_mode, addressing_mode, instruction_set));
    }

    /// Sends a command for setting the X cursor of the display, or rather the X address of
    /// the display RAM.
    /// 
    /// # Safety
    /// 
    /// The instruction set of the LCD must be set to [`Basic`](InstructionSet::Basic).
    /// `x` must be a value less than 84.
    pub unsafe fn set_x_cursor(&mut self, x: u8) {
        debug_assert!(x < COLUMNS as u8);
        self.write_command(command!(7 => x));
    }

    /// Sends a command for setting the Y cursor of the display, or rather the Y address of
    /// the display RAM.
    /// 
    /// # Safety
    /// 
    /// The instruction set of the LCD must be set to [`Basic`](InstructionSet::Basic).
    /// `y` must be a value less than 6.
    pub unsafe fn set_y_cursor(&mut self, y: u8) {
        debug_assert!(y < ROWS as u8);
        self.write_command(command!(6 => y));
    }

    /// Sends a command for setting the mode of the display.
    /// 
    /// # Safety
    /// 
    /// The instruction set of the LCD must be set to [`Basic`](InstructionSet::Basic).
    pub unsafe fn set_display_mode(&mut self, display_mode: DisplayMode) {
        self.write_command(command!(3 => display_mode));
    }

    /// Sends a command for setting the bias system voltage coefficient.
    /// 
    /// # Safety
    /// 
    /// The instruction set of the LCD must be set to [`Extended`](InstructionSet::Extended).
    /// `coeff` must be a value less than 8.
    pub unsafe fn set_bias_voltage_coefficient(&mut self, coeff: u8) {
        debug_assert!(coeff < 8);
        self.write_command(command!(4 => coeff));
    }

    /// Sends a command for setting the temperature coefficient.
    /// 
    /// # Safety
    /// 
    /// The instruction set of the LCD must be set to [`Extended`](InstructionSet::Extended).
    /// `coeff` must be a value less than 4.
    pub unsafe fn set_temperature_coefficient(&mut self, coeff: u8) {
        debug_assert!(coeff < 4);
        self.write_command(command!(2 => coeff));
    }

    /// Sends a command for setting the operation voltage.
    /// 
    /// # Safety
    /// 
    /// The instruction set of the LCD must be set to [`Extended`](InstructionSet::Extended).
    /// `voltage` must be a value less than 128.
    pub unsafe fn set_operation_voltage(&mut self, voltage: u8) {
        assert!(voltage < 128);
        self.write_command(command!(7 => voltage));
    }

    /// Writes data to the display RAM.
    pub fn write_data(&mut self, data: u8) {
        self.write(data, true);
    }

    fn write_command(&mut self, data: u8) {
        self.write(data, false);
    }

    fn write(&mut self, value: u8, is_data: bool) {
        self.sce.set_low();

        if is_data {
            self.dc.set_high();
        } else {
            self.dc.set_low();
        }

        for i in 0..8 {
            if ((value >> (7 - i)) & 1) != 0 {
                self.din.set_high();
            } else {
                self.din.set_low();
            }

            self.clk.set_high();
            self.clk.set_low();
        }

        self.sce.set_high();
    }
}
