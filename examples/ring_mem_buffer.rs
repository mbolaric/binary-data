use binary_data::{BigEndian, BinRingMemoryBuffer, ReadBytes};

fn main() {
    let buff = vec![1, 2, 3, 4, 5];
    let mut cyclic_mem_buff = BinRingMemoryBuffer::from(buff);
    let ret = cyclic_mem_buff.read_u16::<BigEndian>();
    println!("{:?}", ret);
    let ret = cyclic_mem_buff.read_u8();
    println!("{:?}", ret);
    let ret = cyclic_mem_buff.read_into_vec(4);
    println!("{:?}", ret);
}
