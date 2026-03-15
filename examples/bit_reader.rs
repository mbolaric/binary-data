use binary_data::{BinMemoryBuffer, BitReader};

fn main() {
    // Create a buffer with some data
    let data = vec![0b10110011, 0b01010101, 0b11110010];
    let mem_buffer = BinMemoryBuffer::from(data);

    // Create a BitReader
    let mut bit_reader = BitReader::new(mem_buffer);

    // Read the first bit
    let bit = bit_reader.read_bit().unwrap();
    println!("First bit: {}", bit); // Expected: 1

    // Read the next 4 bits
    let four_bits = bit_reader.read_bits(4).unwrap();
    println!("Next 4 bits: {}", four_bits); // Expected: 6 (0110)

    // Seek to bit position 14 and read 3 bits
    bit_reader.seek_bits(14).unwrap();
    let three_bits = bit_reader.read_bits(3).unwrap();
    println!("3 bits at position 14: {}", three_bits); // Expected: 3 (011)

    // Seek to bit position 14 and read 3 bits
    bit_reader.seek_bits(14).unwrap();
    let one_bit = bit_reader.read_bit().unwrap();
    println!("1 bits at position 14: {}", one_bit); // Expected: 0

    // Seek to bit position 2 and read 8 bits
    bit_reader.seek_bits(2).unwrap();
    let eight_bits = bit_reader.read_bits(8).unwrap();
    println!("8 bits at position 2: {}", eight_bits); // Expected: 205 (11001101)

    // Read a single bit at position 22 without moving the reader
    let bit_at_22 = bit_reader.read_bit_at(22).unwrap();
    println!("Bit at position 22: {}", bit_at_22); // Expected: 1

    // Read a single bit at position 23 without moving the reader
    let bit_at_23 = bit_reader.read_bit_at(23).unwrap();
    println!("Bit at position 23: {}", bit_at_23); // Expected: 0

    // Read a single bit as a u8
    bit_reader.seek_bits(0).unwrap();
    let bit_as_u8: u8 = bit_reader.read_bit_as().unwrap();
    println!("Bit as u8: {}", bit_as_u8); // Expected: 1
}
