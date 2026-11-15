//! A MEMFORGE database: a data file of pages, an index in memory, and one page being
//! filled.

use crate::datafile::DataFile;
use crate::entry::{self, Entry};
use crate::index::{Index, Location};
use crate::recovery::{self, Recovery};
use memforge_core::{Error, Result};
use memforge_storage::page::{PageError, SlottedPage};
use std::fs;
use std::path::{Path, PathBuf};

/// The data file's name, inside the database's directory.
const FILE_NAME: &str = "memforge.db";

/// A MEMFORGE database: one directory on disk. Records that are not flushed are lost when
/// the process ends, so `flush` or `close` before it does.
pub struct Database {
    dir: PathBuf,
    file: DataFile,
    index: Index,
    /// The page being filled. It is only in memory until it is written.
    active: SlottedPage,
    recovery: Recovery,
}

impl Database {
    /// Opens the database in `dir`, creating it if needed, and rebuilds the index.
    pub fn open(dir: impl AsRef<Path>) -> Result<Self> {
        let dir = dir.as_ref().to_path_buf();
        fs::create_dir_all(&dir)?;
        let file = DataFile::open(&dir.join(FILE_NAME))?;
        let mut index = Index::default();
        let recovery = recovery::recover(&file, &mut index)?;
        let active = SlottedPage::new(recovery.pages);
        Ok(Self { dir, file, index, active, recovery })
    }

    /// The directory this database lives in.
    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// What opening found in the data file.
    pub fn recovery(&self) -> &Recovery {
        &self.recovery
    }

    /// How many keys have a value.
    pub fn len(&self) -> usize {
        self.index.len()
    }

    /// Stores `value` under `key`. The newest record for a key always wins.
    pub fn put(&mut self, key: &[u8], value: &[u8]) -> Result<()> {
        let at = self.append(key, &entry::value(value))?;
        self.index.set(key, at);
        Ok(())
    }

    /// Removes `key`, by appending a tombstone: a record that says the key is gone.
    pub fn delete(&mut self, key: &[u8]) -> Result<()> {
        self.append(key, &entry::tombstone())?;
        self.index.remove(key);
        Ok(())
    }

    /// The latest value stored under `key`, copied out of its page.
    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        self.get_with(key, |value| value.to_vec())
    }

    /// Calls `f` with `key`'s latest value, borrowed straight from its page: zero-copy. The
    /// page lives only during this call, so the value can be borrowed only inside `f`.
    pub fn get_with<R>(&self, key: &[u8], f: impl FnOnce(&[u8]) -> R) -> Result<Option<R>> {
        let Some(at) = self.index.get(key) else {
            return Ok(None);
        };
        let read;
        let page = if at.page == self.active.id() {
            &self.active
        } else {
            read = SlottedPage::open(self.file.read_page(at.page)?).map_err(|e| {
                Error::CorruptPage { page: at.page, reason: format!("{e:?}") }
            })?;
            &read
        };
        match page.get(at.slot).and_then(|(_, stored)| entry::decode(stored)) {
            Some(Entry::Value(value)) => Ok(Some(f(value))),
            _ => Err(Error::CorruptPage {
                page: at.page,
                reason: "no value in that slot".into(),
            }),
        }
    }

    /// Makes every write so far durable: the active page goes to the file, and the file to
    /// the disk. Records after this go into a new page.
    pub fn flush(&mut self) -> Result<()> {
        if self.active.count() > 0 {
            self.write_active()?;
        }
        self.file.sync()?;
        Ok(())
    }

    /// Flushes, then closes the database.
    pub fn close(mut self) -> Result<()> {
        self.flush()
    }

    /// Adds a record to the active page. When the page is full, it is written, and the
    /// record goes into a new one.
    fn append(&mut self, key: &[u8], stored: &[u8]) -> Result<Location> {
        let slot = match self.active.insert(key, stored) {
            Ok(slot) => slot,
            Err(PageError::TooLarge { len }) => return Err(Error::RecordTooLarge { len }),
            Err(_) => {
                // The only other error is Full: it fits an empty page, just not this one.
                self.write_active()?;
                self.active.insert(key, stored).expect("it fits an empty page")
            }
        };
        Ok(Location { page: self.active.id(), slot })
    }

    /// Seals the active page, writes it at its place in the file, and begins the next one.
    fn write_active(&mut self) -> Result<()> {
        let id = self.active.id();
        self.file.write_page(id, self.active.seal())?;
        self.active = SlottedPage::new(id + 1);
        Ok(())
    }
}
