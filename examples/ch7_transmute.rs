//! transmute: the same bits, read as another type. The compiler knows a better way.

use std::mem;

fn main() {
    let bytes: [u8; 4] = [0x07, 0x00, 0x00, 0x00];
    // SAFETY: [u8; 4] and u32 have the same size, and any 4 bytes are a valid u32.
    let n: u32 = unsafe { mem::transmute(bytes) };
    println!("transmute:     {n}");
    println!("from_le_bytes: {}", u32::from_le_bytes(bytes));
    println!("from_be_bytes: {}", u32::from_be_bytes(bytes));
    // SAFETY: u32 and f32 have the same size, and any 32 bits are a valid f32.
    let one: f32 = unsafe { mem::transmute(0x3F80_0000_u32) };
    println!("0x3F800000 as f32: {one}");
}
