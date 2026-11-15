//! A record into a page and back, through views that cannot outlive the page.

use memforge_memory::allocation::AlignedPage;

fn main() {
    let mut page = AlignedPage::allocate();
    let mut out = page.view_mut();
    out.write(64, b"user:1001").unwrap();
    out.write(73, b"Behrouz").unwrap();
    println!("too far: {:?}", out.write(4090, b"Rust Developer"));

    let view = page.view();
    let key = view.bytes(64, 9).unwrap();
    let value = view.bytes(73, 7).unwrap();
    println!("{} = {}", String::from_utf8_lossy(key), String::from_utf8_lossy(value));
    println!("outside the page: {:?}", view.bytes(4090, 14));

    drop(page);
    println!("after the page is gone: {}", String::from_utf8_lossy(key));
}
