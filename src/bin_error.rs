use core::fmt;
use std::{array::TryFromSliceError, io};

#[derive(Debug)]
pub enum Error {
    InvalidInputArray,
    File(io::Error)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            InvalidInputArray =>  f.write_str("conversion from array to slice fails"),
            File(err) => f.write_str(err.to_string().as_str())
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::File(value)
    }
}

impl From<TryFromSliceError> for Error {
    fn from(_: TryFromSliceError) -> Self {
        Error::InvalidInputArray
    }
}

pub type Result<T> = core::result::Result<T, Error>;