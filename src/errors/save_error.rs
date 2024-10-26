use crate::errors::serialize_one_error::SerializeOneError;
use derive_more::{Display, Error, From};

#[derive(Error, Display, From, Debug)]
pub enum SaveError {
    Io(std::io::Error),
    Serialize(SerializeOneError),
}
