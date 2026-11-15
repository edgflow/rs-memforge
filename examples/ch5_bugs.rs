//! Breaking the rules of raw pointers, and keeping them. Run this only under Miri.

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
        "two-mut" => {
            // SAFETY: base points into the page, and nothing else uses that byte yet.
            let a = unsafe { &mut *base };
            // SAFETY: none. A second &mut to the same byte, while `a` is still to be used.
            let b = unsafe { &mut *base };
            *b = 2;
            *a = 1; // the bug: `b` took the byte over, and `a` is no longer allowed in
        }
        "one-after-another" => {
            // SAFETY: base points into the page, and each &mut is finished with before the
            // next one is made: at any moment, only one of them is in use.
            let a = unsafe { &mut *base };
            *a = 1;
            // SAFETY: as above; `a` is never used again.
            let b = unsafe { &mut *base };
            *b = 2;
            println!("byte 0 = {}", page.as_bytes()[0]);
        }
        _ => eprintln!("usage: ch5_bugs overlap | past-end | two-mut | one-after-another"),
    }
}
