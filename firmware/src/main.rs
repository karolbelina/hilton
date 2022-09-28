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

    let mut lcd = lcd_10168! {
        pins.pc1 => rst,
        pins.pc2 => sce,
        pins.pc3 => dc,
        pins.pc4 => din,
        pins.pc5 => clk,
    };

    lcd.reset();

    lcd.commands().activate_chip();
    lcd.commands().set_horizontal_addressing();
    {
        let mut ext_commands = lcd.ext_commands();
        ext_commands.set_bias_voltage_coefficient(0);
        ext_commands.set_temperature_coefficient(0);
        ext_commands.set_operation_voltage(50);
    }
    lcd.commands().set_normal_display();
    lcd.commands().set_x_cursor(0);
    lcd.commands().set_y_cursor(0);

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
