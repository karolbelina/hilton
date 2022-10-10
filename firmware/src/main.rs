#![no_std]
#![no_main]

use atmega_hal::{pins, Peripherals};

mod canvas;
mod hilton;
mod lcd;
mod panic;

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
        .build()
        .into_buffered()
        .init();

    hilton::draw(&mut lcd);

    lcd.display_frame();

    loop {}
}
