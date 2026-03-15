use binary_data::{BinMemoryBuffer, BitReader};

#[test]
fn test_bit_cursor() {
    let data = vec![0xAA, 0xBB, 0xCC, 0xDD];
    let mem_buffer = BinMemoryBuffer::from(data);
    // Use a small buffer to force refills
    let mut bit_reader = BitReader::with_capacity(mem_buffer, 2);

    assert_eq!(bit_reader.bit_cursor(), 0);

    // Read 8 bits (1 byte). This should trigger fill_buf.
    bit_reader.read_bits(8).unwrap();
    // After reading 1 byte, bit_cursor should be 8.
    assert_eq!(bit_reader.bit_cursor(), 8);

    // Read another 8 bits.
    bit_reader.read_bits(8).unwrap();
    assert_eq!(bit_reader.bit_cursor(), 16);

    // Read another 8 bits. This should trigger another fill_buf.
    bit_reader.read_bits(8).unwrap();
    assert_eq!(bit_reader.bit_cursor(), 24);
}

#[test]
fn test_read_bits_aligned() {
    let data = vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66];
    let mem_buffer = BinMemoryBuffer::from(data);
    let mut bit_reader = BitReader::new(mem_buffer);

    // Read 16 bits (0x1122). This should trigger the byte-aligned optimization.
    let val = bit_reader.read_bits(16).unwrap();
    assert_eq!(val, 0x1122);
    assert_eq!(bit_reader.bit_cursor(), 16);

    // Read another 16 bits (0x3344).
    let val = bit_reader.read_bits(16).unwrap();
    assert_eq!(val, 0x3344);
    assert_eq!(bit_reader.bit_cursor(), 32);
}

#[test]
fn test_fill_buf_sync() {
    let data = vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66];
    let mem_buffer = BinMemoryBuffer::from(data);
    // Large buffer to hold everything
    let mut bit_reader = BitReader::with_capacity(mem_buffer, 10);

    // Read 8 bits (0x11). pos is now 1.
    bit_reader.read_bits(8).unwrap();
    assert_eq!(bit_reader.bit_cursor(), 8);

    // Force a refill by seeking or calling it indirectly (we'll just use the bit_reader's logic).
    // Let's assume some future method might call it. For now, we'll verify seek_bits also syncs.
    bit_reader.seek_bits(16).unwrap(); // Seeks to 0x33
    assert_eq!(bit_reader.bit_cursor(), 16);
    let val = bit_reader.read_bits(8).unwrap();
    assert_eq!(val, 0x33);
}
