//! Temporary wiring test: sends "hello" out of pin 1 with no logger or DMA
#![no_std]
#![no_main]

use imxrt_log as _;
use teensy4_bsp::{board, hal::lpuart::Status};
use teensy4_panic as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    let board::Resources { pins, mut gpio2, lpuart6, .. } = board::t41(board::instances());
    let led = board::led(&mut gpio2, pins.p13);
    let uart = board::lpuart(lpuart6, pins.p1, pins.p0, 500_000);
    loop {
        for &byte in b"hello\r\n" {
            while !uart.status().contains(Status::TRANSMIT_EMPTY) {}
            uart.write_byte(byte);
        }
        led.toggle();
        cortex_m::asm::delay(300_000_000);
    }
}
