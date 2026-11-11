#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
extern crate cu_bincode as bincode;

mod actuator;
mod calibration;
mod codec;
mod log;
mod secondary;
mod usb;

pub use actuator::*;
pub use calibration::*;
pub use codec::*;
pub use log::*;
pub use secondary::*;
pub use usb::*;

pub const PRIME_PICO_SERIAL: &str = "USR-PICO-PRIME";
pub const SECONDARY_PICO_SERIAL: &str = "USR-PICO-SECONDARY";

pub const PROTOCOL_VERSION: u16 = 2;

/// Maximum decoded postcard frame length 
pub const MAX_FRAME_LEN: usize = 128;
/// Maximum on-wire frame length COBS overhead plus the 0
pub const MAX_ENCODED_FRAME_LEN: usize = MAX_FRAME_LEN + MAX_FRAME_LEN / 254 + 2;

pub const SECONDARY_UART_BAUD: u32 = 115_200;
pub const HOST_SERIAL_BAUD: u32 = 115_200;
