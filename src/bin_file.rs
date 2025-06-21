use std::{fs::{File, Metadata}, io::{Read, Seek, SeekFrom}};

use crate::{bin_error::{Error, Result}, BinSeek};

#[derive(Debug)]
pub struct BinFile {
    file: File,
    metadata: Metadata
}

impl BinFile {
    pub(crate) fn new(file: File) -> Result<Self> {
        Ok(BinFile {
            metadata: file.metadata()?,
            file
        })
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn read_bytes(&mut self, buffer: &mut [u8]) -> Result<()> {
        if self.file.stream_position()? + buffer.len() as u64 > self.metadata.len() {
            return Err(Error::File(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "end of file")));
        }
        self.file.read_exact(buffer)?;
        Ok(())
    }

    pub fn read_n_bytes<const N: usize>(&mut self) -> Result<[u8; N]> {
        if self.file.stream_position()? + N as u64 > self.metadata.len() {
            return Err(Error::File(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "end of file")));
        }
        let mut buffer: [u8; N] = [0; N];
        self.file.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    pub fn skip_n_bytes<const N: usize>(&mut self) -> Result<()> {
        if self.file.stream_position()? + N as u64 > self.metadata.len() {
            return Err(Error::File(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "end of file")));
        }
        self.file.seek(SeekFrom::Current(N as i64))?;
        Ok(())
    }

    pub fn is_eof(&mut self) -> Result<bool> {
        Ok(self.file.stream_position()? >= self.metadata.len())
    }
}

impl Read for BinFile {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        self.file.read(buffer)
    }
}

impl BinSeek for BinFile {
    fn seek(&mut self, to: usize) -> Result<usize> {
        Ok(self.file.seek(SeekFrom::Start(to as u64))? as usize)
    }

    fn pos(&mut self) -> Result<usize> {
        Ok(self.file.stream_position()? as usize)
    }

    fn len(&self) -> Result<usize> {
        Ok(self.metadata.len() as usize)
    }
}
