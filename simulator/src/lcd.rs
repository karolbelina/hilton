use std::ptr::NonNull;

use crate::{DigitalPin, IoCtl, Simulator};

impl Simulator {
    pub fn lcd_10168(
        &mut self,
        rst: DigitalPin,
        sce: DigitalPin,
        dc: DigitalPin,
        din: DigitalPin,
        clk: DigitalPin,
    ) -> Lcd10168 {
        // let src = self.io_getirq(IoCtl::IoPortGetIrq { port }, pin as _);
        // let dst = self.alloc_irq("anode");

        // let (tx, rx) = mpsc::channel();

        // unsafe { self.irq_register_notify(dst, Some(Led::anode_irq_hook), Box::into_raw(Box::new(tx))) };

        // unsafe { simavr_ffi::avr_connect_irq(src.as_ptr(), dst.as_ptr()) };

        let rst_irq = self.io_getirq(IoCtl::IoPortGetIrq { port: rst.port }, rst.pin as _);
        let sce_irq = self.io_getirq(IoCtl::IoPortGetIrq { port: sce.port }, sce.pin as _);
        let dc_irq = self.io_getirq(IoCtl::IoPortGetIrq { port: dc.port }, dc.pin as _);
        let din_irq = self.io_getirq(IoCtl::IoPortGetIrq { port: din.port }, din.pin as _);
        let clk_irq = self.io_getirq(IoCtl::IoPortGetIrq { port: clk.port }, clk.pin as _);

        let state = State::default().leak();

        unsafe { Self::irq_register_notify(rst_irq, Some(Lcd10168::rst_irq_hook), state.as_ptr()) };
        unsafe { Self::irq_register_notify(sce_irq, Some(Lcd10168::sce_irq_hook), state.as_ptr()) };
        unsafe { Self::irq_register_notify(dc_irq, Some(Lcd10168::dc_irq_hook), state.as_ptr()) };
        unsafe { Self::irq_register_notify(din_irq, Some(Lcd10168::din_irq_hook), state.as_ptr()) };
        unsafe { Self::irq_register_notify(clk_irq, Some(Lcd10168::clk_irq_hook), state.as_ptr()) };

        Lcd10168 { state }
    }
}

const DISPLAY_WIDTH: u32 = 84;
const DISPLAY_HEIGHT: u32 = 48;
const VIDEO_MEMORY_SIZE: usize = DISPLAY_WIDTH as usize * DISPLAY_HEIGHT as usize;
const COLUMNS: u32 = DISPLAY_WIDTH;
const ROWS: u32 = DISPLAY_HEIGHT / 8;

pub struct Lcd10168 {
    state: NonNull<State>,
}

struct State {
    is_chip_enabled: bool,
    mode: Mode,
    register: Register,
    data_in: bool,

    power_down_control: bool,
    entry_mode: EntryMode,
    is_extended: bool,
    display_mode: DisplayMode,
    x: u8,
    y: u8,
    temperature_control: u8,
    bias_system: u8,
    operation_voltage: u8,
    video_memory: [bool; VIDEO_MEMORY_SIZE],
}

impl State {
    fn leak(self) -> NonNull<Self> {
        NonNull::new(Box::into_raw(Box::new(self))).unwrap()
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            is_chip_enabled: Default::default(),
            mode: Default::default(),
            register: Default::default(),
            data_in: Default::default(),

            power_down_control: Default::default(),
            entry_mode: Default::default(),
            is_extended: Default::default(),
            display_mode: Default::default(),
            x: Default::default(),
            y: Default::default(),
            temperature_control: Default::default(),
            bias_system: Default::default(),
            operation_voltage: Default::default(),
            video_memory: [false; VIDEO_MEMORY_SIZE],
        }
    }
}

impl State {
    pub fn update(&mut self) {
        self.register.shift_in(self.data_in);

        match self.mode {
            Mode::Command => {
                if let Some(command) = self.register.try_command(self.is_extended) {
                    match command {
                        Command::FunctionSet { pd, v, h } => {
                            self.power_down_control = pd;
                            self.entry_mode = v;
                            self.is_extended = h;
                        }
                        Command::DisplayControl(display_mode) => self.display_mode = display_mode,
                        Command::SetYAddress(y) => self.y = y,
                        Command::SetXAddress(x) => self.x = x,
                        Command::TemperatureControl(tc) => self.temperature_control = tc,
                        Command::BiasSystem(bs) => self.bias_system = bs,
                        Command::OperationVoltage(v_op) => self.operation_voltage = v_op,
                    }
                }
            }

            Mode::Data => {
                if let Some(data) = self.register.try_data() {
                    self.update_video_memory(data);
                }
            }
        }
    }

    fn update_video_memory(&mut self, data: u8) {
        for (n, bit) in data.into_bits().into_iter().rev().enumerate() {
            let i = (self.y as usize * 8 as usize + n) * COLUMNS as usize + self.x as usize;
            self.video_memory[i] = bit;
        }

        match self.entry_mode {
            EntryMode::HorizontalAddressing => {
                self.x += 1;
                if self.x as u32 == COLUMNS {
                    self.x = 0;
                    self.y += 1;
                    if self.y as u32 == ROWS {
                        self.y = 0;
                    }
                }
            }
            EntryMode::VerticalAddressing => {
                self.y += 1;
                if self.y as u32 == ROWS {
                    self.y = 0;
                    self.x += 1;
                    if self.x as u32 == COLUMNS {
                        self.x = 0;
                    }
                }
            }
        }
    }
}

#[derive(Default)]
enum Mode {
    Data,
    #[default]
    Command,
}

#[derive(Default)]
struct Register {
    data: u8,
    counter: u8,
}

#[derive(Debug)]
enum Command {
    FunctionSet { pd: bool, v: EntryMode, h: bool },
    DisplayControl(DisplayMode),
    SetYAddress(u8),
    SetXAddress(u8),
    TemperatureControl(u8),
    BiasSystem(u8),
    OperationVoltage(u8),
}

impl Register {
    pub fn shift_in(&mut self, bit: bool) {
        if self.counter == 8 {
            self.counter = 0;
        }

        self.data <<= 1;
        self.data |= bit as u8;
        self.counter += 1;
    }

    pub fn try_data(&self) -> Option<u8> {
        (self.counter == 8).then_some(self.data)
    }

    pub fn try_command(&self, is_extended: bool) -> Option<Command> {
        let data = self.try_data()?.into_bits();

        match data {
            [false, false, true, false, false, pd, v, h] => {
                let v = if v {
                    EntryMode::VerticalAddressing
                } else {
                    EntryMode::HorizontalAddressing
                };

                Some(Command::FunctionSet { pd, v, h })
            }

            [false, false, false, false, true, d, false, e] if !is_extended => {
                Some(Command::DisplayControl(match (d, e) {
                    (false, false) => DisplayMode::AllDotsOff,
                    (true, false) => DisplayMode::NormalMode,
                    (false, true) => DisplayMode::AllDotsOn,
                    (true, true) => DisplayMode::InverseMode,
                }))
            }

            [false, true, false, false, false, y_2, y_1, y_0] if !is_extended => {
                Some(Command::SetYAddress(u8::from_bits([
                    false, false, false, false, false, y_2, y_1, y_0,
                ])))
            }

            [true, x_6, x_5, x_4, x_3, x_2, x_1, x_0] if !is_extended => {
                Some(Command::SetXAddress(u8::from_bits([
                    false, x_6, x_5, x_4, x_3, x_2, x_1, x_0,
                ])))
            }

            [false, false, false, false, false, true, tc_1, tc_0] if is_extended => {
                Some(Command::TemperatureControl(u8::from_bits([
                    false, false, false, false, false, false, tc_1, tc_0,
                ])))
            }

            [false, false, false, true, false, bs_2, bs_1, bs_0] if is_extended => {
                Some(Command::BiasSystem(u8::from_bits([
                    false, false, false, false, false, bs_2, bs_1, bs_0,
                ])))
            }

            [true, v_op_6, v_op_5, v_op_4, v_op_3, v_op_2, v_op_1, v_op_0] if is_extended => {
                Some(Command::OperationVoltage(u8::from_bits([
                    false, v_op_6, v_op_5, v_op_4, v_op_3, v_op_2, v_op_1, v_op_0,
                ])))
            }

            _ => None,
        }
    }
}

#[derive(Debug, Default)]
enum EntryMode {
    #[default]
    HorizontalAddressing,
    VerticalAddressing,
}

#[derive(Debug, Default)]
enum DisplayMode {
    #[default]
    AllDotsOff,
    NormalMode,
    AllDotsOn,
    InverseMode,
}

impl Lcd10168 {
    pub fn is_pixel_on_at(&self, x: u32, y: u32) -> bool {
        let index = y * DISPLAY_WIDTH + x;
        unsafe { self.state.as_ref() }.video_memory[index as usize]
    }

    pub fn display_size(&self) -> (u32, u32) {
        (DISPLAY_WIDTH, DISPLAY_HEIGHT)
    }
}

impl Lcd10168 {
    unsafe extern "C" fn rst_irq_hook(
        _: NonNull<simavr_ffi::avr_irq_t>,
        value: u32,
        state: *mut State,
    ) {
        let should_reset = value == 0;
        let state = state.as_mut().unwrap();

        if should_reset {
            state.video_memory = [false; VIDEO_MEMORY_SIZE];
        }
    }

    unsafe extern "C" fn sce_irq_hook(
        _: NonNull<simavr_ffi::avr_irq_t>,
        value: u32,
        state: *mut State,
    ) {
        let value = value == 0;
        let state = state.as_mut().unwrap();
        state.is_chip_enabled = value;
    }

    unsafe extern "C" fn dc_irq_hook(
        _: NonNull<simavr_ffi::avr_irq_t>,
        value: u32,
        state: *mut State,
    ) {
        let state = state.as_mut().unwrap();
        state.mode = if value != 0 {
            Mode::Data
        } else {
            Mode::Command
        };
    }

    unsafe extern "C" fn din_irq_hook(
        _: NonNull<simavr_ffi::avr_irq_t>,
        value: u32,
        state: *mut State,
    ) {
        let value = value != 0;
        let state = state.as_mut().unwrap();
        state.data_in = value;
    }

    unsafe extern "C" fn clk_irq_hook(
        _: NonNull<simavr_ffi::avr_irq_t>,
        value: u32,
        state: *mut State,
    ) {
        if value == 0 {
            return;
        }

        let state = state.as_mut().unwrap();

        if !state.is_chip_enabled {
            return;
        }

        state.update();
    }
}

trait IntoBits {
    type Output;

    fn into_bits(self) -> Self::Output;
}

impl IntoBits for u8 {
    type Output = [bool; 8];

    fn into_bits(self) -> Self::Output {
        [
            ((self >> 7) & 1) != 0,
            ((self >> 6) & 1) != 0,
            ((self >> 5) & 1) != 0,
            ((self >> 4) & 1) != 0,
            ((self >> 3) & 1) != 0,
            ((self >> 2) & 1) != 0,
            ((self >> 1) & 1) != 0,
            ((self >> 0) & 1) != 0,
        ]
    }
}

trait FromBits {
    type Input;

    fn from_bits(bits: Self::Input) -> Self;
}

impl FromBits for u8 {
    type Input = [bool; 8];

    fn from_bits(bits: Self::Input) -> Self {
        let mut byte = 0u8;

        for bit in bits {
            byte <<= 1;
            byte |= bit as u8;
        }

        byte
    }
}
