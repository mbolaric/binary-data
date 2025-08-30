# Reading/Writing binary data
A fast and flexible Rust crate for reading and writing binary data from files or memory.


[![Build Status][actions-badge]][actions-url]

[actions-badge]: https://github.com/mbolaric/binary-data/actions/workflows/rust.yml/badge.svg?branch=master
[actions-url]: https://github.com/mbolaric/binary-data/actions/workflows/rust.yml?query=branch%3Amaster

# Usage
```rust
use binary_data::{BinFile, BinReader, BinWriter, BinSeek, WriteBytes, BigEndian};

// Read
let mut file = BinReader::open(self.esm_file_path)?;
file.seek(10)?;
let block_size: u16 = reader.read_u16::<BigEndian>()?;
let data: Vec<u8> = reader.read_into_vec(block_size as u32)?;
let mem_reader = BinMemoryBuffer::from(bin);
len id = mem_reader.read_u8()?;
...
// Write
let buff = vec![1, 2, 3, 4, 5];
match BinWriter::create("./test.ddd") {
    Ok(mut bin_file) => {
        let _ = bin_file.write_all(&buff);
        let _ = bin_file.write_u8(0);
        let _ = bin_file.write_u16::<BigEndian>(4112);
        let _ = bin_file.write_u24::<BigEndian>(1579032);
        let _ = bin_file.write_u32::<BigEndian>(538976288);
        let _ = bin_file.write(&[1, 1]);
        let _ = bin_file.flush();
    }
    Err(error) => println!("{:?}", error),
}
...
```
