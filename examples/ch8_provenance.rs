//! A pointer is an address and a permission. Every case here runs under Miri.

use memforge_memory::allocation::AlignedPage;
use memforge_memory::layout::PAGE_SIZE;

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
        "int-cast" => {
            // SAFETY: 1234 is inside page a.
            let record = unsafe { a.as_ptr().add(1234) };
            let addr = record as usize;
            let start = (addr & !(PAGE_SIZE - 1)) as *const u8;
            // SAFETY: start is page a's first byte, if a pointer made from an integer
            // may read it at all.
            let byte = unsafe { start.read() };
            println!("first byte: {byte:#x}");
        }
        _ => eprintln!("usage: ch8_provenance same-address|int-cast"),
    }
}
