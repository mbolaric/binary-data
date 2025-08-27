use std::io::Write;

use binary_data::{BigEndian, BinRingMemoryBuffer, ReadBytes, WriteBytes};

fn main() {
    // Read
    let buff = vec![1, 2, 3, 4, 5];
    let mut cyclic_mem_buff = BinRingMemoryBuffer::from(buff);
    let ret = cyclic_mem_buff.read_u16::<BigEndian>();
    println!("{:?}", ret);
    let ret = cyclic_mem_buff.read_u8();
    println!("{:?}", ret);
    let ret = cyclic_mem_buff.read_into_vec(4);
    println!("{:?}", ret);

    // Write
    let buff = vec![0, 0, 0, 0, 0];
    let mut cyclic_mem_buff = BinRingMemoryBuffer::from(buff);
    let ret = cyclic_mem_buff.write(&[2, 2, 2, 3, 3, 3, 4, 5]);
    println!("{:?}. {:?}", ret, cyclic_mem_buff);
    let ret = cyclic_mem_buff.write_u8(8);
    println!("{:?}. {:?}", ret, cyclic_mem_buff);
}
