use binary_data::{BigEndian, BinMemoryBuffer, BinSeek, ReadBytes};

fn main() {
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
}
