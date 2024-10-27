use crate::errors::unsupported_format_error::UnsupportedFormatError;
use crate::format::Format;
use derive_more::{Display, Error, From};

#[derive(Error, Display, From, Debug)]
#[non_exhaustive]
pub enum DeserializeManyError {
    UnsupportedFormat(UnsupportedFormatError),
}

impl From<Format> for DeserializeManyError {
    fn from(value: Format) -> Self {
        Self::UnsupportedFormat(value.into())
    }
}