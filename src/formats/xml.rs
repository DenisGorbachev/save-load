use crate::{FileToIter, FileToIterOfResults, IterToFile};
use core::convert::Infallible;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::borrow::Borrow;
use std::fs::File;
use thiserror::Error;

#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct Xml;

impl IterToFile for Xml {
    type Output = ();
    type Error = XmlIterToFileError;

    fn iter_to_file<T, I>(&self, _file: &File, _iter: I) -> Result<Self::Output, Self::Error>
    where
        T: Serialize,
        I: IntoIterator,
        I::Item: Borrow<T>,
    {
        use XmlIterToFileError::*;
        Err(IterToFileFailed {})
    }
}

impl FileToIter for Xml {
    type Output<T>
        = std::vec::IntoIter<T>
    where
        T: DeserializeOwned + 'static;
    type Error = XmlFileToIterError;

    fn file_to_iter<T>(&self, _file: &File) -> Result<<Self as FileToIter>::Output<T>, <Self as FileToIter>::Error>
    where
        T: DeserializeOwned + 'static,
    {
        use XmlFileToIterError::*;
        Err(FileToIterFailed {})
    }
}

impl FileToIterOfResults for Xml {
    type Output<T>
        = std::iter::Map<std::vec::IntoIter<T>, fn(T) -> Result<T, Self::ItemError>>
    where
        T: DeserializeOwned + 'static;
    type Error = XmlFileToIterOfResultsError;
    type ItemError = Infallible;

    fn file_to_iter_of_results<T>(&self, _file: &File) -> Result<<Self as FileToIterOfResults>::Output<T>, <Self as FileToIterOfResults>::Error>
    where
        T: DeserializeOwned + 'static,
    {
        use XmlFileToIterOfResultsError::*;
        Err(FileToIterOfResultsFailed {})
    }
}

#[derive(Error, Debug)]
pub enum XmlIterToFileError {
    #[error("XML does not support serializing iterator payloads")]
    IterToFileFailed {},
}

#[derive(Error, Debug)]
pub enum XmlFileToIterError {
    #[error("XML does not support deserializing iterator payloads")]
    FileToIterFailed {},
}

#[derive(Error, Debug)]
pub enum XmlFileToIterOfResultsError {
    #[error("XML does not support deserializing iterator payloads")]
    FileToIterOfResultsFailed {},
}
