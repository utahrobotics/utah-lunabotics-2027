#![no_std]
#![no_main]

mod time_driver;

use embassy_executor::Spawner;
use embassy_time::Timer;
use teensy4_bsp::board;
use teensy4_panic as _;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let board::Resources {
        pins,
        mut gpio2,
        ..
    } = board::t41(board::instances());

    time_driver::init();

    let led = board::led(&mut gpio2, pins.p13);
    spawner.spawn(blink(led)).unwrap();
}

#[embassy_executor::task]
async fn blink(led: board::Led) {
    loop {
        Timer::after_millis(500).await;
        led.toggle();
    }
}