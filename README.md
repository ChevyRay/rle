# RLE
A rust library for run-length encoded (RLE) sequences.

[Run-length encoding on Wikipedia](https://en.wikipedia.org/wiki/Run-length_encoding)

# Usage

```rust
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
let encoded: Vec<u8> = table.encode_bytes_mut(&chars).unwrap().flatten().collect();

// Decode the bytes back into a string
let decoded: String = table.decode_bytes(&encoded).collect();

assert_eq!(input, decoded);

println!("Number of chars in input string ...... {}", chars.len());
println!("Number of unique symbols ............. {}", table.len());
println!("Number of bytes, encoded ............. {}", encoded.len());
println!("Number of chars in decoded string .... {}", decoded.len());

// Number of chars in input string ...... 4160
// Number of unique symbols ............. 2
// Number of bytes, encoded ............. 604
// Number of chars in decoded string .... 4160
```
