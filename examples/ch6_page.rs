//! A page: built, sealed, written to a file, read back, checked, and then damaged.

use memforge_memory::allocation::AlignedPage;
use memforge_storage::page::SlottedPage;
use std::fs;

fn main() -> std::io::Result<()> {
    let mut page = SlottedPage::new(7);
    page.insert(b"user:1001", b"Behrouz").unwrap();
    page.insert(b"user:1002", b"Rust Developer").unwrap();
    page.insert(b"flag:dark-mode", b"").unwrap();
    println!("{} records, {} bytes free", page.count(), page.free());
    fs::write("page.bin", page.seal())?;

    // Back from the file into a fresh page, checked before anything is read.
    let mut fresh = AlignedPage::allocate();
    fresh.view_mut().write(0, &fs::read("page.bin")?).unwrap();
    let opened = SlottedPage::open(fresh).unwrap();
    for index in 0..opened.count() {
        let (key, value) = opened.get(index).unwrap();
        let (key, value) = (String::from_utf8_lossy(key), String::from_utf8_lossy(value));
        println!("slot {index}: {key} = {value:?}");
    }

    // One flipped bit: the B of Behrouz, at byte 33, becomes a C.
    let mut damaged = fs::read("page.bin")?;
    damaged[33] ^= 0x01;
    let mut copy = AlignedPage::allocate();
    copy.view_mut().write(0, &damaged).unwrap();
    println!("after one flipped bit: {:?}", SlottedPage::open(copy).err());
    Ok(())
}
