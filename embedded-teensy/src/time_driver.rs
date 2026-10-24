//! Embassy time driver for the Teensy 4.1
//! Owner: Johnny ("Implement Embassy time driver for Teensy").
//!
//! This is a stub. Until it's implemented, anything that uses `embassy_time`
//! (`Timer`, `with_timeout`, `Instant::now`) panics, and the LED blinks SOS.
//!
//! What the rest of the firmware assumes:
//! - `init` runs once, first thing in `main`, and receives GPT1 (clocked from the
//!   BSP's 1 MHz PERCLK). Change the signature if you'd rather use another timer.
//!   GPT2 is borrowed by the stage 1.5 blink test.
//! - Ticks are 1 MHz (`tick-hz-1_000_000` on `embassy_time` in Cargo.toml).
//! - Contract from `embassy_time_driver::Driver`: `now()` is monotonic, never
//!   overflows (extend the 32-bit counter to 64 bits), and must not fail even
//!   before `init` runs. `schedule_wake` must wake `waker` at or after `at`;
//!   `embassy_time_queue_utils::Queue` keeps the list of pending wakeups.
use core::task::Waker;

use embassy_time_driver::Driver;
use teensy4_bsp::hal::gpt::Gpt;

pub fn init(_gpt: Gpt) {}

struct TeensyTimeDriver;

impl Driver for TeensyTimeDriver {
    fn now(&self) -> u64 {
        todo!("time driver: now()")
    }

    fn schedule_wake(&self, _at: u64, _waker: &Waker) {
        todo!("time driver: schedule_wake()")
    }
}

embassy_time_driver::time_driver_impl!(static DRIVER: TeensyTimeDriver = TeensyTimeDriver);
