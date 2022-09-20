#![no_std]
#![no_main]

use atmega_hal::{clock::MHz16, delay::Delay, pins, Peripherals};
use avr_hal_generic::prelude::*;
use panic_halt as _;

#[atmega_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = pins!(dp);
    let mut delay = Delay::<MHz16>::new();
    let mut led = pins.pb0.into_output();

    loop {
        led.toggle();
        delay.delay_ms(1000u16);
    }
}
