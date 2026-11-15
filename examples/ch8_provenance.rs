//! A pointer is an address and a permission. Every case here runs under Miri.

use memforge_memory::allocation::AlignedPage;

fn main() {
    let case = std::env::args().nth(1).unwrap_or_default();
    let mut a = AlignedPage::allocate();
    let b = AlignedPage::allocate();
    a.fill(0xAA);
    match case.as_str() {
        "same-address" => {
            let fake = a.as_ptr().with_addr(b.as_ptr().addr());
            println!("same address as b: {}", fake == b.as_ptr());
            // SAFETY: none. fake has b's address, but only a's permission.
            let byte = unsafe { fake.read() };
            println!("{byte}");
        }
        _ => eprintln!("usage: ch8_provenance same-address"),
    }
}
