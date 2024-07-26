use derive_more::{Display, Error, From};

use crate::errors::load_error::LoadError;
use crate::errors::try_from_path_error::TryFromPathError;

#[derive(Error, Display, From, Debug)]
pub enum LoadAsError {
    UnknownFormat(TryFromPathError),
    Load(LoadError),
}
