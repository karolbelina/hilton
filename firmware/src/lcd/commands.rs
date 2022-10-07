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

#[derive(Default, Clone, Copy)]
pub enum InstructionSet {
    #[default]
    Basic = 0 << 0,
    Extended = 1 << 0,
}

#[derive(Default, Clone, Copy)]
pub enum DisplayMode {
    #[default]
    Blank = 0b00,
    Normal = 0b10,
    Filled = 0b01,
    Inverse = 0b11,
}
