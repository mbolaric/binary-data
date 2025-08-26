use std::io::{Read, Write};

use binary_data::{BinReader, BinWriter};

fn main() {
    // Write
    let buff = vec![1, 2, 3, 4, 5];
    match BinWriter::create("./test.ddd") {
        Ok(mut bin_file) => {
            let _ = bin_file.write_all(&buff);
            let _ = bin_file.write(&[1, 1]);
            let _ = bin_file.flush();
        }
        Err(error) => println!("{:?}", error),
    }

    // Read
    match BinReader::open("./test.ddd") {
        Ok(mut bin_file) => {
            let mut buf = Vec::<u8>::new();
            let _ = bin_file.read_to_end(&mut buf);
            println!("Read: {:?}", buf);
        }
        Err(error) => println!("{:?}", error),
    }
}
