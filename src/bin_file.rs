use std::{
    fs::{File, Metadata},
    io::{Read, Seek, SeekFrom},
};

use crate::{
    bin_error::{Error, Result},
    BinSeek,
};

/// Struct to represent a binary file with metadata
#[derive(Debug)]
pub struct BinFile {
    file: File,
    metadata: Metadata,
}

impl BinFile {
    // Creates a new instance of `BinFile`, initializing the file and retrieving its metadata
    pub(crate) fn new(file: File) -> Result<Self> {
        Ok(BinFile {
            metadata: file.metadata()?,
            file,
        })
    }

    /// Returns a reference to the file's metadata
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    /// Reads `buffer.len()` bytes from the file into `buffer`, checking for EOF
    pub fn read_bytes(&mut self, buffer: &mut [u8]) -> Result<()> {
        if self.file.stream_position()? + buffer.len() as u64 > self.metadata.len() {
            return Err(Error::File(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "end of file",
            )));
        }
        self.file.read_exact(buffer)?;
        Ok(())
    }

    /// Reads exactly `N` bytes from the file and returns them as an array
    pub fn read_n_bytes<const N: usize>(&mut self) -> Result<[u8; N]> {
        if self.file.stream_position()? + N as u64 > self.metadata.len() {
            return Err(Error::File(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "end of file",
            )));
        }
        let mut buffer: [u8; N] = [0; N];
        self.file.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    /// Skips `N` bytes from the current file position
    pub fn skip_n_bytes<const N: usize>(&mut self) -> Result<()> {
        if self.file.stream_position()? + N as u64 > self.metadata.len() {
            return Err(Error::File(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "end of file",
            )));
        }
        self.file.seek(SeekFrom::Current(N as i64))?;
        Ok(())
    }

    /// Checks if the current position is at the end of the file (EOF)
    pub fn is_eof(&mut self) -> Result<bool> {
        Ok(self.file.stream_position()? >= self.metadata.len())
    }
}

// Implementing the `Read` trait for `BinFile`, allowing it to be read like any other `Read` type
impl Read for BinFile {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        self.file.read(buffer)
    }
}

impl BinSeek for BinFile {
    /// Seeks to a specific position (in bytes) within the file
    fn seek(&mut self, to: usize) -> Result<usize> {
        Ok(self.file.seek(SeekFrom::Start(to as u64))? as usize)
    }

    /// Returns the current position (in bytes) within the file
    fn pos(&mut self) -> Result<usize> {
        Ok(self.file.stream_position()? as usize)
    }

    /// Returns the length (size) of the file
    fn len(&self) -> Result<usize> {
        Ok(self.metadata.len() as usize)
    }
}
