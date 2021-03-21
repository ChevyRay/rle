use crate::{Index, Table};

/// An iterator that decodes a run-length encoded sequence of bytes into
/// a series of `T` values fetched from the table. See [decode_bytes](crate::Table::decode_bytes).
pub struct BytesDecoder<'a, T>
where
    T: Ord + Clone,
{
    pub(crate) table: &'a Table<T>,
    pub(crate) bytes: &'a [u8],
    pub(crate) run: Option<(Index, usize)>,
}

impl<'a, T> Iterator for BytesDecoder<'a, T>
where
    T: Ord + Clone,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.run
            .take()
            .or_else(|| {
                self.bytes.get(0).and_then(|&ind| {
                    self.bytes = &self.bytes[1..];
                    let ind = ind as usize;
                    if (ind & 1) == 1 {
                        self.bytes.get(0).and_then(|&len| {
                            self.bytes = &self.bytes[1..];
                            Some((ind >> 1, len as usize))
                        })
                    } else {
                        Some((ind >> 1, 1))
                    }
                })
            })
            .and_then(|(ind, len)| {
                if len > 1 {
                    self.run = Some((ind, len - 1));
                }
                self.table.get(ind)
            })
    }
}
