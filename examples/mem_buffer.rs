use binary_data::{BigEndian, BinMemoryBuffer, BinSeek, ReadBytes, WriteBytes};

fn main() {
    // Read
    let buff = vec![1, 2, 3, 4, 5];
    let mut mem_buff = BinMemoryBuffer::from(buff);
    let ret = mem_buff.read_u16::<BigEndian>();
    println!("{:?}", ret);
    let ret = mem_buff.is_eof();
    println!("{:?}", ret);
    let ret = mem_buff.read_into_vec(3);
    println!("{:?}", ret);
    let ret = mem_buff.is_eof();
    println!("{:?}", ret);

    // Write
    let mut mem_buff = BinMemoryBuffer::new();
    let ret = mem_buff.write_u16::<BigEndian>(4112);
    println!("{:?}, {:?}", ret, mem_buff);
    let ret = mem_buff.write_u8(8);
    println!("{:?}, {:?}", ret, mem_buff);
    let ret = mem_buff.write_u24::<BigEndian>(1579032);
    println!("{:?}, {:?}", ret, mem_buff);
    let ret = mem_buff.write_u32::<BigEndian>(538976288);
    println!("{:?}, {:?}", ret, mem_buff);
}
