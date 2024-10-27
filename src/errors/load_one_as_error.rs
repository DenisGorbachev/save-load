use derive_more::{Display, Error, From};

use crate::errors::load_one_error::LoadOneError;
use crate::errors::try_from_path_error::TryFromPathError;

#[derive(Error, Display, From, Debug)]
pub enum LoadOneAsError {
    UnknownFormat(TryFromPathError),
    LoadOne(LoadOneError),
}
