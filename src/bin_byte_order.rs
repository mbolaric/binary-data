use crate::bin_error::Result;
use std::{fmt::Debug, io::Read};

/// The ByteOrder trait defines methods for reading various sizes of unsigned integers
/// with respect to the byte order (big-endian or little-endian).
pub trait ByteOrder: Default + Debug + Clone {
    fn read_u16(buf: &[u8]) -> Result<u16>;
    fn read_u24(buf: &[u8]) -> Result<u32>;
    fn read_u32(buf: &[u8]) -> Result<u32>;
}

/// BigEndian is a marker struct for big-endian byte order.
#[derive(Clone, Copy, Debug)]
pub enum BigEndian {}

impl Default for BigEndian {
    /// This panics as the default is not meant to be used for BigEndian.
    fn default() -> BigEndian {
        panic!("BigEndian")
    }
}

impl ByteOrder for BigEndian {
    /// Reads a 16-bit unsigned integer from a big-endian buffer
    #[inline]
    fn read_u16(buf: &[u8]) -> Result<u16> {
        Ok(u16::from_be_bytes(buf[..2].try_into()?))
    }

    /// Reads a 24-bit unsigned integer from a big-endian buffer
    #[inline]
    fn read_u24(buf: &[u8]) -> Result<u32> {
        let mut out = [0; 4];
        out[1..].copy_from_slice(&buf[..3]);
        Ok(u32::from_be_bytes(out))
    }

    /// Reads a 32-bit unsigned integer from a big-endian buffer
    #[inline]
    fn read_u32(buf: &[u8]) -> Result<u32> {
        Ok(u32::from_be_bytes(buf[..4].try_into()?))
    }
}

/// LittleEndian is a marker struct for little-endian byte order.
#[derive(Clone, Copy, Debug)]
pub enum LittleEndian {}

impl Default for LittleEndian {
    /// This panics as the default is not meant to be used for LittleEndian.
    fn default() -> LittleEndian {
        panic!("LittleEndian")
    }
}

impl ByteOrder for LittleEndian {
    /// Reads a 16-bit unsigned integer from a little-endian buffer
    #[inline]
    fn read_u16(buf: &[u8]) -> Result<u16> {
        Ok(u16::from_le_bytes(buf[..2].try_into()?))
    }

    /// Reads a 24-bit unsigned integer from a little-endian buffer
    #[inline]
    fn read_u24(buf: &[u8]) -> Result<u32> {
        let mut out = [0; 4];
        out[..3].copy_from_slice(&buf[..3]);
        Ok(u32::from_le_bytes(out))
    }

    /// Reads a 32-bit unsigned integer from a little-endian buffer
    #[inline]
    fn read_u32(buf: &[u8]) -> Result<u32> {
        Ok(u32::from_le_bytes(buf[..4].try_into()?))
    }
}

/// The ReadBytes trait adds additional methods for reading specific types of data from a byte stream.
pub trait ReadBytes: Read {
    /// Reads a single byte from the stream
    #[inline]
    fn read_u8(&mut self) -> Result<u8> {
        let mut buf: [u8; 1] = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    /// Reads a 16-bit unsigned integer from the stream using the specified byte order.
    #[inline]
    fn read_u16<T: ByteOrder>(&mut self) -> Result<u16> {
        let mut buf: [u8; 2] = [0; 2];
        self.read_exact(&mut buf)?;
        T::read_u16(&buf)
    }

    /// Reads a 24-bit unsigned integer from the stream using the specified byte order.
    #[inline]
    fn read_u24<T: ByteOrder>(&mut self) -> Result<u32> {
        let mut buf = [0; 3];
        self.read_exact(&mut buf)?;
        T::read_u24(&buf)
    }

    /// Reads a 32-bit unsigned integer from the stream using the specified byte order.
    #[inline]
    fn read_u32<T: ByteOrder>(&mut self) -> Result<u32> {
        let mut buffer: [u8; 4] = [0; 4];
        self.read_exact(&mut buffer)?;
        T::read_u32(&buffer)
    }

    /// Reads exactly `N` bytes from the stream into a fixed-size array.
    #[inline]
    fn read_bytes<const N: usize>(&mut self) -> Result<[u8; N]> {
        let mut buffer: [u8; N] = [0; N];
        self.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    /// Reads exactly `length` bytes from the stream into a `Vec<u8>`.
    #[inline]
    fn read_into_vec(&mut self, length: u32) -> Result<Vec<u8>> {
        let mut buffer: Vec<u8> = vec![0; length as usize];
        let buffer_slice = buffer.as_mut_slice();
        self.read_exact(buffer_slice)?;
        Ok(buffer)
    }
}

/// Implement the ReadBytes trait for all types that implement the `Read` trait.
impl<R: Read + ?Sized> ReadBytes for R {}
