#![no_std]
#![no_main]

mod time_driver;

use core::cell::RefCell;

use critical_section::Mutex;
use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use teensy4_bsp::board;
use teensy4_bsp::hal::gpt::{ClockSource, Gpt, Mode, OutputCompareRegister::OCR1};
use teensy4_bsp::ral::interrupt;
use teensy4_panic as _;

/// Stage 1.5 test only: GPT2 interrupts every 0.5 s and wakes the blink task,
/// so Embassy can be tested without a time driver. GPT1 is the time driver's.
static TICK_TIMER: Mutex<RefCell<Option<Gpt>>> = Mutex::new(RefCell::new(None));
static TICK: Signal<CriticalSectionRawMutex, ()> = Signal::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let board::Resources {
        pins,
        mut gpio2,
        gpt1,
        mut gpt2,
        ..
    } = board::t41(board::instances());

    // Stub for Johnny's time driver
    time_driver::init(gpt1);

    // GPT2 counts PERCLK (1 MHz). In restart mode the counter returns to 0 at
    // each OCR1 match, so this interrupts every 500_000 counts = 0.5 s.
    gpt2.set_clock_source(ClockSource::PeripheralClock);
    gpt2.set_divider(1);
    gpt2.set_mode(Mode::Restart);
    gpt2.set_output_compare_count(OCR1, 500_000);
    gpt2.set_reset_on_enable(true);
    gpt2.set_output_interrupt_on_compare(OCR1, true);
    gpt2.enable();
    critical_section::with(|cs| TICK_TIMER.borrow(cs).replace(Some(gpt2)));
    // Safety: the GPT2 handler only uses state behind critical sections.
    unsafe { cortex_m::peripheral::NVIC::unmask(interrupt::GPT2) };

    let led = board::led(&mut gpio2, pins.p13);
    spawner.spawn(blink(led)).unwrap();
}

#[embassy_executor::task]
async fn blink(led: board::Led) {
    loop {
        TICK.wait().await; // Test with Johnny's timer after thats been implemented
        led.toggle();
    }
}

#[cortex_m_rt::interrupt]
fn GPT2() {
    critical_section::with(|cs| {
        if let Some(gpt) = TICK_TIMER.borrow(cs).borrow().as_ref() {
            gpt.clear_elapsed(OCR1);
            while gpt.is_elapsed(OCR1) {}
        }
    });
    TICK.signal(());
}
