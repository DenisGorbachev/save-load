use derive_more::{Display, Error, From};

use crate::errors::deserialize_one_error::DeserializeOneError;

#[derive(Error, Display, From, Debug)]
pub enum LoadError {
    Io(std::io::Error),
    Deserialize(DeserializeOneError),
}
