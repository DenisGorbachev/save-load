use std::borrow::Borrow;
use std::fs::File;

pub trait IterToFile {
    type Output;
    type Error;

    fn iter_to_file<T, I>(&self, file: &File, iter: I) -> Result<Self::Output, Self::Error>
    where
        I: IntoIterator,
        I::Item: Borrow<T>;
}
