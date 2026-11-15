//! The safe baseline MEMFORGE is measured against: a HashMap in memory, with every record
//! appended to one file. No unsafe code, no pages, no checksums.

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufWriter, Read, Write};
use std::path::Path;

pub struct Baseline {
    map: HashMap<Vec<u8>, Vec<u8>>,
    file: BufWriter<File>,
}

impl Baseline {
    /// Opens the file at `path`, and replays every record in it into the map.
    pub fn open(path: &Path) -> io::Result<Self> {
        let mut file =
            OpenOptions::new().read(true).append(true).create(true).open(path)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        let mut map = HashMap::new();
        let mut at = 0;
        // Each record: the key's length and the value's length, 4 bytes each, then both.
        while at + 8 <= bytes.len() {
            let k = u32::from_le_bytes(bytes[at..at + 4].try_into().unwrap()) as usize;
            let v = u32::from_le_bytes(bytes[at + 4..at + 8].try_into().unwrap()) as usize;
            let Some(record) = bytes.get(at + 8..at + 8 + k + v) else { break };
            map.insert(record[..k].to_vec(), record[k..].to_vec());
            at += 8 + k + v;
        }
        Ok(Self { map, file: BufWriter::new(file) })
    }

    pub fn put(&mut self, key: &[u8], value: &[u8]) -> io::Result<()> {
        self.file.write_all(&(key.len() as u32).to_le_bytes())?;
        self.file.write_all(&(value.len() as u32).to_le_bytes())?;
        self.file.write_all(key)?;
        self.file.write_all(value)?;
        self.map.insert(key.to_vec(), value.to_vec());
        Ok(())
    }

    pub fn get(&self, key: &[u8]) -> Option<&[u8]> {
        self.map.get(key).map(Vec::as_slice)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Writes what is buffered, and waits until it is on the disk.
    pub fn flush(&mut self) -> io::Result<()> {
        self.file.flush()?;
        self.file.get_ref().sync_all()
    }
}
