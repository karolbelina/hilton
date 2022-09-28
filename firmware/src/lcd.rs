use atmega_hal::{clock::MHz16, delay::Delay};
use avr_hal_generic::port::{mode::Output, Pin, PinOps};
use avr_hal_generic::prelude::*;

mod commands;

use self::commands::*;

const COLUMNS: u32 = 84;
const ROWS: u32 = 6;

#[macro_export]
macro_rules! lcd_10168 {
    (
        $rst:expr => rst,
        $sce:expr => sce,
        $dc:expr => dc,
        $din:expr => din,
        $clk:expr => clk $(,)?
    ) => {
        Lcd10168::new(
            $rst.into_output(),
            $sce.into_output(),
            $dc.into_output(),
            $din.into_output(),
            $clk.into_output(),
        )
    };
}

pub struct Lcd10168<RST, SCE, DC, DIN, CLK> {
    rst: Pin<Output, RST>,
    sce: Pin<Output, SCE>,
    dc: Pin<Output, DC>,
    din: Pin<Output, DIN>,
    clk: Pin<Output, CLK>,

    delay: Delay<MHz16>,

    chip_mode: ChipMode,
    addressing_mode: AddressingMode,
}

impl<RST, SCE, DC, DIN, CLK> Lcd10168<RST, SCE, DC, DIN, CLK> {
    pub fn new(
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
            chip_mode: Default::default(),
            addressing_mode: Default::default(),
        }
    }
}

impl<RST, SCE, DC, DIN, CLK> Lcd10168<RST, SCE, DC, DIN, CLK>
where
    RST: PinOps,
    SCE: PinOps,
    DC: PinOps,
    DIN: PinOps,
    CLK: PinOps,
{
    pub fn reset(&mut self) {
        self.rst.set_high();
        self.sce.set_high();
        self.delay.delay_ms(10u16);
        self.rst.set_low();
        self.delay.delay_ms(70u16);
        self.rst.set_high();
        self.sce.set_low();
    }

    pub fn write_data(&mut self, data: u8) {
        self.write(data, true);
    }

    fn write_command(&mut self, data: u8) {
        self.write(data, true);
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
