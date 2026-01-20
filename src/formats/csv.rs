use crate::{FileToIterator, FileToIteratorOfResults, IteratorToFile};
use errgonomic::{handle, handle_iter, ErrVec};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::borrow::Borrow;
use std::fs::File;
use thiserror::Error;

#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct Csv;

impl IteratorToFile for Csv {
    type Output = ();
    type Error = CsvIterToFileError;

    fn iter_to_file<T, I>(&self, file: &File, iter: I) -> Result<Self::Output, Self::Error>
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
        handle!(result, SerializeFailed);
        handle!(writer.flush(), FlushFailed);
        Ok(())
    }
}

impl FileToIterator for Csv {
    type Output<T>
        = std::vec::IntoIter<T>
    where
        T: DeserializeOwned + 'static;
    type Error = CsvFileToIterError;

    fn file_to_iter<T>(&self, file: File) -> Result<<Self as FileToIterator>::Output<T>, <Self as FileToIterator>::Error>
    where
        T: DeserializeOwned + 'static,
    {
        use CsvFileToIterError::*;
        let iter = handle!(self.file_to_iter_of_results(file), FileToIterOfResultsFailed);
        let items = handle_iter!(iter, DeserializeFailed);
        Ok(items.into_iter())
    }
}

impl FileToIteratorOfResults for Csv {
    type Output<T>
        = csv::DeserializeRecordsIntoIter<File, T>
    where
        T: DeserializeOwned + 'static;
    type Error = CsvFileToIterOfResultsError;
    type ItemError = csv::Error;

    fn file_to_iter_of_results<T>(&self, file: File) -> Result<<Self as FileToIteratorOfResults>::Output<T>, <Self as FileToIteratorOfResults>::Error>
    where
        T: DeserializeOwned + 'static,
    {
        use CsvFileToIterOfResultsError::*;
        let mut reader = csv::Reader::from_reader(file);
        let _headers = handle!(reader.headers(), HeadersFailed);
        let iter = reader.into_deserialize::<T>();
        Ok(iter)
    }
}

#[derive(Error, Debug)]
pub enum CsvIterToFileError {
    #[error("failed to serialize CSV item")]
    SerializeFailed { source: csv::Error },
    #[error("failed to flush CSV writer")]
    FlushFailed { source: std::io::Error },
}

#[derive(Error, Debug)]
pub enum CsvFileToIterError {
    #[error("failed to create CSV iterator")]
    FileToIterOfResultsFailed { source: CsvFileToIterOfResultsError },
    #[error("failed to deserialize CSV items")]
    DeserializeFailed { source: ErrVec<csv::Error> },
}

#[derive(Error, Debug)]
pub enum CsvFileToIterOfResultsError {
    #[error("failed to read CSV headers")]
    HeadersFailed { source: csv::Error },
}
