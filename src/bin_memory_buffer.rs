use std::io::{Read, Write};

use crate::{bin_error::Result, BinSeek};

/// BinMemoryBuffer is a custom buffer that allows reading from a memory buffer with a position pointer.
#[derive(Debug, Default)]
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

// Implementing the `Write` trait for `BinMemoryBuffer`, allowing it to be write like any other `Write` type
impl Write for BinMemoryBuffer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let bytes_to_end = self.buffer.len() - self.position;
        if buf.len() > bytes_to_end {
            let bytes_out_of_buffer = buf.len() - bytes_to_end;
            self.buffer
                .resize(self.buffer.len() + bytes_out_of_buffer, 0);
        }

        self.buffer[self.position..(self.position + buf.len())].clone_from_slice(buf);
        self.position += buf.len();

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
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
