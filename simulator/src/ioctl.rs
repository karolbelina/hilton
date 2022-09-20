#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IoCtl {
    AdcGetIrq,
    IoPortGetIrq { port: char },
    IoPortGetState { port: char },
    UartGetFlags { uart: char },
    UartGetIrq { uart: char },
    UartSetFlags { uart: char },
}

impl IoCtl {
    pub fn into_ffi(self) -> u32 {
        let ctl = match self {
            Self::AdcGetIrq => [b'a', b'd', b'c', b'0'],
            Self::IoPortGetIrq { port } => [b'i', b'o', b'g', port as u8],
            Self::IoPortGetState { port } => [b'i', b'o', b's', port as u8],
            Self::UartGetFlags { uart } => [b'u', b'a', b'g', uart as u8],
            Self::UartGetIrq { uart } => [b'u', b'a', b'r', uart as u8],
            Self::UartSetFlags { uart } => [b'u', b'a', b's', uart as u8],
        };

        u32::from_be_bytes(ctl)
    }
}
