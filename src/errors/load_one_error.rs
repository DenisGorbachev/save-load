use derive_more::{Display, Error, From};

use crate::errors::deserialize_one_error::DeserializeOneError;

#[derive(Error, Display, From, Debug)]
pub enum LoadOneError {
    Io(std::io::Error),
    DeserializeOne(DeserializeOneError),
}
