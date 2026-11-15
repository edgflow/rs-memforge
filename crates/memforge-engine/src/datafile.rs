//! The data file: page `id` lives at byte `id × PAGE_SIZE`, and pages are only ever added.

use memforge_memory::allocation::AlignedPage;
use memforge_memory::layout::PAGE_SIZE;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::Path;

/// The one file a database keeps its pages in.
pub struct DataFile {
    file: File,
}

impl DataFile {
    /// Opens the file at `path` to read and write, creating it, empty, if it is missing.
    pub fn open(path: &Path) -> io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)?;
        Ok(Self { file })
    }

    /// The file's length in bytes. After a crash, it may end in the middle of a page.
    pub fn len(&self) -> io::Result<u64> {
        Ok(self.file.metadata()?.len())
    }

    /// Reads page `id` into a fresh page. Nothing is checked here: SlottedPage::open does.
    pub fn read_page(&self, id: u64) -> io::Result<AlignedPage> {
        let mut page = AlignedPage::allocate();
        let mut file = &self.file;
        file.seek(SeekFrom::Start(id * PAGE_SIZE as u64))?;
        file.read_exact(page.as_bytes_mut())?;
        Ok(page)
    }

    /// Writes a sealed page at its place. This hands the bytes to the operating system;
    /// they are on the disk only after `sync`.
    pub fn write_page(&mut self, id: u64, bytes: &[u8]) -> io::Result<()> {
        self.file.seek(SeekFrom::Start(id * PAGE_SIZE as u64))?;
        self.file.write_all(bytes)
    }

    /// Asks the operating system to put every byte written so far on the disk, and waits
    /// until it has.
    pub fn sync(&self) -> io::Result<()> {
        self.file.sync_all()
    }
}
