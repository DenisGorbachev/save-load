use derive_more::{Display, Error, From};

#[derive(Error, Display, From, Debug)]
#[non_exhaustive]
pub enum DeserializeOneError {
    #[cfg(feature = "serde_json")]
    SerdeJson(serde_json::Error),
    #[cfg(feature = "serde-jsonlines")]
    SerdeJsonlines(std::io::Error),
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
    ItemNotFound(crate::errors::item_not_found_error::ItemNotFoundError),
}
