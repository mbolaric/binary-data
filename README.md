# Reading binary data
A fast and flexible Rust crate for reading binary data from file or memory structure.


[![Build Status][actions-badge]][actions-url]

[actions-badge]: https://github.com/mbolaric/binary-data/actions/workflows/rust.yml/badge.svg?branch=master
[actions-url]: https://github.com/mbolaric/binary-data/actions/workflows/rust.yml?query=branch%3Amaster

# Usage
```rust
use binary_data::{BinFile, BinReader, BinSeek};

let mut file = BinReader::open(self.esm_file_path)?;
file.seek(10)?;
let block_size: u16 = reader.read_u16::<BigEndian>()?;
let data: Vec<u8> = reader.read_into_vec(block_size as u32)?;
let mem_reader = BinMemoryBuffer::from(bin);
len id = mem_reader.read_u8()?;
...
```
