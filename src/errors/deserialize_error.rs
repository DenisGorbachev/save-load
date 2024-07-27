use derive_more::{Display, Error, From};

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
    #[cfg(feature = "csv")]
    Csv(csv::Error),
    #[cfg(feature = "csv")]
    CsvRowNotFound(crate::errors::record_not_found_error::CsvRowNotFoundError),
}
