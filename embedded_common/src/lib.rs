#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
extern crate cu_bincode as bincode;

mod actuator;
mod codec;
mod log;
mod usb;

pub use actuator::*;
pub use codec::*;
pub use log::*;
pub use usb::*;

pub const PROTOCOL_VERSION: u16 = 2;

/// Maximum decoded postcard frame length 
pub const MAX_FRAME_LEN: usize = 128;
/// Maximum on-wire frame length COBS overhead plus the 0
pub const MAX_ENCODED_FRAME_LEN: usize = MAX_FRAME_LEN + MAX_FRAME_LEN / 254 + 2;

pub const HOST_SERIAL_BAUD: u32 = 115_200;
