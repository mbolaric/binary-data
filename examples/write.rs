use std::io::{Read, Write};

use binary_data::{BigEndian, BinReader, BinWriter, WriteBytes};

fn main() {
    // Write
    let buff = vec![1, 2, 3, 4, 5];
    match BinWriter::create("./test.ddd") {
        Ok(mut bin_file) => {
            let _ = bin_file.write_all(&buff);
            let _ = bin_file.write_u8(0);
            let _ = bin_file.write_u16::<BigEndian>(4112);
            let _ = bin_file.write_u24::<BigEndian>(1579032);
            let _ = bin_file.write_u32::<BigEndian>(538976288);
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
