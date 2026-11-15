//! Rebuilding the index from the data file when the database opens: every whole page, in
//! order, so that a later record for a key wins over an earlier one.

use crate::datafile::DataFile;
use crate::entry::{self, Entry};
use crate::index::{Index, Location};
use memforge_memory::layout::PAGE_SIZE;
use memforge_storage::page::SlottedPage;
use std::io;

/// What recovery found in the data file.
#[derive(Debug, Default)]
pub struct Recovery {
    /// Whole pages in the file.
    pub pages: u64,
    /// Records applied to the index: values and tombstones.
    pub records: usize,
    /// Pages that failed their checks, and why. They are reported, and skipped.
    pub corrupt: Vec<(u64, String)>,
    /// Bytes after the last whole page: a page whose write never finished. Ignored.
    pub tail: u64,
}

/// Scans the file into `index`, and reports what it found.
pub fn recover(file: &DataFile, index: &mut Index) -> io::Result<Recovery> {
    let len = file.len()?;
    let page_size = PAGE_SIZE as u64;
    let mut report =
        Recovery { pages: len / page_size, tail: len % page_size, ..Default::default() };
    for id in 0..report.pages {
        let page = match SlottedPage::open(file.read_page(id)?) {
            Ok(page) => page,
            Err(e) => {
                report.corrupt.push((id, format!("{e:?}")));
                continue;
            }
        };
        // Every record is read first: a page with one unreadable record is skipped whole,
        // so a damaged page never applies half of its records.
        let records: Option<Vec<_>> = (0..page.count())
            .map(|slot| {
                let (key, stored) = page.get(slot)?;
                Some((slot, key, entry::decode(stored)?))
            })
            .collect();
        let Some(records) = records else {
            report.corrupt.push((id, "a record is neither a value nor a tombstone".into()));
            continue;
        };
        for (slot, key, entry) in records {
            match entry {
                Entry::Value(_) => index.set(key, Location { page: id, slot }),
                Entry::Tombstone => index.remove(key),
            }
            report.records += 1;
        }
    }
    Ok(report)
}
