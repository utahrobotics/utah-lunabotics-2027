//! COBS-framed postcard encoding shared by every link (host USB and prime/secondary UART).

use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Error, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "std", derive(bincode::Encode, bincode::Decode))]
pub enum WireError {
    #[error("frame exceeds {max} bytes")]
    FrameTooLong { max: u16 },
    #[error("corrupt COBS framing")]
    Cobs,
    #[error("frame ended early (truncated or layout mismatch)")]
    UnexpectedEnd,
    #[error("malformed varint")]
    BadVarint,
    #[error("invalid bool byte")]
    BadBool,
    #[error("invalid Option tag")]
    BadOption,
    #[error("invalid enum discriminant")]
    BadEnum,
    #[error("value out of range for target type (unknown variant?)")]
    InvalidValue,
    #[error("malformed encoding")]
    BadEncoding,
    #[error("serialize buffer full")]
    BufferFull,
    #[error("unclassified postcard error")]
    Other,
}

impl From<postcard::Error> for WireError {
    fn from(e: postcard::Error) -> Self {
        use postcard::Error as E;
        match e {
            E::DeserializeUnexpectedEnd => Self::UnexpectedEnd,
            E::DeserializeBadVarint => Self::BadVarint,
            E::DeserializeBadBool => Self::BadBool,
            E::DeserializeBadOption => Self::BadOption,
            E::DeserializeBadEnum => Self::BadEnum,
            E::DeserializeBadEncoding => Self::BadEncoding,
            E::SerdeDeCustom => Self::InvalidValue,
            E::SerializeBufferFull => Self::BufferFull,
            _ => Self::Other,
        }
    }
}

/// Serializes `value` with postcard and COBS-frames it into `buf`.
/// The returned slice includes the trailing `0` sentinel.
pub fn encode_frame<'a, T: serde::Serialize>(
    value: &T,
    buf: &'a mut [u8],
) -> Result<&'a mut [u8], WireError> {
    Ok(postcard::to_slice_cobs(value, buf)?)
}

/// Deserializes a postcard value from an already COBS-decoded frame.
pub fn decode_frame<'a, T: serde::Deserialize<'a>>(cobs_decoded: &'a [u8]) -> Result<T, WireError> {
    Ok(postcard::from_bytes(cobs_decoded)?)
}

/// Like [`encode_frame`], but allocates the output.
#[cfg(feature = "std")]
pub fn encode_frame_vec<T: serde::Serialize>(value: &T) -> Result<std::vec::Vec<u8>, WireError> {
    Ok(postcard::to_stdvec_cobs(value)?)
}
