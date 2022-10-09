#![allow(clippy::identity_op)]

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
    Blank = 0b000,
    Normal = 0b100,
    Filled = 0b001,
    Inverse = 0b101,
}
