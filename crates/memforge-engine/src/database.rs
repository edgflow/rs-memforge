use memforge_core::{Error, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// A MEMFORGE database: one directory on disk.
pub struct Database {
    dir: PathBuf,
}

impl Database {
    /// Opens the database in `dir`, creating the directory if it does not exist.
    pub fn open(dir: impl AsRef<Path>) -> Result<Self> {
        let dir = dir.as_ref().to_path_buf();
        fs::create_dir_all(&dir)?;
        Ok(Self { dir })
    }

    /// The directory this database lives in.
    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// Stores `value` under `key`, replacing any earlier value.
    pub fn put(&mut self, _key: &[u8], _value: &[u8]) -> Result<()> {
        Err(Error::NotImplemented("put"))
    }

    /// The latest value stored under `key`, or `None`.
    pub fn get(&self, _key: &[u8]) -> Result<Option<Vec<u8>>> {
        Err(Error::NotImplemented("get"))
    }

    /// Removes `key`.
    pub fn delete(&mut self, _key: &[u8]) -> Result<()> {
        Err(Error::NotImplemented("delete"))
    }

    /// Makes every write so far durable on disk.
    pub fn flush(&mut self) -> Result<()> {
        Err(Error::NotImplemented("flush"))
    }

    /// Flushes, then closes the database.
    pub fn close(mut self) -> Result<()> {
        self.flush()
    }
}
