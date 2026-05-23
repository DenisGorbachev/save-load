use crate::errors::deserialize_one_error::DeserializeOneError;
use crate::errors::unsupported_format_error::UnsupportedFormatError;
use derive_more::{Display, Error, From};

#[derive(Error, Display, From, Debug)]
#[non_exhaustive]
pub enum DeserializeManyError {
    UnsupportedFormat(UnsupportedFormatError),
    DeserializeOneError(DeserializeOneError),
    IoError(std::io::Error),
}
