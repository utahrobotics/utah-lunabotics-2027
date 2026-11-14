//! Structured firmware log records, forwarded to the host over USB.

use thiserror::Error;

use crate::{ActuatorCommand, FaultMask, UsbWriteError, WireError};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "std", derive(bincode::Encode, bincode::Decode))]
pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
}

/// Only "prime" for a robot with one MCU
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "std", derive(bincode::Encode, bincode::Decode))]
pub enum Board {
    Prime,
    Secondary,
}

#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "std", derive(bincode::Encode, bincode::Decode))]
pub struct LogRecord<A> {
    pub board: Board,
    pub level: Level,
    pub uptime_ms: u32,
    pub event: Event<A>,
}

#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "std", derive(bincode::Encode, bincode::Decode))]
pub enum Event<A> {
    Boot { protocol_version: u16 },
    UsbConnected,
    UsbDisconnected,
    UsbWriteFailed { error: UsbWriteError },
    FrameEncodeFailed { error: WireError },
    CommandReceived(ActuatorCommand<A>),
    CommandDecodeFailed { error: WireError, frame_len: u16 },
    CommandIgnored { command: ActuatorCommand<A>, reason: IgnoreReason },
    AngleTargetSet { actuator: A, target_rad: f32 },
    AngleTargetReached { actuator: A, error_rad: f32 },
    PidHalted { actuator: A, reason: PidHaltReason },
    DriverFaultRaised { faults: FaultMask<A> },
    DriverFaultCleared { faults: FaultMask<A> },
    SecondaryTimeout { seq: u8, waited_ms: u16 },
    SecondaryLinkError { seq: u8, error: LinkError },
    SecondarySeqMismatch { expected: u8, got: u8 },
    SecondaryRecovered { failed_requests: u16 },
    SampleFailed { channel: u8, fault: SampleFault },
    SampleRecovered { channel: u8 },
    RequestRejected { error: LinkError },
    UartReadFailed { error: UartError },
    UartWriteFailed { error: UartError },
    LogsDropped { count: u16 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Error, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "std", derive(bincode::Encode, bincode::Decode))]
pub enum UartError {
    #[error("RX FIFO overrun")]
    Overrun,
    #[error("break condition")]
    Break,
    #[error("parity error")]
    Parity,
    #[error("framing error")]
    Framing,
    #[error("unknown UART error")]
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Error, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "std", derive(bincode::Encode, bincode::Decode))]
pub enum LinkError {
    #[error("uart: {0}")]
    Uart(UartError),
    #[error("decode: {0}")]
    Decode(WireError),
    #[error("encode: {0}")]
    Encode(WireError),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Error, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "std", derive(bincode::Encode, bincode::Decode))]
pub enum SampleFault {
    #[error("channel out of range")]
    InvalidChannel,
    #[error("ADC conversion failed")]
    AdcConversionFailed,
    #[error("sample missing from response")]
    Missing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Error, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "std", derive(bincode::Encode, bincode::Decode))]
pub enum IgnoreReason {
    #[error("actuator has no position sensor, angle control unavailable")]
    NoAngleControl,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Error, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "std", derive(bincode::Encode, bincode::Decode))]
pub enum PidHaltReason {
    #[error("position sensor reading unavailable")]
    PositionUnavailable,
}
