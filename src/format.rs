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
}
