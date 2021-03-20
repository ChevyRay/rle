mod bytes_decoder;
mod bytes_encoder;
mod decoder;
mod encoder;
mod encoder_mut;
mod error;
mod table;

pub type Index = usize;

pub use bytes_encoder::BytesEncoder;
pub use decoder::Decoder;
pub use encoder::Encoder;
pub use encoder_mut::EncoderMut;
pub use error::Error;
pub use table::Table;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {

        /*let str = "AAAABBBABBBABBBBBAAACCC";
        println!("{}", str);
        let chars: Vec<char> = str.chars().collect();

        let mut table = Table::default();
        table.extend(chars.iter());

        let bytes: Vec<u8> = table.encode_bytes(&chars).unwrap().collect();

        for chr in table.decode_bytes(&bytes) {
            print!("{}", chr);
        }
        println!();*/
    }
}
