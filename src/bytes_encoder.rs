use crate::Encoder;

/// An iterator that run-length encodes a sequence of `T` values
/// into a compressed byte format. See [encode_bytes](crate::Table::encode_bytes).
pub struct BytesEncoder<'a, T> {
    pub(crate) rle: Encoder<'a, T>,
    pub(crate) run: Option<(u8, usize)>,
    pub(crate) len: Option<u8>,
}

impl<'a, T> Iterator for BytesEncoder<'a, T>
where
    T: Ord + Clone,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(len) = self.len.take() {
            Some(len)
        } else if let Some((ind, len)) = self.run.take().or_else(|| {
            self.rle
                .next()
                .and_then(|(ind, len)| Some((ind as u8, len)))
        }) {
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
            Some(ind_bits)
        } else {
            None
        }
    }
}
