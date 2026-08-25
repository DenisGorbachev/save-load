use derive_more::{Display, Error, From};
use std::io;

use crate::errors::deserialize_one_error::DeserializeOneError;

#[derive(Error, Display, From, Debug)]
pub enum LoadOneError {
    Io(io::Error),
    DeserializeOne(DeserializeOneError),
}
