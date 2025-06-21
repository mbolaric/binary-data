use std::{fs::File, io, path::Path};

use crate::{bin_error::Result, Error, BinFile};

#[derive(Debug, Default)]
pub struct BinReader {

}

impl BinReader {
    pub fn open(sup_file_path: &str) -> Result<BinFile> {
        let path = Path::new(sup_file_path);
        if !path.exists() {
            return Err(Error::File(io::Error::new(io::ErrorKind::NotFound, "File not Exists")));
        }
        let file = File::open(sup_file_path)?;
        BinFile::new(file)
    }
}