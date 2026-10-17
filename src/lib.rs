//! # binary-data
//!
//! A fast and flexible Rust crate for reading and writing binary data from files
//! or in-memory buffers, with support for byte-order conversion and bit-level streams.
//!
//! ## Features
//!
//! - **File & In-Memory I/O**: Read and write binary streams via [`BinReader`], [`BinWriter`],
//!   [`BinFile`], [`BinMemoryBuffer`], and [`BinRingMemoryBuffer`].
//! - **Endianness Support**: Read and write `u8`, `u16`, `u24`, and `u32` integers in
//!   [`BigEndian`] or [`LittleEndian`] byte order through [`ReadBytes`] and [`WriteBytes`].
//! - **Bit-Level Reading**: Parse arbitrary bit fields and seek to bit positions with [`BitReader`].
//! - **Unified Seeking**: Query cursor position, length, and seek across files and memory buffers
//!   via [`BinSeek`].
//!
//! ## Examples
//!
//! ### Reading and Writing with `BinMemoryBuffer`
//!
//! ```rust
//! use binary_data::{BigEndian, BinMemoryBuffer, BinSeek, ReadBytes, WriteBytes};
//!
//! fn main() -> binary_data::Result<()> {
//!     let mut buffer = BinMemoryBuffer::new();
//!     buffer.write_u8(0x01)?;
//!     buffer.write_u16::<BigEndian>(0x1010)?;
//!     buffer.write_u24::<BigEndian>(0x181818)?;
//!     buffer.write_u32::<BigEndian>(0x20202020)?;
//!
//!     buffer.seek(0)?;
//!     assert_eq!(buffer.read_u8()?, 0x01);
//!     assert_eq!(buffer.read_u16::<BigEndian>()?, 0x1010);
//!     assert_eq!(buffer.read_u24::<BigEndian>()?, 0x181818);
//!     assert_eq!(buffer.read_u32::<BigEndian>()?, 0x20202020);
//!     Ok(())
//! }
//! ```
//!
//! ### Bit-Level Reading with `BitReader`
//!
//! ```rust
//! use binary_data::{BinMemoryBuffer, BitReader};
//!
//! fn main() -> binary_data::Result<()> {
//!     let data = vec![0b1011_0011, 0b0101_0101];
//!     let mut bit_reader = BitReader::new(BinMemoryBuffer::from(data));
//!
//!     assert_eq!(bit_reader.read_bit()?, true);
//!     assert_eq!(bit_reader.read_bits(4)?, 0b0110);
//!
//!     bit_reader.seek_bits(8)?;
//!     assert_eq!(bit_reader.read_bits(4)?, 0b0101);
//!     Ok(())
//! }
//! ```

mod bin_bit_reader;
mod bin_byte_order;
mod bin_error;
mod bin_file;
mod bin_memory_buffer;
mod bin_reader;
mod bin_ring_memory_buffer;
mod bin_seek;
mod bin_writer;

pub use bin_bit_reader::BitReader;
pub use bin_byte_order::*;
pub use bin_error::{Error, Result};
pub use bin_file::BinFile;
pub use bin_memory_buffer::*;
pub use bin_reader::BinReader;
pub use bin_ring_memory_buffer::*;
pub use bin_seek::BinSeek;
pub use bin_writer::BinWriter;
