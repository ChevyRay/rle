use std::fmt::{Display, Formatter, Result};

/// A failure to encode.
#[derive(Debug, Clone)]
pub enum Error {
    /// Failed to encode as bytes because the table exceeded 127 items.
    ///
    /// The contained value is the size of the table.
    TableTooLarge(usize),

    /// Failed to encode because an item was not found in the table.
    ///
    /// The contained value is the index of the offending item in the
    /// slice that was being encoded.
    TableMissingItems(usize),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::TableTooLarge(size) => write!(f, "Table size is {}, which exceeds the maximum for encoding as bytes (must be <=127 items)", size),
            Self::TableMissingItems(index) => write!(f, "Cannot encode because item located at [{}] is not in the Table.", index),
        }
    }
}
