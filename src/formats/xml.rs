use crate::{FileToIter, IterToFile};
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
        = Box<dyn Iterator<Item = Result<T, Self::ItemError>>>
    where
        T: DeserializeOwned + 'static;
    type Error = XmlFileToIterError;
    type ItemError = Infallible;

    fn file_to_iter<T>(&self, _file: &File) -> Result<Self::Output<T>, Self::Error>
    where
        T: DeserializeOwned + 'static,
    {
        use XmlFileToIterError::*;
        Err(FileToIterFailed {})
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
