use derive_more::Error;
use std::ffi::OsString;
use std::fmt::{self, Display, Formatter};

#[derive(Error, Eq, PartialEq, Hash, Clone, Debug)]
pub struct UnrecognizedExtensionError {
    pub extension: OsString,
}

impl Display for UnrecognizedExtensionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Unrecognized extension: {:?}", self.extension)
    }
}
