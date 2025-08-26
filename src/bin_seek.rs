use crate::bin_error::Result;

/// Define a trait for seeking in binary files
pub trait BinSeek {
    fn seek(&mut self, to: usize) -> Result<usize>;
    fn pos(&mut self) -> Result<usize>;
    fn len(&self) -> Result<usize>;
    fn is_empty(&self) -> bool {
        self.len().unwrap_or(0) > 0
    }
    /// Return true if we are at the end of the buffer/file.
    fn is_eof(&mut self) -> bool {
        self.pos().unwrap_or(0) >= self.len().unwrap_or(0)
    }
}
