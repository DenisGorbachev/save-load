use std::fs::read_to_string;
use std::path::Path;

use derive_more::{Display, Error, From};
use serde::de::DeserializeOwned;

use crate::format::{Format, TryFromPathError};
use crate::functions::deserialize::{deserialize, DeserializeError};

pub fn load<T>(path: impl AsRef<Path>) -> Result<T, LoadError>
where
    T: DeserializeOwned,
{
    let format = Format::try_from_path(path.as_ref())?;
    let string = read_to_string(path)?;
    let output = deserialize(&string, format)?;
    Ok(output)
}

#[derive(Error, Display, From, Debug)]
pub enum LoadError {
    UnknownFormat(TryFromPathError),
    Io(std::io::Error),
    Deserialize(DeserializeError),
}
