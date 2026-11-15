//! Handing ownership away, and taking it back: what a C program does with a MEMFORGE
//! handle. Every case here runs under Miri.

use memforge_memory::allocation::AlignedPage;
use std::mem;

fn main() {
    let case = std::env::args().nth(1).unwrap_or_default();
    match case.as_str() {
        "round-trip" => {
            let raw: *mut AlignedPage = Box::into_raw(Box::new(AlignedPage::allocate()));
            println!("handed out: a raw pointer; nothing owns the page now");
            // SAFETY: raw came from Box::into_raw just above, and is taken back once.
            let page = unsafe { Box::from_raw(raw) };
            println!("taken back: {} bytes, freed when `page` goes", page.as_bytes().len());
        }
        "forget" => {
            let page = Box::new(AlignedPage::allocate());
            mem::forget(page);
            println!("forgotten: never freed, and still safe");
        }
        "twice" => {
            let raw = Box::into_raw(Box::new(AlignedPage::allocate()));
            // SAFETY: raw came from Box::into_raw; this is the first time it is taken back.
            drop(unsafe { Box::from_raw(raw) });
            // SAFETY: none. raw was already taken back and freed: a second owner.
            drop(unsafe { Box::from_raw(raw) });
        }
        _ => eprintln!("usage: ch10_handles round-trip | forget | twice"),
    }
}
