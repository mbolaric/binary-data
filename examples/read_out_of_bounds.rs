use binary_data::{BinMemoryBuffer, BitReader};

fn main() {
    // Create a buffer with only one byte
    let data = vec![0b10110011];
    let mem_buffer = BinMemoryBuffer::from(data);

    // Create a BitReader
    let mut bit_reader = BitReader::new(mem_buffer);

    // Attempt to read a bit at a position outside the buffer's bounds
    println!("Attempting to read bit at position 24...");
    let result = bit_reader.read_bit_at(24);

    match result {
        Ok(bit) => {
            println!("Unexpectedly succeeded! Bit value: {}", bit);
        }
        Err(e) => {
            println!("Successfully caught expected error: {:?}", e);
        }
    }
}
