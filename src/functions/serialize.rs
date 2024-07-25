use derive_more::{Display, Error, From};
use serde::Serialize;

use crate::format::Format;

#[allow(unreachable_patterns)]
pub fn serialize<T: Serialize>(#[allow(unused_variables)] value: &T, format: Format) -> Result<String, SerializeError> {
    let output = match format {
        #[cfg(feature = "serde_json")]
        Format::Json => serde_json::to_string_pretty(value)?,
        #[cfg(feature = "serde_yaml")]
        Format::Yaml => serde_yaml::to_string(value)?,
        #[cfg(feature = "serde-xml-rs")]
        Format::Xml => serde_xml_rs::to_string(value)?,
        #[cfg(feature = "quick-xml")]
        Format::Xml => quick_xml::se::to_string(value)?,
        #[cfg(feature = "toml")]
        Format::Toml => toml::to_string(value)?,
        _ => String::new(),
    };
    Ok(output)
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
