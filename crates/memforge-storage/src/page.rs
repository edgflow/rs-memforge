//! The page format. A header comes first; records grow from the header toward the end;
//! the slot directory grows from the end toward the header; free space is in between.

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
