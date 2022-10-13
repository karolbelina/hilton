use avr_hal_generic::avr_device::interrupt;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    // Disable interrupts - firmware has panicked so no ISRs should continue running
    interrupt::disable();

    #[cfg(debug_assertions)]
    blink_led();

    #[cfg(not(debug_assertions))]
    halt();
}

#[cfg(debug_assertions)]
fn blink_led() -> ! {
    use atmega_hal::{clock::MHz16, delay::Delay, pins, Peripherals};
    use avr_hal_generic::prelude::*;

    // SAFETY: no other code can run after the panic handler was called
    let dp = unsafe { Peripherals::steal() };
    let pins = pins!(dp);
    let mut led = pins.pb0.into_output();
    let mut delay = Delay::<MHz16>::new();

    loop {
        led.toggle();
        delay.delay_ms(100u16);
    }
}

#[cfg(not(debug_assertions))]
fn halt() -> ! {
    use core::sync::atomic::{compiler_fence, Ordering};

    loop {
        compiler_fence(Ordering::SeqCst);
    }
}
