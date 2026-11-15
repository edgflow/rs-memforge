//! One number, two byte orders.

fn main() {
    let n: u32 = 0x1234_5678;
    println!("little-endian: {:02x?}", n.to_le_bytes());
    println!("big-endian:    {:02x?}", n.to_be_bytes());
    println!("this machine:  {:02x?}", n.to_ne_bytes());

    // The same four bytes, read back with the wrong order.
    let bytes = n.to_le_bytes();
    println!(
        "read as le: {:#x}, read as be: {:#x}",
        u32::from_le_bytes(bytes),
        u32::from_be_bytes(bytes)
    );
}
