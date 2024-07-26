use derive_more::{Display, Error, From};

use crate::errors::save_error::SaveError;
use crate::errors::try_from_path_error::TryFromPathError;

#[derive(Error, Display, From, Debug)]
pub enum SaveAsError {
    UnknownFormat(TryFromPathError),
    Save(SaveError),
}
