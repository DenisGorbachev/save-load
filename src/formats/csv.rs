use crate::{FileToIter, FileToIterOfResults, IterToFile};
use errgonomic::ErrVec;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::borrow::Borrow;
use std::fs::File;
use thiserror::Error;

#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct Csv;

impl IterToFile for Csv {
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
        errgonomic::handle!(result, SerializeFailed);
        errgonomic::handle!(writer.flush(), FlushFailed);
        Ok(())
    }
}

impl FileToIter for Csv {
    type Output<T>
        = std::vec::IntoIter<T>
    where
        T: DeserializeOwned + 'static;
    type Error = CsvFileToIterError;

    fn file_to_iter<T>(&self, file: &File) -> Result<<Self as FileToIter>::Output<T>, <Self as FileToIter>::Error>
    where
        T: DeserializeOwned + 'static,
    {
        use CsvFileToIterError::*;
        let iter = errgonomic::handle!(<Self as FileToIterOfResults>::file_to_iter_of_results(self, file), FileToIterOfResultsFailed);
        let items = errgonomic::handle_iter!(iter, DeserializeFailed);
        Ok(items.into_iter())
    }
}

impl FileToIterOfResults for Csv {
    type Output<T>
        = csv::DeserializeRecordsIntoIter<File, T>
    where
        T: DeserializeOwned + 'static;
    type Error = CsvFileToIterOfResultsError;
    type ItemError = csv::Error;

    fn file_to_iter_of_results<T>(&self, file: &File) -> Result<<Self as FileToIterOfResults>::Output<T>, <Self as FileToIterOfResults>::Error>
    where
        T: DeserializeOwned + 'static,
    {
        use CsvFileToIterOfResultsError::*;
        let file_owned = errgonomic::handle!(file.try_clone(), TryCloneFailed);
        let mut reader = csv::Reader::from_reader(file_owned);
        let _headers = errgonomic::handle!(reader.headers(), HeadersFailed);
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
    #[error("failed to clone CSV file handle")]
    TryCloneFailed { source: std::io::Error },
    #[error("failed to read CSV headers")]
    HeadersFailed { source: csv::Error },
}
