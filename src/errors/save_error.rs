use crate::errors::serialize_error::SerializeError;
use derive_more::{Display, Error, From};

#[derive(Error, Display, From, Debug)]
pub enum SaveError {
    Io(std::io::Error),
    Serialize(SerializeError),
}
