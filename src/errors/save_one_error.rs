use crate::errors::serialize_one_error::SerializeOneError;
use derive_more::{Display, Error, From};
use std::io;

#[derive(Error, Display, From, Debug)]
pub enum SaveOneError {
    Io(io::Error),
    SerializeOne(SerializeOneError),
}
