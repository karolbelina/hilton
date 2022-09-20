use crate::{DigitalPin, IoCtl, Simulator};
use std::ptr::NonNull;

impl Simulator {
    pub fn led(&mut self, anode: DigitalPin) -> Led {
        let anode_irq = self.io_getirq(IoCtl::IoPortGetIrq { port: anode.port }, anode.pin as _);

        let state = State::default().leak();

        unsafe { Self::irq_register_notify(anode_irq, Some(Led::anode_irq_hook), state.as_ptr()) };

        Led { state }
    }
}

pub struct Led {
    state: NonNull<State>,
}

#[derive(Default)]
struct State {
    is_on: bool,
}

impl State {
    fn leak(self) -> NonNull<Self> {
        NonNull::new(Box::into_raw(Box::new(self))).unwrap()
    }
}

impl Led {
    pub fn is_on(&self) -> bool {
        unsafe { self.state.as_ref() }.is_on
    }
}

impl Led {
    unsafe extern "C" fn anode_irq_hook(
        _: NonNull<simavr_ffi::avr_irq_t>,
        value: u32,
        state: *mut State,
    ) {
        let mut state = state.as_mut().unwrap();
        state.is_on = value != 0;
    }
}
