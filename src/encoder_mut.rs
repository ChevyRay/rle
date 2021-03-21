use crate::{Index, Table};
use std::cmp::Ordering;

/// An iterator that run-length encodes a sequence of `T` values
/// into a series of runs, and also adds elements to the table as
/// it encounters them. See [encode_mut](crate::Table::encode_mut).
pub struct EncoderMut<'a, T> {
    pub(crate) table: &'a mut Table<T>,
    pub(crate) items: &'a [T],
    pub(crate) index: usize,
}

impl<'a, T> Iterator for EncoderMut<'a, T>
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
            let ind = self.table.insert_or_get(&self.items[ind]);
            (ind, len)
        })
    }
}
