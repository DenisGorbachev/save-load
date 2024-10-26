use crate::errors::unsupported_format_error::UnsupportedFormatError;
use crate::format::Format;
use derive_more::{Display, Error, From};

#[derive(Error, Display, From, Debug)]
#[non_exhaustive]
pub enum SerializeManyError {
    UnsupportedFormat(UnsupportedFormatError),
    #[cfg(feature = "serde-jsonlines")]
    SerdeJsonlines(std::io::Error),
    #[cfg(feature = "csv")]
    Csv(csv::Error),
    #[cfg(feature = "csv")]
    FromUtf8(std::string::FromUtf8Error),
}

impl From<Format> for SerializeManyError {
    fn from(value: Format) -> Self {
        Self::UnsupportedFormat(value.into())
    }
}
