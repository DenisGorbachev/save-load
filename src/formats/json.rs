/// JSON does not support [`FileToIterator`](crate::FileToIterator) or [`IteratorToFile`](crate::IteratorToFile)
/// because the format lacks a streaming top-level sequence delimiter and would
/// require buffering the entire payload.
#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct Json;
