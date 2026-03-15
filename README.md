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

# Bit-level Reading

For more granular control, you can use the `BitReader` to read data at the bit level. The `BitReader` wraps any reader that implements `Read` and `BinSeek`.

```rust
use binary_data::{BinMemoryBuffer, BitReader};

// Create a buffer with some data
let data = vec![0b10110011, 0b01010101, 0b11110000];
let mem_buffer = BinMemoryBuffer::from(data);

// Create a BitReader
let mut bit_reader = BitReader::new(mem_buffer);

// Read the first bit
let bit = bit_reader.read_bit().unwrap();
println!("First bit: {}", bit); // Expected: 1

// Read the next 4 bits
let four_bits = bit_reader.read_bits(4).unwrap();
println!("Next 4 bits: {}", four_bits); // Expected: 6 (0110)

// Seek to bit position 14 and read 3 bits
bit_reader.seek_bits(14).unwrap();
let three_bits = bit_reader.read_bits(3).unwrap();
println!("3 bits at position 14: {}", three_bits); // Expected: 3 (011)

// Read a single bit at position 23 without moving the reader
let bit_at_23 = bit_reader.read_bit_at(23).unwrap();
println!("Bit at position 23: {}", bit_at_23); // Expected: 0

// Read a single bit as a u8
bit_reader.seek_bits(0).unwrap();
let bit_as_u8: u8 = bit_reader.read_bit_as().unwrap();
println!("Bit as u8: {}", bit_as_u8); // Expected: 1
```
