use crate::{EncoderMut, Error};

/// An iterator that run-length encodes a sequence of `T` values
/// into a compressed byte format, and also adds elements to the
/// table as it encounters them. See [encode_bytes_mut](crate::Table::encode_bytes_mut).
pub struct BytesEncoderMut<'a, T> {
    pub(crate) rle: EncoderMut<'a, T>,
    pub(crate) run: Option<(u8, usize)>,
    pub(crate) len: Option<u8>,
}

impl<'a, T> Iterator for BytesEncoderMut<'a, T>
where
    T: Ord + Clone,
{
    type Item = Result<u8, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(len) = self.len.take() {
            Some(Ok(len))
        } else {
            (if let Some((ind, len)) = self.run.take() {
                Some((ind, len))
            } else if let Some((ind, len)) = self.rle.next() {
                if ind < 128 {
                    Some((ind as u8, len))
                } else {
                    return Some(Err(Error::TableTooLarge(self.rle.table.len())));
                }
            } else {
                None
            })
            .and_then(|(ind, len)| {
                let num = len.min(127);
                let ind_bits = if len > 1 {
                    self.len = Some(num as u8);
                    (ind << 1) | 1
                } else {
                    ind << 1
                };
                if len > num {
                    self.run = Some((ind, len - num));
                }
                Some(Ok(ind_bits))
            })
        }
    }
}
