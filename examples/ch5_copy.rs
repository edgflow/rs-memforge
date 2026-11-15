//! Moving bytes into a page through raw pointers: copy_nonoverlapping, and copy.

use memforge_memory::allocation::AlignedPage;
use std::ptr;

fn main() {
    let mut page = AlignedPage::allocate();
    let key = b"user:1001";
    let base = page.as_mut_ptr();

    // Two separate blocks of memory: the key, and the page. They cannot overlap.
    // SAFETY: the key has key.len() bytes; offset 64 + 9 bytes lies inside the page.
    unsafe { ptr::copy_nonoverlapping(key.as_ptr(), base.add(64), key.len()) };
    println!("at 64: {:?}", String::from_utf8_lossy(&page.as_bytes()[64..73]));

    // Moving bytes inside the same page, four to the left: the two ranges overlap.
    // SAFETY: 64..73 and 60..69 both lie inside the page; copy allows them to overlap.
    unsafe { ptr::copy(base.add(64), base.add(60), key.len()) };
    println!("at 60: {:?}", String::from_utf8_lossy(&page.as_bytes()[60..69]));
}
