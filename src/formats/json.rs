#[cfg(feature = "serde_json")]
use crate::{FileToIter, IterToFile};
#[cfg(feature = "serde_json")]
use core::convert::Infallible;
#[cfg(feature = "serde_json")]
use serde::de::DeserializeOwned;
#[cfg(feature = "serde_json")]
use serde::Serialize;
#[cfg(feature = "serde_json")]
use std::borrow::Borrow;
#[cfg(feature = "serde_json")]
use std::fs::File;
#[cfg(feature = "serde_json")]
use thiserror::Error;

#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct Json;

#[cfg(feature = "serde_json")]
impl IterToFile for Json {
    type Output = ();
    type Error = JsonIterToFileError;

    fn iter_to_file<T, I>(&self, file: &File, iter: I) -> Result<<Self as IterToFile>::Output, <Self as IterToFile>::Error>
    where
        T: Serialize,
        I: IntoIterator,
        I::Item: Borrow<T>,
    {
        use serde::ser::{SerializeSeq, Serializer};
        use JsonIterToFileError::*;
        let mut serializer = serde_json::Serializer::new(file);
        let mut seq = errgonomic::handle!(serializer.serialize_seq(None), SerializeSeqFailed);
        let result = iter
            .into_iter()
            .try_for_each(|item| seq.serialize_element(item.borrow()));
        errgonomic::handle!(result, SerializeElementFailed);
        errgonomic::handle!(seq.end(), SerializeSeqEndFailed);
        Ok(())
    }
}

#[cfg(feature = "serde_json")]
impl FileToIter for Json {
    type Output<T>
        = Box<dyn Iterator<Item = Result<T, Self::ItemError>>>
    where
        T: DeserializeOwned + 'static;
    type Error = JsonFileToIterError;
    type ItemError = Infallible;

    fn file_to_iter<T>(&self, file: &File) -> Result<<Self as FileToIter>::Output<T>, <Self as FileToIter>::Error>
    where
        T: DeserializeOwned + 'static,
    {
        use JsonFileToIterError::*;
        let items = errgonomic::handle!(serde_json::from_reader::<_, Vec<T>>(file), FromReaderFailed);
        let iter = items.into_iter().map(Ok);
        Ok(Box::new(iter))
    }
}

#[cfg(feature = "serde_json")]
#[derive(Error, Debug)]
pub enum JsonIterToFileError {
    #[error("failed to start JSON sequence serialization")]
    SerializeSeqFailed { source: serde_json::Error },
    #[error("failed to serialize JSON sequence item")]
    SerializeElementFailed { source: serde_json::Error },
    #[error("failed to end JSON sequence serialization")]
    SerializeSeqEndFailed { source: serde_json::Error },
}

#[cfg(feature = "serde_json")]
#[derive(Error, Debug)]
pub enum JsonFileToIterError {
    #[error("failed to deserialize JSON sequence from reader")]
    FromReaderFailed { source: serde_json::Error },
}
