use crate::{Index, Table};
use std::cmp::Ordering;

/// An iterator that run-length encodes a sequence of `T` values
/// into a series of runs. See [encode](crate::Table::encode).
pub struct Encoder<'a, T> {
    pub(crate) table: &'a Table<T>,
    pub(crate) items: &'a [T],
    pub(crate) index: usize,
}

impl<'a, T> Iterator for Encoder<'a, T>
where
    T: Ord + Clone,
{
    type Item = (Index, usize);

    fn next(&mut self) -> Option<Self::Item> {
        (self.index < self.items.len()).then(|| {
            let ind = self.index;
            let mut len = 1;
            while ind + len < self.items.len()
                && self.items[ind].cmp(&self.items[ind + len]) == Ordering::Equal
            {
                len += 1;
            }
            self.index += len;
            let ind = self.table.get_index(&self.items[ind]).unwrap();
            (ind, len)
        })
    }
}
