use crate::errors::serialize_one_error::SerializeOneError;
use derive_more::{Display, Error, From};

#[derive(Error, Display, From, Debug)]
pub enum SaveOneError {
    Io(std::io::Error),
    SerializeOne(SerializeOneError),
}
