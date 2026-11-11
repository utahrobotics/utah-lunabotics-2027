#![no_std]
#![no_main]

mod time_driver;
mod usb;

use defmt::{debug, info, warn};
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
        gpt1,
        ..
    } = board::t41(board::instances());

    time_driver::init(gpt1);

    // Debug log: defmt frames out of LPUART6 TX (pin 1) at 500 kbaud.
    let log_uart = board::lpuart(lpuart6, pins.p1, pins.p0, 500_000);
    let Ok(poller) = imxrt_log::defmt::lpuart(
        log_uart,
        dma[7].take().unwrap(),
        imxrt_log::Interrupts::Disabled,
    ) else {
        panic!("logger already initialized");
    };
    spawner.spawn(log_flush(poller)).unwrap();
    info!("embedded-teensy started");
    // lunabot-cu finds the board by its serial number, "USR-PICO-PRIME" is
    // embedded_common::PRIME_PICO_SERIAL; switch to constant once embedded_common
    // is a dependency.
    let (usb_tx, usb_rx) = usb::init(
        usb,
        usb::Identity {
            vid: 0xc0de,
            pid: 0xcafe,
            manufacturer: "USR",
            product: "USR-PICO-PRIME",
            serial_number: "USR-PICO-PRIME",
        },
    );
    spawner.spawn(usb_echo(usb_tx, usb_rx)).unwrap();
    spawner.spawn(bootloader_watch()).unwrap();

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

/// For testing: sends back whatever the host sends; Pico's
/// USB tasks will replace this once they're ported
#[embassy_executor::task]
async fn usb_echo(mut tx: usb::CdcSender, mut rx: usb::CdcReceiver) {
    todo!("finish implementing this task");
    let mut buf = [0u8, 64];
    loop {
        tx.wait_connection().await;
        rx.wait_connection().await;
        info!("USB connected");
        loop {
            let n = match rx.read_packet(&mut buf).await {
                Ok(n) => n,
                Err(usb::Disconnected) => break,
            };
            debug!("echoing {} bytes (DTR {}", n, tx.dtr());
            if tx.write_packet(&buf[..n]).await.is_err() {
                break;
            }
        }
        warn!("USB Disconnected");
    }
}
