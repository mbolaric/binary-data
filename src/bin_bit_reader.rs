use crate::bin_error::{Error, Result};
use crate::bin_seek::BinSeek;
use std::io::Read;

const DEFAULT_BUF_SIZE: usize = 8 * 1024;

/// A buffered reader for bit-level data that allows seeking to arbitrary bit positions.
pub struct BitReader<R: Read + BinSeek> {
    reader: R,
    buffer: Vec<u8>,
    // The byte position in the buffer.
    pos: usize,
    // The number of bytes read into the buffer.
    cap: usize,
    // The bit position within the current byte (the byte at self.buffer[self.pos]).
    // A value of 8 means the byte has been fully read.
    bit_pos: u8,
    // The byte position in the underlying reader where the current buffer starts.
    buffer_start: u64,
}

impl<R: Read + BinSeek> BitReader<R> {
    /// Creates a new `BitReader` with a default buffer capacity.
    pub fn new(reader: R) -> Self {
        Self::with_capacity(reader, DEFAULT_BUF_SIZE)
    }

    /// Creates a new `BitReader` with a specific buffer capacity.
    pub fn with_capacity(mut reader: R, capacity: usize) -> Self {
        let buffer_start = reader.pos().unwrap_or(0) as u64;
        Self {
            reader,
            buffer: vec![0; capacity],
            pos: 0,
            cap: 0,
            bit_pos: 0,
            buffer_start,
        }
    }

    /// Fills the internal buffer from the underlying reader.
    #[inline]
    fn fill_buf(&mut self) -> Result<()> {
        if self.pos < self.cap {
            // If the buffer wasn't fully consumed, we must seek the underlying 
            // reader back to our logical position before refilling.
            self.reader.seek((self.buffer_start + self.pos as u64) as usize)?;
        }
        self.buffer_start = self.reader.pos()? as u64;
        self.cap = self.reader.read(&mut self.buffer)?;
        self.pos = 0;
        Ok(())
    }

    /// Returns the absolute bit position in the stream
    #[inline]
    pub fn bit_cursor(&self) -> u64 {
        (self.buffer_start + self.pos as u64) * 8 + self.bit_pos as u64
    }

    /// Seeks to an arbitrary bit position in the stream.
    /// This will invalidate the internal buffer.
    pub fn seek_bits(&mut self, bit_pos: usize) -> Result<()> {
        let byte_pos = bit_pos >> 3;
        let bit_offset = (bit_pos & 7) as u8;

        self.reader.seek(byte_pos)?;
        self.buffer_start = byte_pos as u64;
        self.cap = 0;
        self.pos = 0;
        self.bit_pos = bit_offset;

        Ok(())
    }

    /// Reads a single bit from the current position.
    #[inline]
    pub fn read_bit(&mut self) -> Result<bool> {
        // Ensure a byte is available
        if self.pos >= self.cap {
            self.fill_buf()?;
            if self.cap == 0 {
                return Err(Error::File(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    "end of file",
                )));
            }
        }

        debug_assert!(self.bit_pos < 8);

        let byte = self.buffer[self.pos];
        let bit = (byte >> (7 - self.bit_pos)) & 1;

        // Advance state
        if self.bit_pos == 7 {
            self.bit_pos = 0;
            self.pos += 1;
        } else {
            self.bit_pos += 1;
        }

        Ok(bit != 0)
    }

    /// Reads a single bit and returns it as the specified type (e.g., u8).
    pub fn read_bit_as<T: From<u8>>(&mut self) -> Result<T> {
        self.read_bit().map(|b| if b { 1.into() } else { 0.into() })
    }

    /// Reads a specified number of bits (up to 64) from the current position.
    #[inline]
    pub fn read_bits(&mut self, num_bits: u8) -> Result<u64> {
        if num_bits == 0 {
            return Ok(0);
        }
        if num_bits > 64 {
            return Err(Error::NotSupported);
        }

        let mut result = 0u64;
        let mut bits_left = num_bits;

        while bits_left > 0 {
            if self.pos >= self.cap {
                self.fill_buf()?;
                if self.cap == 0 {
                    return Err(Error::File(std::io::Error::new(
                        std::io::ErrorKind::UnexpectedEof,
                        "end of file",
                    )));
                }
            }

            // Optimization for byte-aligned multi-byte reads
            if self.bit_pos == 0 && bits_left >= 8 {
                let take_bytes = (bits_left / 8) as usize;
                let available_bytes = self.cap - self.pos;
                let can_take = take_bytes.min(available_bytes);

                if can_take > 0 {
                    let slice = &self.buffer[self.pos..self.pos + can_take];
                    for &b in slice {
                        result = (result << 8) | b as u64;
                    }
                    self.pos += can_take;
                    bits_left -= (can_take * 8) as u8;
                    continue;
                }
            }

            let available_in_byte = 8 - self.bit_pos;
            let take = bits_left.min(available_in_byte);

            let byte = self.buffer[self.pos];
            let shift = available_in_byte - take;
            let mask = (0xFF >> (8 - take)) << shift;
            let val = (byte & mask) >> shift;

            result = (result << take) | val as u64;
            bits_left -= take;
            self.bit_pos += take;

            if self.bit_pos == 8 {
                self.bit_pos = 0;
                self.pos += 1;
            }
        }

        Ok(result)
    }

    /// Reads a single bit from a specific position without altering the reader's main position.
    /// This is a "peek" operation and is less performant than sequential reads.
    pub fn read_bit_at(&mut self, bit_pos: usize) -> Result<bool> {
        let bit_pos_u64 = bit_pos as u64;
        let buf_start_bits = self.buffer_start * 8;
        let buf_end_bits = (self.buffer_start + self.cap as u64) * 8;

        // Fast path: bit is inside the current buffer
        if bit_pos_u64 >= buf_start_bits && bit_pos_u64 < buf_end_bits {
            let relative_bit = bit_pos_u64 - buf_start_bits;
            let byte_idx = (relative_bit >> 3) as usize;
            let bit_idx = (relative_bit & 7) as u8;
            let byte = self.buffer[byte_idx];
            return Ok(((byte >> (7 - bit_idx)) & 1) != 0);
        }

        // Slow path: temporary seek
        let byte_pos = bit_pos >> 3;
        let bit_offset = (bit_pos & 7) as u8;

        let saved_pos = self.reader.pos()?;

        self.reader.seek(byte_pos)?;
        let mut byte = [0u8; 1];
        self.reader.read_exact(&mut byte)?;
        self.reader.seek(saved_pos)?;

        Ok(((byte[0] >> (7 - bit_offset)) & 1) != 0)
    }

    /// Reads a single bit from a specific position without altering the reader's main position.
    /// This is a "peek" operation and is less performant than sequential reads and returns it as the specified type (e.g., u8).
    pub fn read_bit_at_as<T: From<u8>>(&mut self, bit_pos: usize) -> Result<T> {
        self.read_bit_at(bit_pos)
            .map(|b| if b { 1.into() } else { 0.into() })
    }
}
