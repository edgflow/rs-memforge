//! The page format. A header comes first; records grow from the header toward the end;
//! the slot directory grows from the end toward the header; free space is in between.

use crate::codec::{get_u16, get_u32, get_u64, put_u16, put_u32, put_u64};

/// Every MEMFORGE page starts with these four bytes.
pub const MAGIC: [u8; 4] = *b"MFPG";
/// The version of this format.
pub const VERSION: u8 = 1;
/// The header's size on disk.
pub const HEADER_SIZE: usize = 24;

/// A page header, decoded. repr(C) keeps the fields in this order, at C's offsets.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageHeader {
    pub magic: [u8; 4],
    pub version: u8,
    pub page_id: u64,
    pub free_start: u16,
    pub count: u16,
    pub checksum: u32,
}

impl PageHeader {
    /// Writes the header into the first HEADER_SIZE bytes of `out`, field by field.
    pub fn encode(&self, out: &mut [u8]) {
        out[0..4].copy_from_slice(&self.magic);
        out[4] = self.version;
        out[5..8].fill(0); // three unused bytes, always zero on disk
        put_u64(out, 8, self.page_id);
        put_u16(out, 16, self.free_start);
        put_u16(out, 18, self.count);
        put_u32(out, 20, self.checksum);
    }

    /// Reads a header back from the first HEADER_SIZE bytes of `bytes`.
    pub fn decode(bytes: &[u8]) -> PageHeader {
        PageHeader {
            magic: [bytes[0], bytes[1], bytes[2], bytes[3]],
            version: bytes[4],
            page_id: get_u64(bytes, 8),
            free_start: get_u16(bytes, 16),
            count: get_u16(bytes, 18),
            checksum: get_u32(bytes, 20),
        }
    }
}
