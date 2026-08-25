use crate::errors::unsupported_format_error::UnsupportedFormatError;
use derive_more::{Display, Error, From};

#[cfg(feature = "serde-jsonlines")]
use std::io;
#[cfg(feature = "csv")]
use std::string::FromUtf8Error;

#[derive(Error, Display, From, Debug)]
#[non_exhaustive]
pub enum SerializeManyError {
    UnsupportedFormat(UnsupportedFormatError),
    #[cfg(feature = "serde-jsonlines")]
    SerdeJsonlines(io::Error),
    #[cfg(feature = "csv")]
    Csv(csv::Error),
    #[cfg(feature = "csv")]
    FromUtf8(FromUtf8Error),
}
