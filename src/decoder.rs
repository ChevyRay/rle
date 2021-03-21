use crate::{Index, Table};

/// An iterator that decodes a sequence of runs into a series
/// of `T` values fetched from the table. See [decode](crate::Table::decode).
pub struct Decoder<'a, T>
where
    T: Ord + Clone,
{
    pub(crate) table: &'a Table<T>,
    pub(crate) runs: &'a [(Index, usize)],
    pub(crate) run: Option<(Index, usize)>,
}

impl<'a, T> Iterator for Decoder<'a, T>
where
    T: Ord + Clone,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.run
            .take()
            .or_else(|| {
                self.runs.get(0).and_then(|&run| {
                    self.runs = &self.runs[1..];
                    Some(run)
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
