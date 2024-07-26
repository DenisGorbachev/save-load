use crate::errors::path_has_no_extension_error::PathHasNoExtensionError;
use crate::errors::unrecognized_extension_error::UnrecognizedExtensionError;
use derive_more::{Display, Error, From};

#[derive(Error, Display, From, Eq, PartialEq, Hash, Clone, Debug)]
pub enum TryFromPathError {
    PathHasNoExtension(PathHasNoExtensionError),
    UnrecognizedExtension(UnrecognizedExtensionError),
}
