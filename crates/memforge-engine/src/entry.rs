//! What a record's stored value holds: a value, or a tombstone that says the key was
//! deleted.

/// The first stored byte of a value.
const VALUE: u8 = 1;
/// The only stored byte of a tombstone.
const TOMBSTONE: u8 = 0;

/// A record, read back from a page.
#[derive(Debug, PartialEq, Eq)]
pub enum Entry<'a> {
    /// The key's value, borrowed from the page.
    Value(&'a [u8]),
    /// The key was deleted.
    Tombstone,
}

/// The bytes stored for a value: the VALUE mark, then the value itself.
pub fn value(value: &[u8]) -> Vec<u8> {
    let mut stored = Vec::with_capacity(1 + value.len());
    stored.push(VALUE);
    stored.extend_from_slice(value);
    stored
}

/// The bytes stored for a delete: the TOMBSTONE mark alone.
pub fn tombstone() -> [u8; 1] {
    [TOMBSTONE]
}

/// Reads stored bytes back. None for bytes that neither `value` nor `tombstone` wrote.
pub fn decode(stored: &[u8]) -> Option<Entry<'_>> {
    match stored {
        [VALUE, rest @ ..] => Some(Entry::Value(rest)),
        [TOMBSTONE] => Some(Entry::Tombstone),
        _ => None,
    }
}
