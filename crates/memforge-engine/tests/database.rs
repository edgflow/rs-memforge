//! The engine's promises, each checked through the public API, on a real file.

use memforge_core::Error;
use memforge_engine::Database;
use std::fs::{self, OpenOptions};
use std::path::PathBuf;

const PAGE: u64 = 4096;

/// A database directory of its own for each test, empty at the start.
fn fresh(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join("memforge-tests").join(name);
    let _ = fs::remove_dir_all(&dir); // it may not exist yet
    dir
}

#[test]
fn an_empty_database_has_nothing() {
    let db = Database::open(fresh("empty")).unwrap();
    assert_eq!(db.get(b"user:1001").unwrap(), None);
    assert_eq!(db.recovery().pages, 0);
}

#[test]
fn a_value_put_is_a_value_got() {
    let mut db = Database::open(fresh("put-get")).unwrap();
    db.put(b"user:1001", b"Behrouz").unwrap();
    assert_eq!(db.get(b"user:1001").unwrap(), Some(b"Behrouz".to_vec()));
    assert_eq!(db.get(b"user:1002").unwrap(), None, "a missing key is None");
}

#[test]
fn the_latest_value_wins_before_and_after_reopen() {
    let dir = fresh("latest");
    let mut db = Database::open(&dir).unwrap();
    db.put(b"k", b"first").unwrap();
    db.put(b"k", b"second").unwrap();
    assert_eq!(db.get(b"k").unwrap(), Some(b"second".to_vec()));
    db.close().unwrap();
    let db = Database::open(&dir).unwrap();
    assert_eq!(db.get(b"k").unwrap(), Some(b"second".to_vec()));
}

#[test]
fn a_deleted_key_stays_deleted_after_reopen() {
    let dir = fresh("delete");
    let mut db = Database::open(&dir).unwrap();
    db.put(b"k", b"v").unwrap();
    db.delete(b"k").unwrap();
    assert_eq!(db.get(b"k").unwrap(), None);
    db.close().unwrap();
    let db = Database::open(&dir).unwrap();
    assert_eq!(db.get(b"k").unwrap(), None);
    assert_eq!(db.recovery().records, 2, "the value, then its tombstone");
}

#[test]
fn many_records_fill_many_pages_and_all_come_back() {
    let dir = fresh("many");
    let mut db = Database::open(&dir).unwrap();
    for i in 0..1000 {
        db.put(format!("key:{i}").as_bytes(), &[7; 100]).unwrap();
    }
    db.close().unwrap();
    let db = Database::open(&dir).unwrap();
    assert!(db.recovery().pages > 1);
    assert_eq!(db.len(), 1000);
    assert_eq!(db.get(b"key:999").unwrap(), Some(vec![7; 100]));
}

#[test]
fn a_corrupt_page_is_reported_and_skipped() {
    let dir = fresh("corrupt");
    let mut db = Database::open(&dir).unwrap();
    for i in 0..3 {
        db.put(format!("key:{i}").as_bytes(), b"v").unwrap();
        db.flush().unwrap(); // one page per flush
    }
    db.close().unwrap();
    let path = dir.join("memforge.db");
    let mut bytes = fs::read(&path).unwrap();
    bytes[(PAGE + 30) as usize] ^= 0x01; // one bit, in page 1
    fs::write(&path, bytes).unwrap();

    let db = Database::open(&dir).unwrap();
    let corrupt: Vec<u64> = db.recovery().corrupt.iter().map(|(id, _)| *id).collect();
    assert_eq!(corrupt, [1]);
    assert!(db.get(b"key:0").unwrap().is_some());
    assert_eq!(db.get(b"key:1").unwrap(), None);
    assert!(db.get(b"key:2").unwrap().is_some());
}

#[test]
fn a_truncated_file_keeps_its_whole_pages() {
    let dir = fresh("truncated");
    let mut db = Database::open(&dir).unwrap();
    for i in 0..2 {
        db.put(format!("key:{i}").as_bytes(), b"v").unwrap();
        db.flush().unwrap();
    }
    db.close().unwrap();
    let file = OpenOptions::new().write(true).open(dir.join("memforge.db")).unwrap();
    file.set_len(PAGE + 100).unwrap(); // page 1 cut short, as if its write never finished

    let mut db = Database::open(&dir).unwrap();
    assert_eq!((db.recovery().pages, db.recovery().tail), (1, 100));
    assert!(db.get(b"key:0").unwrap().is_some());
    assert_eq!(db.get(b"key:1").unwrap(), None);
    db.put(b"key:2", b"v").unwrap();
    db.close().unwrap();
    let len = fs::metadata(dir.join("memforge.db")).unwrap().len();
    assert_eq!(len, 2 * PAGE, "the next page replaced the tail");
}

#[test]
fn a_record_larger_than_a_page_is_refused() {
    let mut db = Database::open(fresh("too-large")).unwrap();
    let result = db.put(b"k", &[0; 5000]);
    // 5002 bytes as stored: the key, the value's mark, and the value.
    assert!(matches!(result, Err(Error::RecordTooLarge { len: 5002 })));
}
