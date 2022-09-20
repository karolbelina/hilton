use crate::Simulator;

impl Simulator {
    pub fn pins(&self) -> Pins {
        Pins
    }
}

pub struct Pins;

pub struct DigitalPin {
    pub(crate) port: char,
    pub(crate) pin: u8,
}

impl DigitalPin {
    fn new(port: char, pin: u8) -> Self {
        Self { port, pin }
    }
}

macro_rules! digital_pins {
    ( $( $fn:ident($port:expr, $pin:expr) ),* $(,)? ) => {
        impl Pins {
            $(
                pub fn $fn(&self) -> DigitalPin {
                    DigitalPin::new($port, $pin)
                }
            )*
        }
    }
}

digital_pins! {
    pb0('B', 0),
    pb1('B', 1),
    pb2('B', 2),
    pb3('B', 3),
    pb4('B', 4),
    pb5('B', 5),
    pb6('B', 6),
    pb7('B', 7),

    pc0('C', 0),
    pc1('C', 1),
    pc2('C', 2),
    pc3('C', 3),
    pc4('C', 4),
    pc5('C', 5),
    pc6('C', 6),

    pd0('D', 0),
    pd1('D', 1),
    pd2('D', 2),
    pd3('D', 3),
    pd4('D', 4),
    pd5('D', 5),
    pd6('D', 6),
    pd7('D', 7),
}
