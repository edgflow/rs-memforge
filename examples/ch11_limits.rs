//! What Miri cannot promise: it checks one execution, and only Rust.

unsafe extern "C" {
    /// The C library's sort: foreign code, with no Rust behind it.
    fn qsort(
        base: *mut u8,
        n: usize,
        size: usize,
        cmp: extern "C" fn(*const u8, *const u8) -> i32,
    );
}

extern "C" fn by_value(a: *const u8, b: *const u8) -> i32 {
    // SAFETY: qsort passes pointers to two of our bytes.
    unsafe { i32::from(*a) - i32::from(*b) }
}

fn main() {
    let case = std::env::args().nth(1).unwrap_or_default();
    match case.as_str() {
        "misaligned" => {
            let bytes = vec![0u8; 8];
            let one_in = bytes.as_ptr().wrapping_add(1).cast::<u32>();
            // SAFETY: none. A u32 needs an address that is a multiple of 4. One byte into
            // the vector's bytes, that holds only by luck, if at all.
            let n = unsafe { one_in.read() };
            println!("read {n}");
        }
        "foreign" => {
            let mut bytes = [3u8, 1, 2];
            // SAFETY: bytes holds 3 elements of 1 byte, and by_value compares two of them.
            unsafe { qsort(bytes.as_mut_ptr(), 3, 1, by_value) };
            println!("{bytes:?}");
        }
        _ => eprintln!("usage: ch11_limits misaligned | foreign"),
    }
}
