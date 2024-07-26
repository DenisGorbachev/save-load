use derive_more::{Display, Error, From};

use crate::errors::deserialize_error::DeserializeError;

#[derive(Error, Display, From, Debug)]
pub enum LoadError {
    Io(std::io::Error),
    Deserialize(DeserializeError),
}
