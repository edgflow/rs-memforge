//! How much heap memory each side keeps for 100,000 records, counted by a global allocator.

use memforge::Database;
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

#[path = "../benches/baseline.rs"]
mod baseline;
use baseline::Baseline;

/// The bytes allocated right now, across the whole program.
static LIVE: AtomicUsize = AtomicUsize::new(0);

/// The system's allocator, with a count of every byte it hands out and gets back.
struct Counting;

// SAFETY: every call goes straight to System, which keeps GlobalAlloc's contract; the
// count only adds and subtracts the sizes it is told.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: the caller keeps alloc's contract, and System gets the same layout.
        let ptr = unsafe { System.alloc(layout) };
        if !ptr.is_null() {
            LIVE.fetch_add(layout.size(), Ordering::Relaxed);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        LIVE.fetch_sub(layout.size(), Ordering::Relaxed);
        // SAFETY: ptr came from System.alloc above, with this same layout.
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

const VALUE: &[u8] = b"a value of twenty-four b";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = std::fs::remove_dir_all("./data/memory"); // start from an empty directory
    let start = LIVE.load(Ordering::Relaxed);
    let mut db = Database::open("./data/memory/memforge")?;
    for i in 0..100_000 {
        db.put(format!("user:{i}").as_bytes(), VALUE)?;
    }
    db.flush()?;
    let held = LIVE.load(Ordering::Relaxed) - start;
    println!("MEMFORGE: {} keys, {} KB of heap", db.len(), held / 1024);
    drop(db);

    let start = LIVE.load(Ordering::Relaxed);
    let mut base = Baseline::open("./data/memory/base.log".as_ref())?;
    for i in 0..100_000 {
        base.put(format!("user:{i}").as_bytes(), VALUE)?;
    }
    base.flush()?;
    assert_eq!(base.get(b"user:1"), Some(VALUE));
    let held = LIVE.load(Ordering::Relaxed) - start;
    println!("baseline: {} keys, {} KB of heap", base.len(), held / 1024);
    Ok(())
}
