use std::ffi::{OsStr, OsString};
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

use derive_more::{Display, Error, From};
use serde::{Deserialize, Serialize};

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
}

impl Format {
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

#[derive(Error, Eq, PartialEq, Hash, Clone, Debug)]
pub struct UnrecognizedExtensionError {
    pub extension: OsString,
}

impl Display for UnrecognizedExtensionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unrecognized extension: {:?}", self.extension)
    }
}

#[derive(Error, Eq, PartialEq, Hash, Clone, Debug)]
pub struct PathHasNoExtensionError {
    pub path: PathBuf,
}

impl Display for PathHasNoExtensionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Path has no extension: {:?}", self.path)
    }
}

#[derive(Error, Display, From, Eq, PartialEq, Hash, Clone, Debug)]
pub enum TryFromPathError {
    PathHasNoExtension(PathHasNoExtensionError),
    UnrecognizedExtension(UnrecognizedExtensionError),
}
