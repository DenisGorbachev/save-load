use std::fs::File;
use std::io::Write;
use std::path::Path;

use derive_more::{Display, Error, From};
use serde::Serialize;

use crate::format::Format;
use crate::functions::serialize;
use crate::functions::serialize::SerializeError;

pub fn save<T: Serialize>(value: &T, file_dir: impl AsRef<Path>, file_stem: &str, format: Format) -> Result<(), SaveError> {
    let path_buf = file_dir.as_ref().join(format.to_file_name(file_stem));
    let mut file = File::create(path_buf)?;
    let output = serialize::serialize(value, format)?;
    file.write_all(output.as_bytes())?;
    Ok(())
}

#[derive(Error, Display, From, Debug)]
pub enum SaveError {
    Io(std::io::Error),
    Serialize(SerializeError),
}
