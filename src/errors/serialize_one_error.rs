use derive_more::{Display, Error, From};

#[derive(Error, Display, From, Debug)]
#[non_exhaustive]
pub enum SerializeOneError {
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
    Toml(toml::ser::Error),
    #[cfg(feature = "csv")]
    Csv(csv::Error),
    #[cfg(feature = "csv")]
    FromUtf8(std::string::FromUtf8Error),
}
