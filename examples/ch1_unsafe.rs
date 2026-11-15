//! What `unsafe` changes in Rust, and what it does not.

fn main() {
    let record = [10u8, 20, 30, 40];

    // get_unchecked skips the bounds check, so it is an unsafe function:
    // its caller has to promise that the index is in bounds.
    // SAFETY: 2 < 4, the length of `record`.
    let third = unsafe { *record.get_unchecked(2) };
    println!("get_unchecked(2) = {third}");

    println!("byte_at(3)       = {}", byte_at(&record, 3));

    // The two mutable references from before, as a safe API: two halves that can't overlap.
    let mut page = [0u8; 8];
    let (a, b) = page.split_at_mut(4);
    a[0] = 1;
    b[0] = 2;
    println!("page             = {page:?}");

    println!("byte_at(9)       = {}", byte_at(&record, 9));
}

/// Byte `i` of `bytes`. Panics if `i` is out of bounds.
fn byte_at(bytes: &[u8], i: usize) -> u8 {
    assert!(i < bytes.len(), "byte {i} is out of bounds");
    // SAFETY: the assert above guarantees that i < bytes.len().
    unsafe { *bytes.get_unchecked(i) }
}
