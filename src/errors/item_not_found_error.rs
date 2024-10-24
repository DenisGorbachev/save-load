use derive_more::{Display, Error};

#[derive(Error, Display, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct ItemNotFoundError;
