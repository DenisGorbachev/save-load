use derive_more::{Display, Error, From};

use crate::errors::load_many_error::LoadManyError;
use crate::errors::try_from_path_error::TryFromPathError;

#[derive(Error, Display, From, Debug)]
pub enum LoadManyAsError {
    UnknownFormat(TryFromPathError),
    LoadMany(LoadManyError),
}
