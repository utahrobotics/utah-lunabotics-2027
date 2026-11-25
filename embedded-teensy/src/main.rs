#![no_std]
#![no_main]

mod actuators;
mod host;
mod time_driver;
mod usb;

use actuators::ActuatorDriver;
use defmt::{debug, info, warn};
use embassy_executor::Spawner;
use embassy_time::Timer;
use embedded_common::Actuator;
use static_cell::StaticCell;
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
        usb,
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
    // lunabot-cu finds the board by its serial number, so the Teensy uses the
    // Prime Pico's (embedded_common::PRIME_PICO_SERIAL).
    // is a dependency.
    let (_usb_tx, usb_rx) = usb::init(
        usb,
        usb::Identity {
            vid: 0xc0de,
            pid: 0xcafe,
            manufacturer: "USR",
            product: embedded_common::PRIME_PICO_SERIAL,
            serial_number: embedded_common::PRIME_PICO_SERIAL,
        },
    );

    // Stand-ins for now: they only log. The actuator-outputs stage swaps in the
    // real drivers. Same order as the Pico, so index 3 is the fan.
    static ACTUATORS: StaticCell<[ActuatorDriver; 4]> = StaticCell::new();
    let actuators = ACTUATORS.init([
        ActuatorDriver::new(Actuator::Bucket),
        ActuatorDriver::new(Actuator::Lift),
        ActuatorDriver::new(Actuator::Dumper),
        ActuatorDriver::new(Actuator::Motor4),
    ]);
    // The sending half, _usb_tx, goes to usb_tx_loop in the next substage.
    spawner.spawn(host::usb_rx_loop(usb_rx, actuators)).unwrap();
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
    let mut buf = [0u8; 64];
    loop {
        tx.wait_connection().await;
        rx.wait_connection().await;
        info!("USB connected");
        loop {
            let n = match rx.read_packet(&mut buf).await {
                Ok(n) => n,
                Err(usb::Disconnected) => break,
            };
            debug!("echoing {} bytes (DTR {})", n, tx.dtr());
            if tx.write_packet(&buf[..n]).await.is_err() {
                break;
            }
        }
        warn!("USB disconnected");
    }
}

/// Reboots into the bootloader when the host sets the port to 134 baud.
#[embassy_executor::task]
async fn bootloader_watch() {
    usb::reboot_to_bootloader_on_request().await
}
