use crate::format::Format;
use derive_more::{Display, Error, From};

#[derive(Error, Display, From, Debug, Copy, Clone)]
pub struct UnsupportedFormatError {
    pub format: Format,
}
