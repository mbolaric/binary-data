use std::io::Read;

use crate::{bin_error::Result, BinSeek, Error};

/// BinRingMemoryBuffer is a custom circular buffer that allows reading from a memory buffer with a position pointer.
#[derive(Default)]
pub struct BinRingMemoryBuffer {
    buffer: Vec<u8>,
    position: usize,
}

impl BinRingMemoryBuffer {
    pub fn new_with_offset(buffer: Vec<u8>, offset: usize) -> Self {
        let mut mem_buff = BinRingMemoryBuffer::from(buffer);
        mem_buff.position = offset;
        mem_buff
    }

    fn read_byte(&mut self) -> u8 {
        let result = self.buffer[self.position];
        self.next();
        result
    }

    fn read_bytes(&mut self, buffer: &mut [u8]) -> usize {
        if self.buffer.is_empty() {
            return 0;
        }
        for item in buffer.iter_mut() {
            *item = self.read_byte();
        }
        buffer.len()
    }

    fn next(&mut self) {
        let mut new_pos: usize = self.position + 1;
        if new_pos == self.buffer.len() {
            new_pos = 0;
        }
        self.position = new_pos;
    }
}

/// Implement the `BinSeek` trait for `BinRingMemoryBuffer` to support seeking, getting the current position, and the buffer length.
impl BinSeek for BinRingMemoryBuffer {
    fn seek(&mut self, _: usize) -> Result<usize> {
        Err(Error::NotSupported)
    }

    fn pos(&mut self) -> Result<usize> {
        Ok(self.position)
    }

    fn len(&self) -> Result<usize> {
        Ok(self.buffer.len())
    }
    /// Return true only if buffer is empty becouse that is ring buffer.
    fn is_eof(&mut self) -> bool {
        self.len().unwrap_or(0) == 0
    }
}

/// Implement the `Read` trait for `BinRingMemoryBuffer` to allow reading from it just like a file.
impl Read for BinRingMemoryBuffer {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        let len = self.read_bytes(buffer);
        Ok(len)
    }

    fn read_exact(&mut self, buffer: &mut [u8]) -> std::io::Result<()> {
        self.read_bytes(buffer);
        Ok(())
    }
}

/// Allows conversion from a `Vec<u8>` to a `BinRingMemoryBuffer`
impl From<Vec<u8>> for BinRingMemoryBuffer {
    fn from(buffer: Vec<u8>) -> Self {
        BinRingMemoryBuffer {
            buffer,
            position: 0,
        }
    }
}

/// Allows conversion from a byte slice (`&[u8]`) to a `BinRingMemoryBuffer`
impl From<&[u8]> for BinRingMemoryBuffer {
    fn from(buffer: &[u8]) -> Self {
        BinRingMemoryBuffer {
            buffer: Vec::from(buffer),
            position: 0,
        }
    }
}
