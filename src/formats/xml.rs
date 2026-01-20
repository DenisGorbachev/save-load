/// XML does not support  [`FileToIterator`](crate::FileToIterator) or [`IteratorToFile`](crate::IteratorToFile)
/// because the format requires an explicit container element and schema
/// assumptions that are out of scope for the generic iterator conversions.
#[derive(Default, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Debug)]
pub struct Xml;
