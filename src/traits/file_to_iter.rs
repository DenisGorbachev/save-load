use serde::de::DeserializeOwned;
use std::fs::File;

pub trait FileToIter {
    type Output<T>: Iterator<Item = Result<T, Self::ItemError>>
    where
        T: DeserializeOwned + 'static;
    type Error;
    type ItemError;

    fn file_to_iter<T>(&self, file: &File) -> Result<Self::Output<T>, Self::Error>
    where
        T: DeserializeOwned + 'static;
}
