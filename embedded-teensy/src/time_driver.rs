//! Implementation of embassy_time_driver for the iMXRT1062 (used in the Teensy 4.0 and 4.1)
//! Note that currently, the implementation only uses 32 bits for the timer and pads the result to 64 bits when needed.
//! With the 32kHz clock, it will take 37 hours for the 32-bits to overflow. This should be fine for our use.
//! 
//! Besides (optionally) calling the init function, user code will likely not need to directly interact with this:
//! Embassy uses this in the background to implement asynchronous delays.

use embassy_time_driver::Driver;
use embassy_time_queue_utils::Queue;
use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::lazy_lock::LazyLock;

use critical_section::CriticalSection;
use core::cell::RefCell;
use teensy4_bsp::hal::gpt::{ClockSource, Gpt, Mode, OutputCompareRegister};
use teensy4_bsp::ral::gpt;

use cortex_m_rt::interrupt;
use teensy4_bsp::interrupt;

const OCR: OutputCompareRegister = OutputCompareRegister::OCR1;

struct TimeDriver {
    timer: LazyLock<Mutex<CriticalSectionRawMutex, Gpt>>,
    queue: Mutex<CriticalSectionRawMutex, RefCell<Queue>>
}


impl Driver for TimeDriver {
    fn now(&self) -> u64 {
        critical_section::with(|cs| {
            self.timer.get().borrow(cs).count().into()
        })
    }

    fn schedule_wake(&self, at: u64, waker: &core::task::Waker) {
        critical_section::with(|cs| {
            let mut queue = self.queue.borrow(cs).borrow_mut();

            if queue.schedule_wake(at, waker) {
                self.update_alarm(cs);
            }
        })
    }
}


// Helper methods
impl TimeDriver {

    /// Called when a timer interrupt occurs
    fn on_alarm(&self) {
        critical_section::with(|cs| {

            // Clear interrupt
            self.timer.get().borrow(cs).clear_elapsed(OCR);

            self.update_alarm(cs);
        });
    }

    /// Updates the waker queue and sets the next alarm
    fn update_alarm(&self, cs: CriticalSection) {
        let mut next_alarm = self.queue.borrow(cs).borrow_mut().next_expiration(self.now());

        // During the process of setting the next alarm, the next alarm may have passed.
        while !self.set_alarm(cs, next_alarm) {
            next_alarm = self.queue.borrow(cs).borrow_mut().next_expiration(self.now())
        }
    }

    /// Updates the timer to interrupt at given time.
    fn set_alarm(&self, cs: CriticalSection, time: u64) -> bool {

        // Set the timer
        let gpt = self.timer.get().borrow(cs);

        gpt.set_output_compare_count(OCR, time as u32);

        // If the alarm time has passed, return false
        if time <= self.now() {
            gpt.clear_elapsed(OCR); // Just in case; not sure if this line is actually needed
            false
        } else {
            true
        }
    }
}

// Register this time driver for embassy_time
embassy_time_driver::time_driver_impl!(
    static DRIVER: TimeDriver = TimeDriver {
        timer: LazyLock::new(|| unsafe {setup_gpt()}),
        queue: Mutex::new(RefCell::new(Queue::new()))
    }
);

/// Safety: Must be called no more than once
unsafe fn setup_gpt() -> Mutex<CriticalSectionRawMutex, Gpt> {
    let mut gpt = Gpt::new(unsafe {gpt::GPT1::instance()});

    gpt.set_clock_source(ClockSource::LowFrequencyReferenceClock);
    gpt.set_divider(1);
    gpt.set_mode(Mode::FreeRunning);
    gpt.set_output_interrupt_on_compare(OCR, true);
    gpt.set_reset_on_enable(true);
    gpt.set_rollover_interrupt_enable(false);
    gpt.set_wait_mode_enable(true);

    gpt.enable();

    Mutex::const_new(CriticalSectionRawMutex::new(), gpt)
}

pub fn init() {
    // Force the initialization of the timer by accessing the lazylock
    DRIVER.timer.get();
}

// Register the interrupt handler
#[interrupt]
fn GPT1() {
    DRIVER.on_alarm();
}