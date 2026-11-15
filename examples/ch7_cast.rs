//! The shortcut: reading a page's header by casting its bytes to a &PageHeader.
//! Every case here runs under Miri first.

use memforge_storage::page::{PageHeader, SlottedPage};

/// The same start as PageHeader, but with a flag stored as a bool where the version is.
#[repr(C)]
struct FlagHeader {
    magic: [u8; 4],
    sealed: bool,
}

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
        "as-bytes" => {
            let header = PageHeader::decode(bytes);
            let size = size_of::<PageHeader>();
            let start = &header as *const PageHeader as *const u8;
            // SAFETY: none. The 24 bytes of a PageHeader include 3 bytes of padding,
            // and padding is never initialized.
            let raw = unsafe { std::slice::from_raw_parts(start, size) };
            let version = raw[4];
            println!("byte 4: {version}");
            let after = raw[5];
            println!("byte 5: {after}");
        }
        "flag" => {
            let mut damaged = bytes.to_vec();
            damaged[4] = 2;
            // SAFETY: none. A bool must be 0 or 1, and byte 4 is now 2.
            let header = unsafe { &*(damaged.as_ptr() as *const FlagHeader) };
            let sealed = header.sealed;
            println!("{:?} sealed: {sealed}", header.magic);
        }
        _ => eprintln!("usage: ch7_cast page | offset-1 | as-bytes | flag"),
    }
}
