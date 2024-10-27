use derive_more::{Display, Error, From};

use crate::errors::save_one_error::SaveOneError;
use crate::errors::try_from_path_error::TryFromPathError;

#[derive(Error, Display, From, Debug)]
pub enum SaveOneAsError {
    UnknownFormat(TryFromPathError),
    SaveOne(SaveOneError),
}
