#![no_std]
#![no_main]

use atmega_hal::{pins, Peripherals};
use panic_halt as _;

mod lcd;

use self::lcd::Lcd10168;

#[atmega_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = pins!(dp);

    let mut lcd = Lcd10168::builder()
        .reset(pins.pc1)
        .chip_enable(pins.pc2)
        .data_command(pins.pc3)
        .data_in(pins.pc4)
        .clock(pins.pc5)
        .build();

    lcd.reset();

    lcd.function_set(lcd::ChipMode::Active, lcd::AddressingMode::Horizontal, lcd::InstructionSet::Extended);
    unsafe {
        lcd.set_bias_voltage_coefficient(0);
        lcd.set_temperature_coefficient(0);
        lcd.set_operation_voltage(50);
    }
    lcd.function_set(lcd::ChipMode::Active, lcd::AddressingMode::Horizontal, lcd::InstructionSet::Basic);
    unsafe {
        lcd.set_display_mode(lcd::DisplayMode::Normal);
        lcd.set_x_cursor(0);
        lcd.set_y_cursor(0);
    }

    lcd.write_data(0b01111110);
    lcd.write_data(0b11110011);
    lcd.write_data(0b11011111);
    lcd.write_data(0b10011111);
    lcd.write_data(0b10011111);
    lcd.write_data(0b11011111);
    lcd.write_data(0b11110011);
    lcd.write_data(0b01111110);

    loop {}
}
