//! # RLE
//! A rust library for run-length encoded (RLE) sequences.
//!
//! [Run-length encoding on Wikipedia](https://en.wikipedia.org/wiki/Run-length_encoding)
//!
//! # Usage
//!
//! First, you create a [Table](crate::Table), which serves as a lookup table for
//! all the possible values you'll be encoding.
//!
//! ```
//! use rle::Table;
//!
//! let mut table = Table::default();
//! table.extend_from_slice(&['A', 'B', 'C']);
//! ```
//!
//! Now, you can run-length encode sequences of those values...
//!
//! ```
//! # use rle::Table;
//! # let mut table = Table::default();
//! # table.extend_from_slice(&['A', 'B', 'C']);
//! #
//! let str: Vec<char> = "AAAAABBBBBBBBBBCCCAAAAAAAAAA".chars().collect();
//!
//! for (ind, len) in table.encode(&str).unwrap() {
//!     print!("{}{} ", len, table[ind]);
//! }
//! println!();
//!
//! // 5A 10B 3C 10A
//! ```
//!
//! Rather than a series of runs, you can also encode the values into
//! a run-length encoded byte sequence. Here, we are also using [encode_bytes_mut](crate::Table::encode_bytes_mut),
//! instead of [encode_bytes](crate::Table::encode_bytes), which will build the table as it encodes the sequence.
//!
//! ```
//! # use rle::Table;
//! # let mut table = Table::default();
//! # table.extend_from_slice(&['A', 'B', 'C']);
//! #
//! let str: Vec<char> = "AAAAABBBBBBBBBBCCCAAAAAAAAAA".chars().collect();
//!
//! let mut table = Table::default();
//! for byte in table.encode_bytes_mut(&str).unwrap().flatten() {
//!     print!("{:02X} ", byte);
//! }
//! println!();
//!
//! // 01 05 03 0A 05 03 01 0A
//! ```
//!
//! Here's an example of a large sequence full of lots of runs being
//! compressed into an RLE sequence of bytes...
//!
//! ```
//! # use rle::Table;
//! #
//! let input = "................................................................
//! ..........................XXXXXXXXXXXX..........................
//! ......................XXXXXXXXXXXXXXXXXXXX......................
//! ....................XXXXXXXXXXXXXXXXXXXXXXXX....................
//! .................XXXXXXXXXXXXXXXXXXXXXXXXXXXXXX.................
//! ................XXXXXXXXXXX..........XXXXXXXXXXX................
//! ..............XXXXXXXXX..................XXXXXXXXX..............
//! .............XXXXXXX........................XXXXXXX.............
//! ...........XXXXXXX............................XXXXXXX...........
//! ..........XXXXXXX..............................XXXXXXX..........
//! .........XXXXXX..................................XXXXXX.........
//! ........XXXXXX....................................XXXXXX........
//! .......XXXXXX......................................XXXXXX.......
//! .......XXXXX........................................XXXXX.......
//! ......XXXXX..........................................XXXXX......
//! .....XXXXX............................................XXXXX.....
//! .....XXXX..............................................XXXX.....
//! ....XXXXX...........XXXX................XXXX...........XXXXX....
//! ...XXXXX..........XXXXXXXX............XXXXXXXX..........XXXXX...
//! ...XXXX..........XXXXXXXXXX..........XXXXXXXXXX..........XXXX...
//! ...XXXX..........XXXXXXXXXX..........XXXXXXXXXX..........XXXX...
//! ..XXXX..........XXXXXXXXXXXX........XXXXXXXXXXXX..........XXXX..
//! ..XXXX..........XXXXXXXXXXXX........XXXXXXXXXXXX..........XXXX..
//! .XXXXX..........XXXXXXXXXXXX........XXXXXXXXXXXX..........XXXXX.
//! .XXXX...........XXXXXXXXXXXX........XXXXXXXXXXXX...........XXXX.
//! .XXXX............XXXXXXXXXX..........XXXXXXXXXX............XXXX.
//! .XXXX............XXXXXXXXXX..........XXXXXXXXXX............XXXX.
//! XXXXX.............XXXXXXXX............XXXXXXXX.............XXXXX
//! XXXX................XXXX................XXXX................XXXX
//! XXXX........................................................XXXX
//! XXXX........................................................XXXX
//! XXXX........................................................XXXX
//! XXXX........................................................XXXX
//! XXXX........................................................XXXX
//! XXXX........................................................XXXX
//! XXXX........................................................XXXX
//! XXXX........................................................XXXX
//! XXXX........................................................XXXX
//! XXXXX......................................................XXXXX
//! .XXXX......................................................XXXX.
//! .XXXX........XX..................................XX........XXXX.
//! .XXXX........XXX................................XXX........XXXX.
//! .XXXXX.......XXXX..............................XXXX.......XXXXX.
//! ..XXXX........XXXX............................XXXX........XXXX..
//! ..XXXX........XXXXXX........................XXXXXX........XXXX..
//! ...XXXX........XXXXXXXX..................XXXXXXXX........XXXX...
//! ...XXXX.........XXXXXXXXXXX..........XXXXXXXXXXX.........XXXX...
//! ...XXXXX.........XXXXXXXXXXXXXXXXXXXXXXXXXXXXXX.........XXXXX...
//! ....XXXXX...........XXXXXXXXXXXXXXXXXXXXXXXX...........XXXXX....
//! .....XXXX.............XXXXXXXXXXXXXXXXXXXX.............XXXX.....
//! .....XXXXX................XXXXXXXXXXXX................XXXXX.....
//! ......XXXXX..........................................XXXXX......
//! .......XXXXX........................................XXXXX.......
//! .......XXXXXX......................................XXXXXX.......
//! ........XXXXXX....................................XXXXXX........
//! .........XXXXXX..................................XXXXXX.........
//! ..........XXXXXXX..............................XXXXXXX..........
//! ...........XXXXXXX............................XXXXXXX...........
//! .............XXXXXXX........................XXXXXXX.............
//! ..............XXXXXXXXX..................XXXXXXXXX..............
//! ................XXXXXXXXXXX..........XXXXXXXXXXX................
//! .................XXXXXXXXXXXXXXXXXXXXXXXXXXXXXX.................
//! ....................XXXXXXXXXXXXXXXXXXXXXXXX....................
//! ......................XXXXXXXXXXXXXXXXXXXX......................
//! ..........................XXXXXXXXXXXX..........................";
//!
//! let chars: Vec<char> = input.chars().collect();
//!
//! // Create an RLE table with an entry for each unique character
//! let mut table = Table::default();
//!
//! // Encode the ASCII image into a sequence of RLE bytes
//! let encoded: Vec<u8> = table.encode_bytes_mut(&chars).unwrap().flatten().collect();
//!
//! // Decode the bytes back into a string
//! let decoded: String = table.decode_bytes(&encoded).collect();
//!
//! // Make sure we decoded properly
//! assert_eq!(input, decoded);
//!
//! println!("Number of chars in input string ...... {}", chars.len());
//! println!("Number of unique symbols ............. {}", table.len());
//! println!("Number of bytes, encoded ............. {}", encoded.len());
//! println!("Number of chars in decoded string .... {}", decoded.len());
//!
//! // Number of chars in input string ...... 4160
//! // Number of unique symbols ............. 2
//! // Number of bytes, encoded ............. 604
//! // Number of chars in decoded string .... 4160
//! ```

mod bytes_decoder;
mod bytes_encoder;
mod bytes_encoder_mut;
mod decoder;
mod encoder;
mod encoder_mut;
mod error;
mod table;

pub type Index = usize;

pub use bytes_decoder::BytesDecoder;
pub use bytes_encoder::BytesEncoder;
pub use bytes_encoder_mut::BytesEncoderMut;
pub use decoder::Decoder;
pub use encoder::Encoder;
pub use encoder_mut::EncoderMut;
pub use error::Error;
pub use table::Table;

#[cfg(test)]
mod tests {
    use crate::*;
    use std::time::Instant;

    #[test]
    fn hex_str() {
        let str = "GGGGJJJJEEEEIIIIIIIAAAACCCCCCCCAAABBBBXXXXXRRRRRRRRR";
        println!("STR: {}", str);

        let str: Vec<char> = str.chars().collect();

        let table = Table::from_slice(&str);
        println!("{:?}", table.as_ref());
        println!("{:?}", table.iter_sorted().copied().collect::<Vec<char>>());

        let str = table.encode_hex_str(&str).unwrap();

        println!("HEX: {}", str);
    }

    #[test]
    fn sorting() {
        let str: Vec<char> = "EEEEAAACCCCCCCBBBBBBDD".chars().collect();
        let mut table = Table::default();
        table.extend(str.iter().copied());

        for chr in table.iter() {
            print!("{}", chr);
        }
        println!();

        for chr in table.iter_sorted() {
            print!("{}", chr);
        }
        println!();
    }

    #[test]
    fn large_string() {
        let input = "................................................................\
        ..........................XXXXXXXXXXXX..........................\
        ......................XXXXXXXXXXXXXXXXXXXX......................\
        ....................XXXXXXXXXXXXXXXXXXXXXXXX....................\
        .................XXXXXXXXXXXXXXXXXXXXXXXXXXXXXX.................\
        ................XXXXXXXXXXX..........XXXXXXXXXXX................\
        ..............XXXXXXXXX..................XXXXXXXXX..............\
        .............XXXXXXX........................XXXXXXX.............\
        ...........XXXXXXX............................XXXXXXX...........\
        ..........XXXXXXX..............................XXXXXXX..........\
        .........XXXXXX..................................XXXXXX.........\
        ........XXXXXX....................................XXXXXX........\
        .......XXXXXX......................................XXXXXX.......\
        .......XXXXX........................................XXXXX.......\
        ......XXXXX..........................................XXXXX......\
        .....XXXXX............................................XXXXX.....\
        .....XXXX..............................................XXXX.....\
        ....XXXXX...........XXXX................XXXX...........XXXXX....\
        ...XXXXX..........XXXXXXXX............XXXXXXXX..........XXXXX...\
        ...XXXX..........XXXXXXXXXX..........XXXXXXXXXX..........XXXX...\
        ...XXXX..........XXXXXXXXXX..........XXXXXXXXXX..........XXXX...\
        ..XXXX..........XXXXXXXXXXXX........XXXXXXXXXXXX..........XXXX..\
        ..XXXX..........XXXXXXXXXXXX........XXXXXXXXXXXX..........XXXX..\
        .XXXXX..........XXXXXXXXXXXX........XXXXXXXXXXXX..........XXXXX.\
        .XXXX...........XXXXXXXXXXXX........XXXXXXXXXXXX...........XXXX.\
        .XXXX............XXXXXXXXXX..........XXXXXXXXXX............XXXX.\
        .XXXX............XXXXXXXXXX..........XXXXXXXXXX............XXXX.\
        XXXXX.............XXXXXXXX............XXXXXXXX.............XXXXX\
        XXXX................XXXX................XXXX................XXXX\
        XXXX........................................................XXXX\
        XXXX........................................................XXXX\
        XXXX........................................................XXXX\
        XXXX........................................................XXXX\
        XXXX........................................................XXXX\
        XXXX........................................................XXXX\
        XXXX........................................................XXXX\
        XXXX........................................................XXXX\
        XXXX........................................................XXXX\
        XXXXX......................................................XXXXX\
        .XXXX......................................................XXXX.\
        .XXXX........XX..................................XX........XXXX.\
        .XXXX........XXX................................XXX........XXXX.\
        .XXXXX.......XXXX..............................XXXX.......XXXXX.\
        ..XXXX........XXXX............................XXXX........XXXX..\
        ..XXXX........XXXXXX........................XXXXXX........XXXX..\
        ...XXXX........XXXXXXXX..................XXXXXXXX........XXXX...\
        ...XXXX.........XXXXXXXXXXX..........XXXXXXXXXXX.........XXXX...\
        ...XXXXX.........XXXXXXXXXXXXXXXXXXXXXXXXXXXXXX.........XXXXX...\
        ....XXXXX...........XXXXXXXXXXXXXXXXXXXXXXXX...........XXXXX....\
        .....XXXX.............XXXXXXXXXXXXXXXXXXXX.............XXXX.....\
        .....XXXXX................XXXXXXXXXXXX................XXXXX.....\
        ......XXXXX..........................................XXXXX......\
        .......XXXXX........................................XXXXX.......\
        .......XXXXXX......................................XXXXXX.......\
        ........XXXXXX....................................XXXXXX........\
        .........XXXXXX..................................XXXXXX.........\
        ..........XXXXXXX..............................XXXXXXX..........\
        ...........XXXXXXX............................XXXXXXX...........\
        .............XXXXXXX........................XXXXXXX.............\
        ..............XXXXXXXXX..................XXXXXXXXX..............\
        ................XXXXXXXXXXX..........XXXXXXXXXXX................\
        .................XXXXXXXXXXXXXXXXXXXXXXXXXXXXXX.................\
        ....................XXXXXXXXXXXXXXXXXXXXXXXX....................\
        ......................XXXXXXXXXXXXXXXXXXXX......................\
        ..........................XXXXXXXXXXXX..........................";

        let chars: Vec<char> = input.chars().collect();

        // Create an RLE table with an entry for each unique character
        //let table = Table::from_slice(&chars);
        let mut table = Table::default();

        // Encode the ASCII image into a sequence of RLE bytes
        let start = Instant::now();
        let encoded: Vec<u8> = table.encode_bytes_mut(&chars).unwrap().flatten().collect();
        let encode_time = (Instant::now() - start).as_micros();

        // Decode the bytes back into a string
        let start = Instant::now();
        let decoded: String = table.decode_bytes(&encoded).collect();
        let decode_time = (Instant::now() - start).as_micros();

        assert_eq!(input, decoded);

        println!("Number of chars in input string ...... {}", chars.len());
        println!("Number of unique symbols ............. {}", table.len());
        println!("Number of bytes, encoded ............. {}", encoded.len());
        println!("Number of chars in decoded string .... {}", decoded.len());
        println!("Time to encode ....................... {} μs", encode_time);
        println!("Time to decode ....................... {} μs", decode_time);
    }
}
