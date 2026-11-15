//! Three ways to break the rules of raw pointers. Run this only under Miri.

use memforge_memory::allocation::AlignedPage;
use std::ptr;

fn main() {
    let case = std::env::args().nth(1).unwrap_or_default();
    let mut page = AlignedPage::allocate();
    let base = page.as_mut_ptr();
    match case.as_str() {
        "overlap" => {
            // SAFETY: none. 64..73 and 68..77 overlap: copy_nonoverlapping forbids that.
            unsafe { ptr::copy_nonoverlapping(base.add(64), base.add(68), 9) };
        }
        "past-end" => {
            // SAFETY: none. 5000 bytes on is past the page's allocation: add forbids that,
            // even if nothing is ever read there.
            let far = unsafe { base.add(5000) };
            println!("{far:p}");
        }
        _ => eprintln!("usage: ch5_bugs overlap | past-end"),
    }
}
