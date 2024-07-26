use derive_more::{Display, Error, From};
use serde::de::DeserializeOwned;

use crate::format::Format;

#[allow(unreachable_patterns, unused_variables, unreachable_code)]
pub fn deserialize<T: DeserializeOwned>(input: &str, format: Format) -> Result<T, DeserializeError> {
    Ok(match format {
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
    })
}

#[derive(Error, Display, From, Debug)]
#[non_exhaustive]
pub enum DeserializeError {
    #[cfg(feature = "serde_json")]
    SerdeJson(serde_json::Error),
    #[cfg(feature = "serde_yaml")]
    SerdeYaml(serde_yaml::Error),
    #[cfg(feature = "serde-xml-rs")]
    SerdeXmlRs(serde_xml_rs::Error),
    #[cfg(feature = "quick-xml")]
    QuickXml(quick_xml::DeError),
    #[cfg(feature = "toml")]
    Toml(toml::de::Error),
}
