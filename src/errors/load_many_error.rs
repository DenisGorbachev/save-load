use crate::errors::deserialize_many_error::DeserializeManyError;
use derive_more::{Display, Error, From};

#[derive(Error, Display, From, Debug)]
pub enum LoadManyError {
    Io(std::io::Error),
    DeserializeMany(DeserializeManyError),
}
