//! MEMFORGE's own pages: aligned by contract, not by habit.

use memforge_memory::allocation::AlignedPage;
use memforge_memory::layout::{PAGE_ALIGN, PAGE_SIZE};

fn main() {
    let pages: Vec<AlignedPage> = (0..12).map(|_| AlignedPage::allocate()).collect();
    let offsets: Vec<usize> =
        pages.iter().map(|p| p.as_ptr().addr() % PAGE_ALIGN).collect();
    println!("page address % {PAGE_ALIGN}: {offsets:?}");

    // Fill one page, write a few bytes through a slice, and read them back.
    let mut page = AlignedPage::allocate();
    page.fill(0xAB);
    page.as_bytes_mut()[..4].copy_from_slice(b"MEMF");
    let bytes = page.as_bytes();
    println!("first 8 bytes: {:02x?}", &bytes[..8]);
    println!("last byte:     {:#04x} (of {PAGE_SIZE})", bytes[PAGE_SIZE - 1]);
}
