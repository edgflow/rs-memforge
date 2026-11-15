//! Reading the same bits as another type, with the functions the compiler suggested.

fn main() {
    let bytes: [u8; 4] = [0x07, 0x00, 0x00, 0x00];
    println!("from_ne_bytes: {}", u32::from_ne_bytes(bytes));
    println!("from_le_bytes: {}", u32::from_le_bytes(bytes));
    println!("from_be_bytes: {}", u32::from_be_bytes(bytes));
    println!("0x3F800000 as f32: {}", f32::from_bits(0x3F80_0000));
}
