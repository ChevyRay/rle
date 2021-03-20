use crate::bytes_decoder::BytesDecoder;
use crate::decoder::Decoder;
use crate::Error;
use crate::{BytesEncoder, Encoder, EncoderMut, Index};
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::marker::PhantomData;
use std::ops::Deref;
use std::slice::SliceIndex;

/// A table to store items encoded into run-length format.
#[derive(Default, Clone, Debug)]
pub struct Table<T> {
    items: Vec<T>,
}

impl<T> Table<T>
where
    T: Ord + Clone,
{
    /// Constructs a new, empty `Table<T>` with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
        }
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
    }

    pub(crate) fn insert_or_get(&mut self, item: &T) -> usize {
        match self.items.binary_search(item) {
            Ok(i) => i,
            Err(i) => {
                self.items.insert(i, item.clone());
                i
            }
        }
    }

    /// Inserts the item into the table. Tables only contain unique
    /// values, so if the item is already in the table, it will not
    /// add a duplicate.
    pub fn insert(&mut self, item: &T) {
        self.insert_or_get(item);
    }

    pub(crate) fn get_index(&self, item: &T) -> Option<usize> {
        self.items.binary_search(item).ok()
    }

    /// Extend the table with the contents of the iterator.
    pub fn extend<'a, I>(&'a mut self, items: I)
    where
        I: Iterator<Item = &'a T>,
    {
        for item in items {
            self.insert(&item);
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
            if let Err(_) = self.items.binary_search(&items[i]) {
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

    pub fn decode<'a>(&'a self, runs: &'a [(Index, usize)]) -> Decoder<T> {
        Decoder {
            table: self,
            runs,
            run: None,
        }
    }
    pub fn decode_bytes<'a>(&'a self, bytes: &'a [u8]) -> BytesDecoder<T> {
        BytesDecoder {
            table: self,
            bytes,
            run: None,
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

#[cfg(feature = "serde")]
impl<T> Serialize for Table<T>
where
    Vec<T>: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        self.items.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, T> Deserialize<'de> for Table<T>
where
    Vec<T>: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self {
            items: Vec::<T>::deserialize(deserializer)?,
        })
    }
}