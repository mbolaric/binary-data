use std::io::Read;

use crate::{bin_error::Result, BinSeek};

/// BinMemoryBuffer is a custom buffer that allows reading from a memory buffer with a position pointer.
#[derive(Default)]
pub struct BinMemoryBuffer {
    buffer: Vec<u8>,
    position: usize,
}

impl BinMemoryBuffer {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            position: 0,
        }
    }

    /// Returns a slice of the remaining data in the buffer, starting from the current position
    pub fn remaining_slice(&self) -> &[u8] {
        let start_pos = self.position.min(self.buffer.len());
        &self.buffer.as_slice()[(start_pos)..]
    }
}

/// Implement the `BinSeek` trait for `BinMemoryBuffer` to support seeking, getting the current position, and the buffer length.
impl BinSeek for BinMemoryBuffer {
    fn seek(&mut self, to: usize) -> Result<usize> {
        self.position = to;
        Ok(self.position)
    }

    fn pos(&mut self) -> Result<usize> {
        Ok(self.position)
    }

    fn len(&self) -> Result<usize> {
        Ok(self.buffer.len())
    }
}

/// Implement the `Read` trait for `BinMemoryBuffer` to allow reading from it just like a file.
impl Read for BinMemoryBuffer {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        let len = Read::read(&mut self.remaining_slice(), buffer)?;
        self.position += len;
        Ok(len)
    }

    fn read_exact(&mut self, buffer: &mut [u8]) -> std::io::Result<()> {
        let buf_len = buffer.len();
        Read::read_exact(&mut self.remaining_slice(), buffer)?;
        self.position += buf_len;
        Ok(())
    }
}

/// Allows conversion from a `Vec<u8>` to a `BinMemoryBuffer`
impl From<Vec<u8>> for BinMemoryBuffer {
    fn from(buffer: Vec<u8>) -> Self {
        BinMemoryBuffer {
            buffer,
            position: 0,
        }
    }
}

/// Allows conversion from a byte slice (`&[u8]`) to a `BinMemoryBuffer`
impl From<&[u8]> for BinMemoryBuffer {
    fn from(buffer: &[u8]) -> Self {
        BinMemoryBuffer {
            buffer: Vec::from(buffer),
            position: 0,
        }
    }
}
