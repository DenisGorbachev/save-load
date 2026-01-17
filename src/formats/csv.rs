#[cfg(feature = "csv")]
use crate::{FileToIter, IterToFile};
#[cfg(feature = "csv")]
use serde::de::DeserializeOwned;
#[cfg(feature = "csv")]
use serde::Serialize;
#[cfg(feature = "csv")]
use std::borrow::Borrow;
#[cfg(feature = "csv")]
use std::fs::File;
#[cfg(feature = "csv")]
use thiserror::Error;

#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct Csv;

#[cfg(feature = "csv")]
impl IterToFile for Csv {
    type Output = ();
    type Error = CsvIterToFileError;

    fn iter_to_file<T, I>(&self, file: &File, iter: I) -> Result<<Self as IterToFile>::Output, <Self as IterToFile>::Error>
    where
        T: Serialize,
        I: IntoIterator,
        I::Item: Borrow<T>,
    {
        use CsvIterToFileError::*;
        let mut writer = csv::Writer::from_writer(file);
        let result = iter
            .into_iter()
            .try_for_each(|item| writer.serialize(item.borrow()));
        errgonomic::handle!(result, SerializeFailed);
        errgonomic::handle!(writer.flush(), FlushFailed);
        Ok(())
    }
}

#[cfg(feature = "csv")]
impl FileToIter for Csv {
    type Output<T>
        = Box<dyn Iterator<Item = Result<T, Self::ItemError>>>
    where
        T: DeserializeOwned + 'static;
    type Error = CsvFileToIterError;
    type ItemError = CsvFileToIterItemError;

    fn file_to_iter<T>(&self, file: &File) -> Result<<Self as FileToIter>::Output<T>, <Self as FileToIter>::Error>
    where
        T: DeserializeOwned + 'static,
    {
        use CsvFileToIterError::*;
        use CsvFileToIterItemError::*;
        let file_owned = errgonomic::handle!(file.try_clone(), TryCloneFailed);
        let mut reader = csv::Reader::from_reader(file_owned);
        let _headers = errgonomic::handle!(reader.headers(), HeadersFailed);
        let iter = reader
            .into_deserialize::<T>()
            .map(|result| errgonomic::map_err!(result, DeserializeFailed));
        Ok(Box::new(iter))
    }
}

#[cfg(feature = "csv")]
#[derive(Error, Debug)]
pub enum CsvIterToFileError {
    #[error("failed to serialize CSV item")]
    SerializeFailed { source: csv::Error },
    #[error("failed to flush CSV writer")]
    FlushFailed { source: std::io::Error },
}

#[cfg(feature = "csv")]
#[derive(Error, Debug)]
pub enum CsvFileToIterError {
    #[error("failed to clone CSV file handle")]
    TryCloneFailed { source: std::io::Error },
    #[error("failed to read CSV headers")]
    HeadersFailed { source: csv::Error },
}

#[cfg(feature = "csv")]
#[derive(Error, Debug)]
pub enum CsvFileToIterItemError {
    #[error("failed to deserialize CSV item")]
    DeserializeFailed { source: csv::Error },
}
