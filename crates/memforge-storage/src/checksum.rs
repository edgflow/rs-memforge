//! CRC-32: a 32-bit fingerprint of some bytes. Change any of them, and it changes.

/// The CRC-32 polynomial used by zip, PNG and Ethernet, written bit-reversed.
const POLY: u32 = 0xEDB8_8320;

/// For every possible byte, the effect it has on the remainder: computed once,
/// at compile time, so checking a page is one table lookup per byte.
const TABLE: [u32; 256] = make_table();

const fn make_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    let mut i = 0;
    while i < 256 {
        let mut crc = i as u32;
        let mut bit = 0;
        while bit < 8 {
            crc = if crc & 1 == 1 { (crc >> 1) ^ POLY } else { crc >> 1 };
            bit += 1;
        }
        table[i] = crc;
        i += 1;
    }
    table
}

/// The CRC-32 of the parts, one after another, as if they were one slice.
pub fn crc32(parts: &[&[u8]]) -> u32 {
    let mut crc = !0u32;
    for part in parts {
        for &byte in *part {
            crc = (crc >> 8) ^ TABLE[((crc ^ byte as u32) & 0xFF) as usize];
        }
    }
    !crc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_standard_check_value() {
        assert_eq!(crc32(&[b"123456789"]), 0xCBF4_3926);
    }

    #[test]
    fn parts_give_the_same_result_as_one_slice() {
        assert_eq!(crc32(&[b"1234", b"56789"]), crc32(&[b"123456789"]));
    }

    #[test]
    fn one_flipped_bit_changes_it() {
        assert_ne!(crc32(&[b"Behrouz"]), crc32(&[b"Cehrouz"]));
    }
}
