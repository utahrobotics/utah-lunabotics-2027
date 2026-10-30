#![no_std]
#![no_main]

mod time_driver;

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::Timer;
use teensy4_bsp::board;
use teensy4_panic as _;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let board::Resources {
        lpuart6,
        pins,
        mut dma,
        mut gpio2,
        ..
    } = board::t41(board::instances());

    time_driver::init();

    // Debug log: defmt frames out of LPUART6 TX (pin 1) at 1 Mbaud.
    let log_uart = board::lpuart(lpuart6, pins.p1, pins.p0, 1_000_000);
    let Ok(poller) = imxrt_log::defmt::lpuart(
        log_uart,
        dma[7].take().unwrap(),
        imxrt_log::Interrupts::Disabled,
    ) else {
        panic!("logger already initialized");
    };
    spawner.spawn(log_flush(poller)).unwrap();
    info!("embedded-teensy started");

    let led = board::led(&mut gpio2, pins.p13);
    spawner.spawn(blink(led)).unwrap();
}

#[embassy_executor::task]
async fn blink(led: board::Led) {
    let mut count: u32 = 0;
    loop {
        Timer::after_millis(500).await;
        led.toggle();
        count += 1;
        info!("blink {}", count);
    }
}

// Moves buffered log frames out ot the UART.
#[embassy_executor::task]
async fn log_flush(mut poller: imxrt_log::Poller) {
    loop {
        poller.poll();
        Timer::after_millis(2).await;
    }
}
