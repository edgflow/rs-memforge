//! MEMFORGE's own pages: aligned by contract, not by habit.

use memforge_memory::allocation::AlignedPage;
use memforge_memory::layout::PAGE_ALIGN;

fn main() {
    let pages: Vec<AlignedPage> = (0..12).map(|_| AlignedPage::allocate()).collect();
    let offsets: Vec<usize> =
        pages.iter().map(|p| p.as_ptr().addr() % PAGE_ALIGN).collect();
    println!("page address % {PAGE_ALIGN}: {offsets:?}");
}
