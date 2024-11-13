use crate::errors::deserialize_many_error::DeserializeManyError;
use crate::errors::deserialize_one_error::DeserializeOneError;
use crate::errors::load_many_as_error::LoadManyAsError;
use crate::errors::load_many_error::LoadManyError;
use crate::errors::load_one_as_error::LoadOneAsError;
use crate::errors::load_one_error::LoadOneError;
use crate::errors::path_has_no_extension_error::PathHasNoExtensionError;
use crate::errors::save_one_as_error::SaveOneAsError;
use crate::errors::save_one_error::SaveOneError;
use crate::errors::serialize_many_error::SerializeManyError;
use crate::errors::serialize_one_error::SerializeOneError;
use crate::errors::try_from_path_error::TryFromPathError;
use crate::errors::unrecognized_extension_error::UnrecognizedExtensionError;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use strum::Display;

#[derive(Serialize, Deserialize, Display, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[non_exhaustive]
pub enum Format {
    #[cfg(feature = "serde_json")]
    Json,
    #[cfg(feature = "serde-jsonlines")]
    Jsonl,
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
    pub fn save_one<T: Serialize>(self, path: impl AsRef<Path>, value: &T) -> Result<(), SaveOneError> {
        let mut file = File::create(path)?;
        let output = self.serialize_one(value)?;
        file.write_all(output.as_bytes())?;
        Ok(())
    }

    pub fn save_one_as<T: Serialize>(path: impl AsRef<Path>, value: &T) -> Result<(), SaveOneAsError> {
        let format = Format::try_from_path(path.as_ref())?;
        format.save_one(path, value).map_err(From::from)
    }

    pub fn save_one_to<T: Serialize>(self, file_dir: impl AsRef<Path>, file_stem: &str, value: &T) -> Result<(), SaveOneError> {
        let path_buf = file_dir.as_ref().join(self.to_file_name(file_stem));
        self.save_one(path_buf, value)
    }

    pub fn load_one<T: DeserializeOwned>(self, path: impl AsRef<Path>) -> Result<T, LoadOneError> {
        let string = read_to_string(path)?;
        let output = self.deserialize_one(&string)?;
        Ok(output)
    }

    pub fn load_many<T: DeserializeOwned + 'static>(self, path: impl AsRef<Path>) -> Result<Box<dyn Iterator<Item = Result<T, DeserializeOneError>>>, LoadManyError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let output = self.deserialize_many_from_reader(reader)?;
        Ok(output)
    }

    pub fn load_one_as<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T, LoadOneAsError> {
        let format = Format::try_from_path(path.as_ref())?;
        format.load_one(path).map_err(From::from)
    }

    pub fn load_many_as<T: DeserializeOwned + 'static>(path: impl AsRef<Path>) -> Result<Box<dyn Iterator<Item = Result<T, DeserializeOneError>>>, LoadManyAsError> {
        let format = Format::try_from_path(path.as_ref())?;
        format.load_many(path).map_err(From::from)
    }

    pub fn print_one<T: Serialize>(self, input: &T) -> Result<(), SerializeOneError> {
        let string = self.serialize_one(input)?;
        print!("{string}");
        Ok(())
    }

    pub fn eprint_one<T: Serialize>(self, input: &T) -> Result<(), SerializeOneError> {
        let string = self.serialize_one(input)?;
        eprint!("{string}");
        Ok(())
    }

    pub fn println_one<T: Serialize>(self, input: &T) -> Result<(), SerializeOneError> {
        let string = self.serialize_one(input)?;
        println!("{string}");
        Ok(())
    }

    pub fn eprintln_one<T: Serialize>(self, input: &T) -> Result<(), SerializeOneError> {
        let string = self.serialize_one(input)?;
        eprintln!("{string}");
        Ok(())
    }

    pub fn write_one<T: Serialize>(self, writer: &mut impl Write, input: &T) -> Result<(), SaveOneError> {
        let string = self.serialize_one(input)?;
        write!(writer, "{string}")?;
        Ok(())
    }

    pub fn writeln_one<T: Serialize>(self, writer: &mut impl Write, input: &T) -> Result<(), SaveOneError> {
        let string = self.serialize_one(input)?;
        writeln!(writer, "{string}")?;
        Ok(())
    }

    #[allow(unreachable_patterns, unused_variables, unreachable_code)]
    pub fn serialize_one<T: Serialize>(self, input: &T) -> Result<String, SerializeOneError> {
        Ok(match self {
            #[cfg(feature = "serde_json")]
            Format::Json => serde_json::to_string_pretty(input)?,
            #[cfg(feature = "serde-jsonlines")]
            Format::Jsonl => {
                let mut buffer = Vec::new();
                let mut ser = serde_jsonlines::JsonLinesWriter::new(&mut buffer);
                ser.write(input)
                    .map_err(SerializeOneError::SerdeJsonlines)?;
                String::from_utf8_lossy(&buffer).into_owned()
            }
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
    pub fn serialize_many_to_writer<'a, T: Serialize + 'a>(self, input: impl IntoIterator<Item = &'a T>, writer: &mut impl Write) -> Result<(), SerializeManyError> {
        let items = input.into_iter();
        match self {
            #[cfg(feature = "serde_json")]
            Format::Json => Err(crate::errors::unsupported_format_error::UnsupportedFormatError {
                format: self,
            })?,
            #[cfg(feature = "serde-jsonlines")]
            Format::Jsonl => {
                let mut writer = serde_jsonlines::JsonLinesWriter::new(writer);
                for item in items {
                    writer.write(item)?;
                }
                Ok(())
            }
            #[cfg(feature = "serde_yaml")]
            Format::Yaml => Err(crate::errors::unsupported_format_error::UnsupportedFormatError {
                format: self,
            })?,
            #[cfg(feature = "serde-xml-rs")]
            Format::Xml => Err(crate::errors::unsupported_format_error::UnsupportedFormatError {
                format: self,
            })?,
            #[cfg(feature = "quick-xml")]
            Format::Xml => Err(crate::errors::unsupported_format_error::UnsupportedFormatError {
                format: self,
            })?,
            #[cfg(feature = "toml")]
            Format::Toml => Err(crate::errors::unsupported_format_error::UnsupportedFormatError {
                format: self,
            })?,
            #[cfg(feature = "csv")]
            Format::Csv => {
                let mut writer = csv::Writer::from_writer(writer);
                for item in items {
                    writer.serialize(item)?;
                }
                Ok(())
            }
        }
    }

    #[allow(unreachable_patterns, unused_variables, unreachable_code)]
    pub fn deserialize_one<T: DeserializeOwned>(self, input: &str) -> Result<T, DeserializeOneError> {
        Ok(match self {
            #[cfg(feature = "serde_json")]
            Format::Json => serde_json::from_str(input)?,
            #[cfg(feature = "serde-jsonlines")]
            Format::Jsonl => {
                let mut reader = serde_jsonlines::JsonLinesReader::new(input.as_bytes());
                reader
                    .read()
                    .map_err(DeserializeOneError::SerdeJsonlines)?
                    .ok_or::<DeserializeOneError>(crate::errors::item_not_found_error::ItemNotFoundError.into())?
            }
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
                    .ok_or::<DeserializeOneError>(crate::errors::item_not_found_error::ItemNotFoundError.into())??
            }
        })
    }

    #[allow(unreachable_patterns, unused_variables, unreachable_code)]
    pub fn deserialize_many_from_reader<T: DeserializeOwned + 'static>(self, mut reader: impl BufRead + 'static) -> Result<Box<dyn Iterator<Item = Result<T, DeserializeOneError>>>, DeserializeManyError> {
        Ok(match self {
            #[cfg(feature = "serde_json")]
            Format::Json => Err(crate::errors::unsupported_format_error::UnsupportedFormatError {
                format: self,
            })?,
            #[cfg(feature = "serde-jsonlines")]
            Format::Jsonl => {
                let reader = serde_jsonlines::JsonLinesReader::new(reader);
                let iter = reader
                    .read_all()
                    .map(|x| x.map_err(DeserializeOneError::SerdeJsonlines));
                Box::new(iter)
            }
            #[cfg(feature = "serde_yaml")]
            Format::Yaml => Err(crate::errors::unsupported_format_error::UnsupportedFormatError {
                format: self,
            })?,
            #[cfg(feature = "serde-xml-rs")]
            Format::Xml => Err(crate::errors::unsupported_format_error::UnsupportedFormatError {
                format: self,
            })?,
            #[cfg(feature = "quick-xml")]
            Format::Xml => Err(crate::errors::unsupported_format_error::UnsupportedFormatError {
                format: self,
            })?,
            #[cfg(feature = "toml")]
            Format::Toml => {
                // crate::errors::unsupported_format_error::UnsupportedFormatError {
                //     format: self,
                // }.into()
                // TODO: TOML doesn't support line-by-line deserialization, so using it with reader doesn't make sense
                // TODO: Decide on how to handle such formats (most likely )
                let mut s = String::new();
                reader.read_to_string(&mut s)?;
                let vec = self.deserialize_one::<Vec<T>>(&s)?;
                Box::new(vec.into_iter().map(Ok))
            }
            #[cfg(feature = "csv")]
            Format::Csv => {
                // NOTE: The input must contain the columns
                let iter = csv::Reader::from_reader(reader)
                    .into_deserialize()
                    .map(|result| result.map_err(DeserializeOneError::Csv));
                Box::new(iter)
            }
        })
    }

    pub fn to_file_extension(&self) -> &'static str {
        match self {
            #[cfg(feature = "serde_json")]
            Format::Json => "json",
            #[cfg(feature = "serde-jsonlines")]
            Format::Jsonl => "jsonl",
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
            #[cfg(feature = "serde-jsonlines")]
            Some("jsonl") => Ok(Format::Jsonl),
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
