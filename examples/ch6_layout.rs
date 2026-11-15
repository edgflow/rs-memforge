//! Where each header field sits in memory: repr(C), and Rust's own layout.

use memforge_storage::page::PageHeader;
use std::mem::offset_of;

/// The same fields, without repr(C): Rust may put them in any order.
#[allow(dead_code)]
struct RustLayout {
    magic: [u8; 4],
    version: u8,
    page_id: u64,
    free_start: u16,
    count: u16,
    checksum: u32,
}

fn main() {
    let names = ["magic", "version", "page_id", "free_start", "count", "checksum"];
    let c = [
        offset_of!(PageHeader, magic),
        offset_of!(PageHeader, version),
        offset_of!(PageHeader, page_id),
        offset_of!(PageHeader, free_start),
        offset_of!(PageHeader, count),
        offset_of!(PageHeader, checksum),
    ];
    let rust = [
        offset_of!(RustLayout, magic),
        offset_of!(RustLayout, version),
        offset_of!(RustLayout, page_id),
        offset_of!(RustLayout, free_start),
        offset_of!(RustLayout, count),
        offset_of!(RustLayout, checksum),
    ];
    println!("{:<11}{:>8}{:>12}", "field", "repr(C)", "repr(Rust)");
    for i in 0..names.len() {
        println!("{:<11}{:>8}{:>12}", names[i], c[i], rust[i]);
    }
    let sizes = (size_of::<PageHeader>(), size_of::<RustLayout>());
    println!("{:<11}{:>8}{:>12}", "size", sizes.0, sizes.1);
}
