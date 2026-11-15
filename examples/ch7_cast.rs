//! The shortcut: reading a page's header by casting its bytes to a &PageHeader.
//! Every case here runs under Miri first.

use memforge_storage::page::{PageHeader, SlottedPage};

fn main() {
    let case = std::env::args().nth(1).unwrap_or_default();
    let mut page = SlottedPage::new(7);
    page.insert(b"user:1001", b"Behrouz").unwrap();
    let bytes = page.seal();
    match case.as_str() {
        "page" => {
            // SAFETY: the claim this chapter checks: the bytes are aligned for PageHeader,
            // and they hold a valid PageHeader.
            let header = unsafe { &*(bytes.as_ptr() as *const PageHeader) };
            println!("cast:    {header:?}");
            println!("decoded: {:?}", PageHeader::decode(bytes));
        }
        "offset-1" => {
            // SAFETY: none. One byte in, the address is odd, and PageHeader needs 8.
            let header = unsafe { &*(bytes.as_ptr().add(1) as *const PageHeader) };
            println!("{header:?}");
        }
        _ => eprintln!("usage: ch7_cast page | offset-1"),
    }
}
