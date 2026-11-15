//! Where each key's latest value is. The index lives only in memory: it is rebuilt from the
//! data file every time the database opens.

use std::collections::HashMap;

/// A record's place: the page it is in, and its slot in that page.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
    pub page: u64,
    pub slot: u16,
}

/// Every key that has a value, and where its latest value is.
#[derive(Debug, Default)]
pub struct Index {
    map: HashMap<Vec<u8>, Location>,
}

impl Index {
    /// A newer value for `key`, at `at`: it replaces any older one.
    pub fn set(&mut self, key: &[u8], at: Location) {
        self.map.insert(key.to_vec(), at);
    }

    /// `key` was deleted: it has no value any more.
    pub fn remove(&mut self, key: &[u8]) {
        self.map.remove(key);
    }

    /// Where `key`'s latest value is, if it has one.
    pub fn get(&self, key: &[u8]) -> Option<Location> {
        self.map.get(key).copied()
    }

    /// How many keys have a value.
    pub fn len(&self) -> usize {
        self.map.len()
    }
}
