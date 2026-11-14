//! MCU ↔ host USB protocol.

use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Error, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "std", derive(bincode::Encode, bincode::Decode))]
pub enum UsbWriteError {
    #[error("endpoint buffer overflow")]
    BufferOverflow,
    #[error("endpoint disabled (host detached)")]
    Disabled,
}
