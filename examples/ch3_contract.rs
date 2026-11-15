//! The allocation contract, broken one rule at a time. Run this only under Miri.

use memforge_memory::allocation::AlignedPage;
use memforge_memory::layout::PAGE_SIZE;
use std::alloc::{Layout, alloc, dealloc};

fn main() {
    let case = std::env::args().nth(1).unwrap_or_default();
    match case.as_str() {
        "read-fresh" => {
            let page = AlignedPage::allocate();
            // SAFETY: none. Nothing was ever written to the page: this read is the bug.
            println!("byte 10 = {}", unsafe { *page.as_ptr().add(10) });
        }
        "wrong-layout" => {
            let layout = AlignedPage::layout();
            // SAFETY: the layout's size is not zero.
            let raw = unsafe { alloc(layout) };
            let other = Layout::from_size_align(PAGE_SIZE, 8).unwrap();
            // SAFETY: none. Giving it back with another alignment is the bug.
            unsafe { dealloc(raw, other) };
        }
        "double-free" => {
            let page = AlignedPage::allocate();
            let raw = page.as_ptr().cast_mut();
            drop(page); // the page's own Drop gives the memory back here
            // SAFETY: none. Giving the same memory back a second time is the bug.
            unsafe { dealloc(raw, AlignedPage::layout()) };
        }
        "zero-size" => {
            let empty = Layout::from_size_align(0, 8).unwrap();
            // SAFETY: none. alloc requires a size that is not zero: this call is the bug.
            let raw = unsafe { alloc(empty) };
            println!("{raw:p}");
        }
        _ => eprintln!(
            "usage: ch3_contract read-fresh | wrong-layout | double-free | zero-size"
        ),
    }
}
