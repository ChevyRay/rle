use crate::{
    BytesDecoder, BytesEncoder, BytesEncoderMut, Decoder, Encoder, EncoderMut, Error, Index,
};
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::ops::Deref;
use std::slice::SliceIndex;

/// A table to store items to be encoded into run-length format.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Table<T> {
    /// This is a list of the items in the order they were added,
    /// their positions in this list will not ever change.
    items: Vec<T>,

    /// This is a sorted list of the items (usize maps to the item's
    /// index in `items`) for fast lookup/retrieval when encoding.
    sorted: Vec<usize>,
}

impl<T> Default for Table<T> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            sorted: Vec::new(),
        }
    }
}

impl<T> Table<T>
where
    T: Ord + Clone,
{
    /// Constructs a new, empty table with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
            sorted: Vec::with_capacity(capacity),
        }
    }

    /// Constructs a new table with items collected from an iterator.
    pub fn from_iter<I: Iterator<Item = T>>(iter: I) -> Self {
        let mut table = Self::default();
        table.extend(iter);
        table
    }

    /// Constructs a new table with items from a slice.
    pub fn from_slice(slice: &[T]) -> Self {
        let mut table = Self::default();
        table.extend_from_slice(slice);
        table
    }

    /// The number of unique elements in the table.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns a reference to an item or subslice depending on the type of index.
    ///
    /// - If given a position, returns a reference to the item at that
    ///   position or `None` if out of bounds.
    /// - If given a range, returns the subslice corresponding to that range,
    ///   or `None` if out of bounds.
    pub fn get<I>(&self, index: I) -> Option<&I::Output>
    where
        I: SliceIndex<[T]>,
    {
        self.items.get(index)
    }

    /// Clears the table, removing all items.
    ///
    /// Note that this method has no effect on the allocated capacity
    /// of the table.
    pub fn clear(&mut self) {
        self.items.clear();
        self.sorted.clear();
    }

    pub(crate) fn insert_or_get(&mut self, item: &T) -> usize {
        match self.sorted.binary_search_by(|&i| self.items[i].cmp(item)) {
            Ok(i) => self.sorted[i],
            Err(i) => {
                let ind = self.items.len();
                self.items.push(item.clone());
                self.sorted.insert(i, ind);
                ind
            }
        }
        /*self.sorted
        .binary_search_by(|&ind| self.items[ind].cmp(item))
        .and_then(|i| Ok(self.sorted[i]))
        .unwrap_or_else(|i| {
            let ind = self.items.len();
            self.items.push(item.clone());
            self.sorted.insert(i, ind);
            ind
        })*/
    }

    /// Inserts the item into the table. Tables only contain unique
    /// values, so if the item is already in the table, it will not
    /// add a duplicate.
    pub fn insert(&mut self, item: T) {
        self.insert_or_get(&item);
    }

    pub(crate) fn get_index(&self, item: &T) -> Option<usize> {
        self.sorted
            .binary_search_by(|&i| self.items[i].cmp(item))
            .ok()
            .and_then(|i| Some(self.sorted[i]))
    }

    /// Extend the table with the contents of an iterator.
    pub fn extend<I>(&mut self, items: I)
    where
        I: Iterator<Item = T>,
    {
        for item in items {
            self.insert_or_get(&item);
        }
    }

    /// Extend the table with the contents of a slice.
    pub fn extend_from_slice(&mut self, items: &[T]) {
        for item in items {
            self.insert_or_get(&item);
        }
    }

    /// Returns an iterator to run-length encode the items.
    ///
    /// Unlike [encode](Table<T>::encode), this method will not fail because
    /// it will add items to the table as they were found, resulting in a table
    /// that contains one of every item encountered in the encoded slice.
    pub fn encode_mut<'a>(&'a mut self, items: &'a [T]) -> EncoderMut<T> {
        EncoderMut {
            table: self,
            items,
            index: 0,
        }
    }

    /// Returns an iterator to run-length encode the items,
    /// using this table as a lookup.
    ///
    /// # Errors
    ///
    /// If `items` contains any elements not found in the table, this method
    /// will return a [TableMissingItems](Error::TableMissingItems) error.
    pub fn encode<'a>(&'a self, items: &'a [T]) -> Result<Encoder<T>, Error> {
        // Fail if any of the items are not in the table
        for i in 0..items.len() {
            if let None = self.get_index(&items[i]) {
                return Err(Error::TableMissingItems(i));
            }
        }
        Ok(Encoder {
            table: self,
            items,
            index: 0,
        })
    }

    /// Returns an iterator to run-length encode the items as a sequence of bytes.
    ///
    /// # Format
    ///
    /// You can use the [decode_bytes](Table<T>::decode_bytes) method to decode these
    /// byte sequences. But if you have to write a parser from another client or language,
    /// the format of the bytes can be parsed as follows:
    ///
    /// ```
    /// # use rle::*;
    /// # fn stream_has_bytes() -> bool { true }
    /// # fn read_next_byte() -> Option<u8> { None }
    /// fn read_run_length() -> Option<(usize, usize)> {
    ///     // First, we read a byte from the stream.
    ///     let byte: u8 = read_next_byte()?;
    ///
    ///     // If the first bit is set, we read the next byte to get how
    ///     // many times to repeat the item in sequence (the "run-length").
    ///     // If the bit is not set, this is a run of just 1 item, and the
    ///     // next byte is the next new item instead of the run-length.
    ///     let length: usize = if (byte & 1) == 1 {
    ///         read_next_byte()? as usize
    ///     } else {
    ///         1
    ///     };
    ///
    ///     // To get the index of the item in the table, we then shift by 1.
    ///     let index: usize = (byte >> 1) as usize;
    ///     
    ///     Some((index, length))
    /// }
    ///
    /// # let table = Table::<char>::default();
    /// # let mut results = Vec::<char>::new();
    /// // Now, as long as the table matches the one the bytes were encoded
    /// // with, we can then rebuild the sequence of items...
    /// while let Some((index, length)) = read_run_length() {
    ///     let item = table[index];
    ///     for _ in 0..length {
    ///         results.push(item);
    ///     }
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Because the index of each item is stored in 7 bits, this format
    /// only works for tables up to 127 items in length.
    ///
    /// If the provided table contains >= 128 items, this will return a
    /// [TableTooLarge](Error::TableTooLarge) error.
    pub fn encode_bytes<'a>(&'a self, items: &'a [T]) -> Result<BytesEncoder<T>, Error> {
        if self.items.len() < 128 {
            Ok(BytesEncoder {
                rle: self.encode(items)?,
                run: None,
                len: None,
            })
        } else {
            Err(Error::TableTooLarge(self.items.len()))
        }
    }

    /// Returns an iterator to run-length encode the items as a sequence of bytes.
    ///
    /// Unlike [encode_bytes](Table<T>::encode_bytes), this method will add items
    /// to the table as they were found, resulting in a table that contains one of
    /// every item encountered in the encoded slice.
    pub fn encode_bytes_mut<'a>(&'a mut self, items: &'a [T]) -> Result<BytesEncoderMut<T>, Error> {
        if self.items.len() < 128 {
            Ok(BytesEncoderMut {
                rle: self.encode_mut(items),
                run: None,
                len: None,
            })
        } else {
            Err(Error::TableTooLarge(self.items.len()))
        }
    }

    pub fn encode_hex_str<'a>(&'a self, items: &'a [T]) -> Result<String, Error> {
        let mut str = String::new();
        for (ind, len) in self.encode(items)? {
            write!(str, "{:X}:{:X},", ind, len).unwrap();
        }
        Ok(str)
    }

    /// Return an iterator that decodes the series of runs using this table
    /// as the index lookup for the elements.
    pub fn decode<'a>(&'a self, runs: &'a [(Index, usize)]) -> Decoder<T> {
        Decoder {
            table: self,
            runs,
            run: None,
        }
    }

    /// Return an iterator that decodes the run-length encoded bytes using
    /// this table as the index lookup for the elements.
    pub fn decode_bytes<'a>(&'a self, bytes: &'a [u8]) -> BytesDecoder<T> {
        BytesDecoder {
            table: self,
            bytes,
            run: None,
        }
    }

    pub fn iter(&self) -> TableIter<T> {
        TableIter { items: &self.items }
    }

    pub fn iter_sorted(&self) -> SortedTableIter<T> {
        SortedTableIter {
            items: &self.items,
            sorted: &self.sorted,
        }
    }
}

impl<T> AsRef<[T]> for Table<T> {
    fn as_ref(&self) -> &[T] {
        &self.items
    }
}

impl<T> Deref for Table<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

pub struct TableIter<'a, T> {
    items: &'a [T],
}

impl<'a, T> Iterator for TableIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.items.get(0).and_then(|item| {
            self.items = &self.items[1..];
            Some(item)
        })
    }
}

pub struct SortedTableIter<'a, T> {
    items: &'a [T],
    sorted: &'a [usize],
}

impl<'a, T> Iterator for SortedTableIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.sorted.get(0).and_then(|&i| {
            self.sorted = &self.sorted[1..];
            Some(&self.items[i])
        })
    }
}
