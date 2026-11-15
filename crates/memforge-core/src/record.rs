use std::fmt;

/// One key and its value, each in heap memory of its own.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
}

impl Record {
    pub fn new(key: &[u8], value: &[u8]) -> Self {
        Self { key: key.to_vec(), value: value.to_vec() }
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let key = String::from_utf8_lossy(&self.key);
        write!(f, "{key} = {}", String::from_utf8_lossy(&self.value))
    }
}
