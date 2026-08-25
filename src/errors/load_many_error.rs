use crate::errors::deserialize_many_error::DeserializeManyError;
use derive_more::{Display, Error, From};
use std::io;

#[derive(Error, Display, From, Debug)]
pub enum LoadManyError {
    Io(io::Error),
    DeserializeMany(DeserializeManyError),
}
