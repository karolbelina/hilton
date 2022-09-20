mod ioctl;
mod led;
mod pins;

use std::time::{Duration, Instant};
use std::{alloc, mem, thread};
use std::{ffi::CString, path::Path, ptr::NonNull};

pub use self::ioctl::IoCtl;
pub use self::pins::{DigitalPin, Pins};

const FREQUENCY: u64 = 16_000_000;
const TIME_STEP: Duration = Duration::from_millis(100);

pub struct Simulator {
    avr: NonNull<simavr_ffi::avr_t>,
}

type Cycle = u64;
type CyclesTaken = u64;

impl Simulator {
    pub fn atmega328p(path: impl AsRef<Path>) -> Self {
        Self::new(path, "atmega328p", FREQUENCY as u32)
    }

    fn new(path: impl AsRef<Path>, mcu: &str, frequency: u32) -> Self {
        let c_mcu = CString::new(mcu).unwrap();
        let avr = unsafe { simavr_ffi::avr_make_mcu_by_name(c_mcu.as_ptr()) };

        let mut avr = NonNull::new(avr)
            .unwrap_or_else(|| panic!("avr_make_mcu_by_name() failed: MCU `{}` is not known", mcu));

        let status = unsafe { simavr_ffi::avr_init(avr.as_ptr()) };

        if status != 0 {
            panic!("avr_init() failed (status={})", status);
        }

        unsafe {
            avr.as_mut().frequency = frequency;
        }

        // Safety: We know that `elf_firmware_t`'s layout has a non-zero size.
        //
        // (we also use `alloc_zeroed`, because that's what simavr's docs
        // suggest to do.)
        let ptr = unsafe {
            alloc::alloc_zeroed(alloc::Layout::new::<simavr_ffi::avr_t>())
                as *mut simavr_ffi::elf_firmware_t
        };

        // Unwrap-safety: This can fail only if the underlying allocator failed
        //                to find enough memory to allocate the chunk. In that
        //                case, panicking is the best we can afford anyway.
        let ptr = NonNull::new(ptr).unwrap();

        let path = path.as_ref().display().to_string();

        // Unwrap-safety: Paths cannot contain null-terminators, so a string
        //                we've got from `.display().to_string()` cannot either
        let c_path = CString::new(path).unwrap();

        // Safety: `self.ptr` points at a valid, zeroed instance of
        //         `elf_firmware_t`; `c_path` points at a valid `CString`
        let status = unsafe { simavr_ffi::elf_read_firmware(c_path.as_ptr(), ptr.as_ptr()) };

        if status != 0 {
            panic!(
                "Couldn't load firmware from: {} (status = {})",
                c_path.into_string().unwrap(),
                status
            );
        }

        // Safety: We're non-null, the firmware is non-null, what can go wrong
        unsafe {
            simavr_ffi::avr_load_firmware(avr.as_ptr(), ptr.as_ptr());
        }

        unsafe { simavr_ffi::avr_reset(avr.as_mut()) };

        Self { avr }
    }

    pub fn start(mut self) {
        thread::Builder::new()
            .name("simulator".into())
            .spawn(move || loop {
                let time_taken = Instant::now();
                self.run_for(TIME_STEP);
                let time_taken = time_taken.elapsed();

                if time_taken < TIME_STEP {
                    thread::sleep(TIME_STEP - time_taken);
                }
            })
            .unwrap();
    }

    /// Shorthand for: [`simavr_ffi::avr_io_getirq()`].
    pub(crate) fn io_getirq(&self, ioctl: IoCtl, irq: u32) -> NonNull<simavr_ffi::avr_irq_t> {
        // Safety: This function only searches for a pointer in `avr_t`'s data
        //         structures, so it's safe to call on all parameters
        let ptr =
            unsafe { simavr_ffi::avr_io_getirq(self.avr.as_ptr(), ioctl.into_ffi(), irq as _) };

        NonNull::new(ptr).unwrap()
    }

    /// Shorthand for: [`simavr_ffi::avr_alloc_irq()`].
    #[allow(dead_code)]
    pub(crate) fn alloc_irq(&mut self, name: &'static str) -> NonNull<simavr_ffi::avr_irq_t> {
        let pool = unsafe { &mut self.avr.as_mut().irq_pool as *mut simavr_ffi::avr_irq_pool_t };

        let name = CString::new(name).unwrap();
        let mut name = name.as_ptr();
        let names = &mut name as *mut *const i8;

        let irq = unsafe { simavr_ffi::avr_alloc_irq(pool, 0, 1, names) };

        NonNull::new(irq).unwrap()
    }

    /// Shorthand for: [`simavr_ffi::avr_irq_register_notify()`].
    ///
    /// # Safety
    ///
    /// Callers must ensure that given callback is meant for `irq`.
    pub(crate) unsafe fn irq_register_notify<T>(
        irq: NonNull<simavr_ffi::avr_irq_t>,
        notify: Option<unsafe extern "C" fn(NonNull<simavr_ffi::avr_irq_t>, u32, *mut T)>,
        param: *mut T,
    ) {
        // Safety: We're transmuting two parameters:
        // - `NonNull<ffi::avr_irq_t>` -> `*mut ffi::avr_irq_t`,
        // - `*mut T` -> `*mut c_void`
        //
        // ... where both conversions are legal.
        let notify = mem::transmute(notify);

        // Safety: We're transmuting `*mut T` -> `*mut c_void`, which is legal
        let param = mem::transmute(param);

        simavr_ffi::avr_irq_register_notify(irq.as_ptr(), notify, param);
    }

    fn cycle(&self) -> Cycle {
        unsafe { self.avr.as_ref() }.cycle
    }

    pub fn step(&mut self) -> CyclesTaken {
        let cycle = self.cycle();
        let state = unsafe { simavr_ffi::avr_run(self.avr.as_ptr()) };
        let cycles_taken = self.cycle() - cycle;

        let state = CpuState::from_ffi(state);

        match state {
            CpuState::Running => {}

            CpuState::Crashed => {
                panic!(
                    "AVR crashed (e.g. the program stepped on an invalid \
                     instruction)"
                );
            }

            CpuState::Sleeping => {
                panic!(
                    "AVR went to sleep (this panics, because the simulator doesn't \
                     provide any way to wake up the microcontroller yet)"
                );
            }

            state => {
                panic!("Unexpected CPU state: {:?}", state);
            }
        }

        cycles_taken
    }

    pub fn run_for(&mut self, duration: Duration) {
        let mut cycles = (duration.as_secs_f64() * FREQUENCY as f64) as u64;

        while cycles > 0 {
            cycles = cycles.saturating_sub(self.step());
        }
    }
}

unsafe impl Send for Simulator {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CpuState {
    Limbo,
    Stopped,
    Running,
    Sleeping,
    Step,
    StepDone,
    Done,
    Crashed,
}

impl CpuState {
    pub fn from_ffi(val: i32) -> Self {
        match val as u32 {
            simavr_ffi::cpu_Limbo => Self::Limbo,
            simavr_ffi::cpu_Stopped => Self::Stopped,
            simavr_ffi::cpu_Running => Self::Running,
            simavr_ffi::cpu_Sleeping => Self::Sleeping,
            simavr_ffi::cpu_Step => Self::Step,
            simavr_ffi::cpu_StepDone => Self::StepDone,
            simavr_ffi::cpu_Done => Self::Done,
            simavr_ffi::cpu_Crashed => Self::Crashed,

            val => {
                panic!("Unknown CpuState: {}", val);
            }
        }
    }
}
