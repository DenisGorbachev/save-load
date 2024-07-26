use std::ffi::OsStr;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::Path;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::errors::deserialize_error::DeserializeError;
use crate::errors::load_as_error::LoadAsError;
use crate::errors::load_error::LoadError;
use crate::errors::path_has_no_extension_error::PathHasNoExtensionError;
use crate::errors::record_not_found_error::CsvRowNotFoundError;
use crate::errors::save_as_error::SaveAsError;
use crate::errors::save_error::SaveError;
use crate::errors::serialize_error::SerializeError;
use crate::errors::try_from_path_error::TryFromPathError;
use crate::errors::unrecognized_extension_error::UnrecognizedExtensionError;

#[derive(Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "strum", derive(strum::Display))]
#[non_exhaustive]
pub enum Format {
    #[cfg(feature = "serde_json")]
    Json,
    #[cfg(feature = "serde_yaml")]
    Yaml,
    #[cfg(any(feature = "serde-xml-rs", feature = "quick-xml"))]
    Xml,
    #[cfg(feature = "toml")]
    Toml,
    #[cfg(feature = "csv")]
    Csv,
}

impl Format {
    pub fn save<T: Serialize>(self, value: &T, path: impl AsRef<Path>) -> Result<(), SaveError> {
        let mut file = File::create(path)?;
        let output = self.serialize(value)?;
        file.write_all(output.as_bytes())?;
        Ok(())
    }

    pub fn save_as<T: Serialize>(value: &T, path: impl AsRef<Path>) -> Result<(), SaveAsError> {
        let format = Format::try_from_path(path.as_ref())?;
        format.save(value, path).map_err(From::from)
    }

    pub fn save_to<T: Serialize>(self, value: &T, file_dir: impl AsRef<Path>, file_stem: &str) -> Result<(), SaveError> {
        let path_buf = file_dir.as_ref().join(self.to_file_name(file_stem));
        self.save(value, path_buf)
    }

    pub fn load<T: DeserializeOwned>(self, path: impl AsRef<Path>) -> Result<T, LoadError> {
        let string = read_to_string(path)?;
        let output = self.deserialize(&string)?;
        Ok(output)
    }

    pub fn load_as<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T, LoadAsError> {
        let format = Format::try_from_path(path.as_ref())?;
        format.load(path).map_err(From::from)
    }

    #[allow(unreachable_patterns, unused_variables, unreachable_code)]
    pub fn serialize<T: Serialize>(self, input: &T) -> Result<String, SerializeError> {
        Ok(match self {
            #[cfg(feature = "serde_json")]
            Format::Json => serde_json::to_string_pretty(input)?,
            #[cfg(feature = "serde_yaml")]
            Format::Yaml => serde_yaml::to_string(input)?,
            #[cfg(feature = "serde-xml-rs")]
            Format::Xml => serde_xml_rs::to_string(input)?,
            #[cfg(feature = "quick-xml")]
            Format::Xml => quick_xml::se::to_string(input)?,
            #[cfg(feature = "toml")]
            Format::Toml => toml::to_string(input)?,
            #[cfg(feature = "csv")]
            Format::Csv => {
                let mut writer = csv::Writer::from_writer(vec![]);
                writer.serialize(input)?;
                let vec = writer
                    .into_inner()
                    .expect("Writer must return a vec without errors");
                String::from_utf8(vec)?
            }
        })
    }

    #[allow(unreachable_patterns, unused_variables, unreachable_code)]
    pub fn deserialize<T: DeserializeOwned>(self, input: &str) -> Result<T, DeserializeError> {
        Ok(match self {
            #[cfg(feature = "serde_json")]
            Format::Json => serde_json::from_str(input)?,
            #[cfg(feature = "serde_yaml")]
            Format::Yaml => serde_yaml::from_str(input)?,
            #[cfg(feature = "serde-xml-rs")]
            Format::Xml => serde_xml_rs::from_str(input)?,
            #[cfg(feature = "quick-xml")]
            Format::Xml => quick_xml::de::from_str(input)?,
            #[cfg(feature = "toml")]
            Format::Toml => toml::from_str(input)?,
            #[cfg(feature = "csv")]
            Format::Csv => {
                // NOTE: The input must contain the columns
                let mut reader = csv::Reader::from_reader(input.as_bytes());
                let mut iter = reader.deserialize();
                iter.next()
                    .ok_or::<DeserializeError>(CsvRowNotFoundError.into())??
            }
        })
    }

    pub fn to_file_extension(&self) -> &'static str {
        match self {
            #[cfg(feature = "serde_json")]
            Format::Json => "json",
            #[cfg(feature = "serde_yaml")]
            Format::Yaml => "yaml",
            #[cfg(any(feature = "serde-xml-rs", feature = "quick-xml"))]
            Format::Xml => "xml",
            #[cfg(feature = "toml")]
            Format::Toml => "toml",
            #[allow(unreachable_patterns)]
            _ => "txt",
        }
    }

    pub fn to_file_name(&self, stem: &str) -> String {
        format!("{stem}.{extension}", extension = self.to_file_extension())
    }

    pub fn try_from_file_extension(extension: &OsStr) -> Result<Self, UnrecognizedExtensionError> {
        match extension.to_str() {
            #[cfg(feature = "serde_json")]
            Some("json") => Ok(Format::Json),
            #[cfg(feature = "serde_yaml")]
            Some("yaml") | Some("yml") => Ok(Format::Yaml),
            #[cfg(any(feature = "serde-xml-rs", feature = "quick-xml"))]
            Some("xml") => Ok(Format::Xml),
            #[cfg(feature = "toml")]
            Some("toml") => Ok(Format::Toml),
            #[allow(unreachable_patterns)]
            _ => Err(UnrecognizedExtensionError {
                extension: extension.to_owned(),
            }),
        }
    }

    pub fn try_from_path(path: impl AsRef<Path>) -> Result<Self, TryFromPathError> {
        let extension = path
            .as_ref()
            .extension()
            .ok_or_else(|| PathHasNoExtensionError {
                path: path.as_ref().into(),
            })?;
        let format = Self::try_from_file_extension(extension)?;
        Ok(format)
    }
}

impl TryFrom<&str> for Format {
    type Error = UnrecognizedExtensionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from_file_extension(value.as_ref())
    }
}

impl TryFrom<&OsStr> for Format {
    type Error = UnrecognizedExtensionError;

    fn try_from(value: &OsStr) -> Result<Self, Self::Error> {
        Self::try_from_file_extension(value)
    }
}

impl TryFrom<&Path> for Format {
    type Error = TryFromPathError;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        Self::try_from_path(value)
    }
}
