//! The slot directory: one entry per record, stored from the end of the page backward.

use crate::codec::{get_u16, put_u16};
use memforge_memory::layout::PAGE_SIZE;

/// The size of one slot on disk.
pub const SLOT_SIZE: usize = 6;

/// Where one record lives in its page: its offset, and the lengths of its key and value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Slot {
    pub offset: u16,
    pub key_len: u16,
    pub value_len: u16,
}

impl Slot {
    /// Where slot `index` is stored: counting back from the end of the page.
    pub fn position(index: u16) -> usize {
        PAGE_SIZE - (index as usize + 1) * SLOT_SIZE
    }

    pub fn encode(&self, out: &mut [u8]) {
        put_u16(out, 0, self.offset);
        put_u16(out, 2, self.key_len);
        put_u16(out, 4, self.value_len);
    }

    pub fn decode(bytes: &[u8]) -> Slot {
        Slot {
            offset: get_u16(bytes, 0),
            key_len: get_u16(bytes, 2),
            value_len: get_u16(bytes, 4),
        }
    }
}
