//! Integers to and from page bytes: always little-endian, whatever this machine uses.

/// Writes `value` into `buf` at `at`, as 2 little-endian bytes.
pub fn put_u16(buf: &mut [u8], at: usize, value: u16) {
    buf[at..at + 2].copy_from_slice(&value.to_le_bytes());
}

/// Writes `value` into `buf` at `at`, as 4 little-endian bytes.
pub fn put_u32(buf: &mut [u8], at: usize, value: u32) {
    buf[at..at + 4].copy_from_slice(&value.to_le_bytes());
}

/// Writes `value` into `buf` at `at`, as 8 little-endian bytes.
pub fn put_u64(buf: &mut [u8], at: usize, value: u64) {
    buf[at..at + 8].copy_from_slice(&value.to_le_bytes());
}

/// Reads the 2 little-endian bytes at `at` back into a u16.
pub fn get_u16(buf: &[u8], at: usize) -> u16 {
    u16::from_le_bytes(buf[at..at + 2].try_into().expect("a 2-byte range"))
}

/// Reads the 4 little-endian bytes at `at` back into a u32.
pub fn get_u32(buf: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(buf[at..at + 4].try_into().expect("a 4-byte range"))
}

/// Reads the 8 little-endian bytes at `at` back into a u64.
pub fn get_u64(buf: &[u8], at: usize) -> u64 {
    u64::from_le_bytes(buf[at..at + 8].try_into().expect("an 8-byte range"))
}
