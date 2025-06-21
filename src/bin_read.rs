use std::io::Read;

use crate::bin_error::Result;

pub trait BinSeek {
    fn seek(&mut self, to: usize) -> Result<usize>;
    fn pos(&mut self) -> Result<usize>;
    fn len(&self) -> Result<usize>;
    fn is_empty(&self) -> bool {
        self.len().unwrap_or(0) > 0
    }
}

pub trait BinRead: Read + BinSeek {}