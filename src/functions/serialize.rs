use derive_more::{Display, Error, From};
use serde::Serialize;

use crate::format::Format;

#[allow(unreachable_patterns, unused_variables, unreachable_code)]
pub fn serialize<T: Serialize>(input: &T, format: Format) -> Result<String, SerializeError> {
    Ok(match format {
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
    })
}

#[derive(Error, Display, From, Debug)]
#[non_exhaustive]
pub enum SerializeError {
    #[cfg(feature = "serde_json")]
    SerdeJson(serde_json::Error),
    #[cfg(feature = "serde_yaml")]
    SerdeYaml(serde_yaml::Error),
    #[cfg(feature = "serde-xml-rs")]
    SerdeXmlRs(serde_xml_rs::Error),
    #[cfg(feature = "quick-xml")]
    QuickXml(quick_xml::DeError),
    #[cfg(feature = "toml")]
    Toml(toml::ser::Error),
}
