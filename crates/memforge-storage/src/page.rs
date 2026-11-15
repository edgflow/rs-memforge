//! The page format. A header comes first; records grow from the header toward the end;
//! the slot directory grows from the end toward the header; free space is in between.

use crate::checksum::crc32;
use crate::codec::{get_u16, get_u32, get_u64, put_u16, put_u32, put_u64};
use crate::slot::{SLOT_SIZE, Slot};
use memforge_memory::allocation::AlignedPage;
use memforge_memory::layout::PAGE_SIZE;

/// Every MEMFORGE page starts with these four bytes.
pub const MAGIC: [u8; 4] = *b"MFPG";
/// The version of this format.
pub const VERSION: u8 = 1;
/// The header's size on disk.
pub const HEADER_SIZE: usize = 24;

/// A page header, decoded. repr(C) keeps the fields in this order, at C's offsets.
/// Page bytes are decoded into it field by field, and never cast to it.
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
        out[5..8].fill(0); // the three padding bytes, always zero on disk
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

/// What can go wrong with a page.
#[derive(Debug, PartialEq, Eq)]
pub enum PageError {
    /// The record could never fit, not even in an empty page.
    TooLarge { len: usize },
    /// The record does not fit in the free space that is left.
    Full { needed: usize, free: usize },
    /// The bytes do not start with MAGIC: this is not a MEMFORGE page.
    NotAPage,
    /// A format version this code does not know.
    UnknownVersion(u8),
    /// The bytes changed after the page was sealed.
    ChecksumMismatch { stored: u32, computed: u32 },
}

/// A page in the slotted format: header, records, free space, slot directory.
pub struct SlottedPage {
    page: AlignedPage,
    header: PageHeader,
}

impl SlottedPage {
    /// An empty page: a header, and nothing else.
    pub fn new(page_id: u64) -> Self {
        let header = PageHeader {
            magic: MAGIC,
            version: VERSION,
            page_id,
            free_start: HEADER_SIZE as u16,
            count: 0,
            checksum: 0,
        };
        Self { page: AlignedPage::allocate(), header }
    }

    /// How many records the page holds.
    pub fn count(&self) -> u16 {
        self.header.count
    }

    /// The free bytes between the last record and the slot directory.
    pub fn free(&self) -> usize {
        let slots = self.header.count as usize * SLOT_SIZE;
        PAGE_SIZE - slots - self.header.free_start as usize
    }

    /// Adds a record: its bytes after the last record, its slot before the first slot.
    pub fn insert(&mut self, key: &[u8], value: &[u8]) -> Result<u16, PageError> {
        let len = key.len() + value.len();
        let needed = len + SLOT_SIZE;
        if needed > PAGE_SIZE - HEADER_SIZE {
            return Err(PageError::TooLarge { len });
        }
        if needed > self.free() {
            return Err(PageError::Full { needed, free: self.free() });
        }
        let offset = self.header.free_start as usize;
        let index = self.header.count;
        let slot = Slot {
            offset: offset as u16,
            key_len: key.len() as u16,
            value_len: value.len() as u16,
        };
        let mut entry = [0u8; SLOT_SIZE];
        slot.encode(&mut entry);

        let mut view = self.page.view_mut();
        view.write(offset, key).expect("checked: it fits");
        view.write(offset + key.len(), value).expect("checked: it fits");
        view.write(Slot::position(index), &entry).expect("checked: it fits");
        self.header.free_start += len as u16;
        self.header.count += 1;
        Ok(index)
    }

    /// The key and value of record `index`, read straight from the page.
    pub fn get(&self, index: u16) -> Option<(&[u8], &[u8])> {
        if index >= self.header.count {
            return None;
        }
        let view = self.page.view();
        let slot = Slot::decode(view.bytes(Slot::position(index), SLOT_SIZE)?);
        let start = slot.offset as usize;
        let key = view.bytes(start, slot.key_len as usize)?;
        let value = view.bytes(start + key.len(), slot.value_len as usize)?;
        Some((key, value))
    }

    /// Writes the header, then a checksum over every other byte of the page into the
    /// header's last four bytes, and returns the page's bytes, ready for disk.
    pub fn seal(&mut self) -> &[u8] {
        let mut head = [0u8; HEADER_SIZE];
        self.header.encode(&mut head);
        self.page.view_mut().write(0, &head).expect("the header fits");
        let bytes = self.page.as_bytes();
        self.header.checksum = crc32(&[&bytes[..20], &bytes[24..]]);
        let checksum = self.header.checksum.to_le_bytes();
        self.page.view_mut().write(20, &checksum).expect("the checksum fits");
        self.page.as_bytes()
    }

    /// Opens a page read back from disk, after checking that it is one, and intact.
    pub fn open(page: AlignedPage) -> Result<Self, PageError> {
        let bytes = page.as_bytes();
        let header = PageHeader::decode(bytes);
        if header.magic != MAGIC {
            return Err(PageError::NotAPage);
        }
        if header.version != VERSION {
            return Err(PageError::UnknownVersion(header.version));
        }
        let computed = crc32(&[&bytes[..20], &bytes[24..]]);
        if computed != header.checksum {
            return Err(PageError::ChecksumMismatch { stored: header.checksum, computed });
        }
        Ok(Self { page, header })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A sealed page's bytes, copied into a fresh page, as if read back from disk.
    fn reopen(bytes: &[u8]) -> Result<SlottedPage, PageError> {
        let mut fresh = AlignedPage::allocate();
        fresh.view_mut().write(0, bytes).unwrap();
        SlottedPage::open(fresh)
    }

    #[test]
    fn records_survive_seal_and_open() {
        let mut page = SlottedPage::new(1);
        assert_eq!(page.insert(b"k1", b"v1"), Ok(0));
        assert_eq!(page.insert(b"k2", b""), Ok(1));
        let opened = reopen(page.seal()).unwrap();
        assert_eq!(opened.get(0), Some((&b"k1"[..], &b"v1"[..])));
        assert_eq!(
            opened.get(1),
            Some((&b"k2"[..], &b""[..])),
            "an empty value is a value"
        );
        assert_eq!(opened.get(2), None);
    }

    #[test]
    fn a_record_larger_than_any_page_is_refused() {
        let mut page = SlottedPage::new(1);
        let huge = vec![0u8; PAGE_SIZE];
        assert_eq!(
            page.insert(b"k", &huge),
            Err(PageError::TooLarge { len: PAGE_SIZE + 1 })
        );
    }

    #[test]
    fn a_full_page_says_how_much_is_left() {
        let mut page = SlottedPage::new(1);
        let big = vec![7u8; 3000];
        assert_eq!(page.insert(b"a", &big), Ok(0));
        let free = page.free();
        assert_eq!(page.insert(b"b", &big), Err(PageError::Full { needed: 3007, free }));
    }

    #[test]
    fn any_changed_byte_is_detected() {
        let mut page = SlottedPage::new(1);
        page.insert(b"user:1001", b"Behrouz").unwrap();
        let mut bytes = page.seal().to_vec();
        bytes[4000] ^= 0x80; // free space: the checksum covers it too
        assert!(matches!(reopen(&bytes), Err(PageError::ChecksumMismatch { .. })));
    }

    #[test]
    fn bytes_that_are_not_a_page_are_refused() {
        assert_eq!(reopen(&[0u8; PAGE_SIZE]).err(), Some(PageError::NotAPage));
    }

    #[test]
    fn the_header_is_little_endian_on_every_machine() {
        let header = PageHeader {
            magic: MAGIC,
            version: VERSION,
            page_id: 0x0102_0304_0506_0708,
            free_start: 0x090A,
            count: 0x0B0C,
            checksum: 0x0D0E_0F10,
        };
        let mut out = [0xFF; HEADER_SIZE];
        header.encode(&mut out);
        assert_eq!(out[5..8], [0, 0, 0], "padding is written as zero");
        assert_eq!(out[8..16], [8, 7, 6, 5, 4, 3, 2, 1], "page_id, smallest byte first");
        assert_eq!(PageHeader::decode(&out), header);
    }

    #[test]
    fn a_damaged_version_byte_is_an_error() {
        let mut page = SlottedPage::new(1);
        let mut bytes = page.seal().to_vec();
        bytes[4] = 2;
        assert_eq!(reopen(&bytes).err(), Some(PageError::UnknownVersion(2)));
    }
}
