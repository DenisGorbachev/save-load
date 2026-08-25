use derive_more::Error;
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;

#[derive(Error, Eq, PartialEq, Hash, Clone, Debug)]
pub struct PathHasNoExtensionError {
    pub path: PathBuf,
}

impl Display for PathHasNoExtensionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Path has no extension: {:?}", self.path)
    }
}
