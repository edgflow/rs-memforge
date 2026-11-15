//! Two threads, one counter, no synchronization: a data race. Run this only under Miri.

use std::thread;

static mut COUNT: u64 = 0;

fn main() {
    let other = thread::spawn(|| {
        // SAFETY: none. The main thread writes COUNT at the same time, with no lock.
        unsafe { *(&raw mut COUNT) += 1 };
    });
    // SAFETY: none, as above.
    unsafe { *(&raw mut COUNT) += 1 };
    other.join().unwrap();
    // SAFETY: the other thread has finished: nothing else touches COUNT now.
    println!("count = {}", unsafe { *(&raw const COUNT) });
}
