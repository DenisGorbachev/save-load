use std::fs::File;

pub trait FileToIter {
    type Output;
    type Error;

    fn file_to_iter<T, I>(&self, file: &File) -> Result<I, Self::Error>
    where
        I: Iterator<Item = T>;
}
