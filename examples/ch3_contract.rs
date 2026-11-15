//! The allocation contract, broken one rule at a time. Run this only under Miri.

use memforge_memory::allocation::AlignedPage;

fn main() {
    let case = std::env::args().nth(1).unwrap_or_default();
    match case.as_str() {
        "read-fresh" => {
            let page = AlignedPage::allocate();
            // SAFETY: none. Nothing was ever written to the page: this read is the bug.
            println!("byte 10 = {}", unsafe { *page.as_ptr().add(10) });
        }
        _ => eprintln!("usage: ch3_contract read-fresh"),
    }
}
