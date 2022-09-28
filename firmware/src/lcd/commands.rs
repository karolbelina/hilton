#![allow(dead_code)]

use avr_hal_generic::port::PinOps;

use super::{Lcd10168, COLUMNS, ROWS};

impl<RST, SCE, DC, DIN, CLK> Lcd10168<RST, SCE, DC, DIN, CLK>
where
    RST: PinOps,
    SCE: PinOps,
    DC: PinOps,
    DIN: PinOps,
    CLK: PinOps,
{
    pub fn commands<'a>(&'a mut self) -> Commands<'a, RST, SCE, DC, DIN, CLK> {
        Commands {
            lcd: self,
        }
    }

    pub fn ext_commands<'a>(&'a mut self) -> ExtCommands<'a, RST, SCE, DC, DIN, CLK> {
        let command = 1 << 5;
        let pd = self.chip_mode as u8;
        let v = self.addressing_mode as u8;
        let h = InstructionSet::Extended as u8;
        self.write(command | pd | v | h, false);

        ExtCommands {
            lcd: self,
        }
    }
}

pub struct Commands<'a, RST, SCE, DC, DIN, CLK> {
    lcd: &'a mut Lcd10168<RST, SCE, DC, DIN, CLK>,
}

impl<'a, RST, SCE, DC, DIN, CLK> Commands<'a, RST, SCE, DC, DIN, CLK>
where
    RST: PinOps,
    SCE: PinOps,
    DC: PinOps,
    DIN: PinOps,
    CLK: PinOps,
{
    pub fn activate_chip(&mut self) {
        self.lcd.chip_mode = ChipMode::Active;
        self.function_set(InstructionSet::Basic);
    }

    pub fn power_down_chip(&mut self) {
        self.lcd.chip_mode = ChipMode::PowerDown;
        self.function_set(InstructionSet::Basic);
    }

    pub fn set_horizontal_addressing(&mut self) {
        self.lcd.addressing_mode = AddressingMode::Horizontal;
        self.function_set(InstructionSet::Basic);
    }

    pub fn set_vertical_addressing(&mut self) {
        self.lcd.addressing_mode = AddressingMode::Vertical;
        self.function_set(InstructionSet::Basic);
    }

    fn function_set(&mut self, instruction_set: InstructionSet) {
        let command = 1 << 5;
        let pd = self.lcd.chip_mode as u8;
        let v = self.lcd.addressing_mode as u8;
        let h = instruction_set as u8;
        self.lcd.write_command(command | pd | v | h);
    }
}

impl<'a, RST, SCE, DC, DIN, CLK> Commands<'a, RST, SCE, DC, DIN, CLK>
where
    RST: PinOps,
    SCE: PinOps,
    DC: PinOps,
    DIN: PinOps,
    CLK: PinOps,
{
    pub fn set_normal_display(&mut self) {
        let command = 1 << 3;
        let de = DisplayMode::Normal as u8;
        self.lcd.write_command(command | de);
    }

    pub fn set_inverted_display(&mut self) {
        let command = 1 << 3;
        let de = DisplayMode::Inverse as u8;
        self.lcd.write_command(command | de);
    }

    pub fn set_blank_display(&mut self) {
        let command = 1 << 3;
        let de = DisplayMode::Blank as u8;
        self.lcd.write_command(command | de);
    }

    pub fn set_filled_display(&mut self) {
        let command = 1 << 3;
        let de = DisplayMode::Filled as u8;
        self.lcd.write_command(command | de);
    }

    pub fn set_x_cursor(&mut self, x: u8) {
        assert!(x < COLUMNS as u8);

        let command = 1 << 7;
        self.lcd.write(command | x, false);
    }

    pub fn set_y_cursor(&mut self, y: u8) {
        assert!(y < ROWS as u8);

        let command = 1 << 6;
        self.lcd.write(command | y, false);
    }
}

pub struct ExtCommands<'a, RST, SCE, DC, DIN, CLK>
where
    RST: PinOps,
    SCE: PinOps,
    DC: PinOps,
    DIN: PinOps,
    CLK: PinOps,
{
    lcd: &'a mut Lcd10168<RST, SCE, DC, DIN, CLK>,
}

impl<'a, RST, SCE, DC, DIN, CLK> ExtCommands<'a, RST, SCE, DC, DIN, CLK>
where
    RST: PinOps,
    SCE: PinOps,
    DC: PinOps,
    DIN: PinOps,
    CLK: PinOps,
{
    pub fn set_bias_voltage_coefficient(&mut self, coeff: u8) {
        assert!(coeff < 8);

        let command = 1 << 4;
        self.lcd.write(command | coeff, false);
    }

    pub fn set_temperature_coefficient(&mut self, coeff: u8) {
        assert!(coeff < 4);

        let command = 1 << 2;
        self.lcd.write(command | coeff, false);
    }

    pub fn set_operation_voltage(&mut self, coeff: u8) {
        assert!(coeff < 128);

        let command = 1 << 7;
        self.lcd.write(command | coeff, false);
    }
}

impl<'a, RST, SCE, DC, DIN, CLK> Drop for ExtCommands<'a, RST, SCE, DC, DIN, CLK>
where
    RST: PinOps,
    SCE: PinOps,
    DC: PinOps,
    DIN: PinOps,
    CLK: PinOps,
{
    fn drop(&mut self) {
        let command = 1 << 5;
        let pd = self.lcd.chip_mode as u8;
        let v = self.lcd.addressing_mode as u8;
        let h = InstructionSet::Basic as u8;
        self.lcd.write(command | pd | v | h, false);
    }
}

#[derive(Default, Clone, Copy)]
pub enum ChipMode {
    Active = 0 << 2,
    #[default]
    PowerDown = 1 << 2,
}

#[derive(Default, Clone, Copy)]
pub enum AddressingMode {
    #[default]
    Horizontal = 0 << 1,
    Vertical = 1 << 1,
}

#[derive(Default)]
enum InstructionSet {
    #[default]
    Basic = 0 << 0,
    Extended = 1 << 0,
}

enum DisplayMode {
    Blank = 0b00,
    Normal = 0b10,
    Filled = 0b01,
    Inverse = 0b11,
}
