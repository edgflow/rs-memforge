//! A Layout: how many bytes, and what their address must be a multiple of.

use memforge_memory::layout::{PAGE_ALIGN, PAGE_SIZE};
use std::alloc::Layout;

fn main() {
    let page = Layout::from_size_align(PAGE_SIZE, PAGE_ALIGN);
    println!("{PAGE_SIZE} bytes, align {PAGE_ALIGN}: {page:?}");

    // An alignment must be a power of two.
    let odd = Layout::from_size_align(PAGE_SIZE, 3000);
    println!("{PAGE_SIZE} bytes, align 3000: {odd:?}");

    // Every type has a layout of its own.
    println!("a u64: {:?}", Layout::new::<u64>());
}
