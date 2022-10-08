use super::Lcd10168;
use atmega_hal::{clock::MHz16, delay::Delay};
use avr_hal_generic::port::{
    mode::{Io, Output},
    Pin, PinOps,
};

impl Lcd10168<(), (), (), (), ()> {
    pub fn builder() -> UnconnectedLcd10168Builder {
        Lcd10168Builder::new()
    }
}

pub struct Unconnected;

pub struct Connected<PIN>(Pin<Output, PIN>);

impl<PIN> Connected<PIN> {
    pub fn new(pin: Pin<Output, PIN>) -> Self {
        Self(pin)
    }

    pub fn into_inner(self) -> Pin<Output, PIN> {
        self.0
    }
}

pub struct Lcd10168Builder<R, S, D, I, C> {
    rst: R,
    sce: S,
    dc: D,
    din: I,
    clk: C,
}

pub type UnconnectedLcd10168Builder =
    Lcd10168Builder<Unconnected, Unconnected, Unconnected, Unconnected, Unconnected>;

impl UnconnectedLcd10168Builder {
    pub fn new() -> Self {
        Self {
            rst: Unconnected,
            sce: Unconnected,
            dc: Unconnected,
            din: Unconnected,
            clk: Unconnected,
        }
    }
}

impl<S, D, I, C> Lcd10168Builder<Unconnected, S, D, I, C> {
    pub fn reset<MODE: Io, RST: PinOps>(
        self,
        pin: Pin<MODE, RST>,
    ) -> Lcd10168Builder<Connected<RST>, S, D, I, C> {
        Lcd10168Builder {
            rst: Connected::new(pin.into_output()),
            sce: self.sce,
            dc: self.dc,
            din: self.din,
            clk: self.clk,
        }
    }
}

impl<R, D, I, C> Lcd10168Builder<R, Unconnected, D, I, C> {
    pub fn chip_enable<MODE: Io, SCE: PinOps>(
        self,
        pin: Pin<MODE, SCE>,
    ) -> Lcd10168Builder<R, Connected<SCE>, D, I, C> {
        Lcd10168Builder {
            rst: self.rst,
            sce: Connected::new(pin.into_output()),
            dc: self.dc,
            din: self.din,
            clk: self.clk,
        }
    }
}

impl<R, S, I, C> Lcd10168Builder<R, S, Unconnected, I, C> {
    pub fn data_command<MODE: Io, DC: PinOps>(
        self,
        pin: Pin<MODE, DC>,
    ) -> Lcd10168Builder<R, S, Connected<DC>, I, C> {
        Lcd10168Builder {
            rst: self.rst,
            sce: self.sce,
            dc: Connected::new(pin.into_output()),
            din: self.din,
            clk: self.clk,
        }
    }
}

impl<R, S, D, C> Lcd10168Builder<R, S, D, Unconnected, C> {
    pub fn data_in<MODE: Io, DIN: PinOps>(
        self,
        pin: Pin<MODE, DIN>,
    ) -> Lcd10168Builder<R, S, D, Connected<DIN>, C> {
        Lcd10168Builder {
            rst: self.rst,
            sce: self.sce,
            dc: self.dc,
            din: Connected::new(pin.into_output()),
            clk: self.clk,
        }
    }
}

impl<R, S, D, I> Lcd10168Builder<R, S, D, I, Unconnected> {
    pub fn clock<MODE: Io, CLK: PinOps>(
        self,
        pin: Pin<MODE, CLK>,
    ) -> Lcd10168Builder<R, S, D, I, Connected<CLK>> {
        Lcd10168Builder {
            rst: self.rst,
            sce: self.sce,
            dc: self.dc,
            din: self.din,
            clk: Connected::new(pin.into_output()),
        }
    }
}

impl<RST, SCE, DC, DIN, CLK>
    Lcd10168Builder<Connected<RST>, Connected<SCE>, Connected<DC>, Connected<DIN>, Connected<CLK>>
{
    pub fn build(self) -> Lcd10168<RST, SCE, DC, DIN, CLK> {
        Lcd10168 {
            rst: self.rst.into_inner(),
            sce: self.sce.into_inner(),
            dc: self.dc.into_inner(),
            din: self.din.into_inner(),
            clk: self.clk.into_inner(),
            delay: Delay::<MHz16>::new(),
        }
    }
}
