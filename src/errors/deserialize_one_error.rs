use derive_more::{Display, Error, From};

#[cfg(any(feature = "csv", feature = "serde-jsonlines"))]
use crate::errors::item_not_found_error::ItemNotFoundError;
#[cfg(feature = "serde-jsonlines")]
use std::io;

#[derive(Error, Display, From, Debug)]
#[non_exhaustive]
pub enum DeserializeOneError {
    #[cfg(feature = "serde_json")]
    SerdeJson(serde_json::Error),
    #[cfg(feature = "serde-jsonlines")]
    SerdeJsonlines(io::Error),
    #[cfg(feature = "serde_yaml")]
    SerdeYaml(serde_yaml::Error),
    #[cfg(feature = "serde-xml-rs")]
    SerdeXmlRs(serde_xml_rs::Error),
    #[cfg(feature = "quick-xml")]
    QuickXml(quick_xml::DeError),
    #[cfg(feature = "toml")]
    Toml(toml::de::Error),
    #[cfg(feature = "csv")]
    Csv(csv::Error),
    #[cfg(any(feature = "csv", feature = "serde-jsonlines"))]
    ItemNotFound(ItemNotFoundError),
}
