use std::{fs::File, path::Path};

use crate::{bin_error::Result, BinFile};

#[derive(Debug, Default)]
pub struct BinWriter {}

impl BinWriter {
    /// This function will create a file if it does not exist, and will truncate it if it does,
    /// and create a new BinFile instance from it ()
    pub fn create(file_path: &str) -> Result<BinFile> {
        let path = Path::new(file_path);
        let file = File::create(path)?;
        BinFile::new(file)
    }
}
