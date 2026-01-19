use crate::{FileToIter, FileToIterOfResults, IterToFile};
use core::convert::Infallible;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::borrow::Borrow;
use std::fs::File;
use thiserror::Error;

#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct Json;

impl IterToFile for Json {
    type Output = ();
    type Error = JsonIterToFileError;

    fn iter_to_file<T, I>(&self, file: &File, iter: I) -> Result<Self::Output, Self::Error>
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

impl FileToIter for Json {
    type Output<T>
        = std::vec::IntoIter<T>
    where
        T: DeserializeOwned + 'static;
    type Error = JsonFileToIterError;

    fn file_to_iter<T>(&self, file: &File) -> Result<<Self as FileToIter>::Output<T>, <Self as FileToIter>::Error>
    where
        T: DeserializeOwned + 'static,
    {
        use JsonFileToIterError::*;
        let items = errgonomic::handle!(serde_json::from_reader::<_, Vec<T>>(file), FromReaderFailed);
        Ok(items.into_iter())
    }
}

impl FileToIterOfResults for Json {
    type Output<T>
        = std::iter::Map<std::vec::IntoIter<T>, fn(T) -> Result<T, Self::ItemError>>
    where
        T: DeserializeOwned + 'static;
    type Error = JsonFileToIterOfResultsError;
    type ItemError = Infallible;

    fn file_to_iter_of_results<T>(&self, file: &File) -> Result<<Self as FileToIterOfResults>::Output<T>, <Self as FileToIterOfResults>::Error>
    where
        T: DeserializeOwned + 'static,
    {
        use JsonFileToIterOfResultsError::*;
        let iter = errgonomic::handle!(self.file_to_iter(file), FileToIterFailed);
        let iter = iter.map(Ok::<T, Infallible> as fn(T) -> Result<T, Infallible>);
        Ok(iter)
    }
}

#[derive(Error, Debug)]
pub enum JsonIterToFileError {
    #[error("failed to start JSON sequence serialization")]
    SerializeSeqFailed { source: serde_json::Error },
    #[error("failed to serialize JSON sequence item")]
    SerializeElementFailed { source: serde_json::Error },
    #[error("failed to end JSON sequence serialization")]
    SerializeSeqEndFailed { source: serde_json::Error },
}

#[derive(Error, Debug)]
pub enum JsonFileToIterError {
    #[error("failed to deserialize JSON sequence from reader")]
    FromReaderFailed { source: serde_json::Error },
}

#[derive(Error, Debug)]
pub enum JsonFileToIterOfResultsError {
    #[error("failed to deserialize JSON sequence from reader")]
    FileToIterFailed { source: JsonFileToIterError },
}
