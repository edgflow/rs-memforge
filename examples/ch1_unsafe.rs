//! What `unsafe` changes in Rust, and what it does not.

fn main() {
    let mut page = [0u8; 8];

    // Two mutable references to the same array, inside an unsafe block.
    unsafe {
        let a = &mut page;
        let b = &mut page;
        a[0] = 1;
        b[0] = 2;
    }
}
