//! A page of memory that MEMFORGE owns: allocated at a 4096 boundary, given back once.

use crate::layout::{PAGE_ALIGN, PAGE_SIZE};
use std::alloc::{self, Layout};
use std::ptr::NonNull;

/// PAGE_SIZE bytes, starting at a multiple of PAGE_ALIGN. Every byte always holds a value.
pub struct AlignedPage {
    ptr: NonNull<u8>,
}

impl AlignedPage {
    /// The layout of every page: PAGE_SIZE bytes, aligned to PAGE_ALIGN.
    pub fn layout() -> Layout {
        Layout::from_size_align(PAGE_SIZE, PAGE_ALIGN)
            .expect("PAGE_ALIGN is a power of two")
    }

    /// Allocates a page with every byte set to zero. If the allocator has no memory left,
    /// the program stops.
    pub fn allocate() -> Self {
        let layout = Self::layout();
        // SAFETY: the layout's size, PAGE_SIZE, is not zero.
        let raw = unsafe { alloc::alloc_zeroed(layout) };
        let ptr = NonNull::new(raw).unwrap_or_else(|| alloc::handle_alloc_error(layout));
        Self { ptr }
    }

    /// Where the page starts.
    pub fn as_ptr(&self) -> *const u8 {
        self.ptr.as_ptr()
    }

    /// Sets every byte of the page to `byte`.
    pub fn fill(&mut self, byte: u8) {
        // SAFETY: ptr points to PAGE_SIZE bytes that this page owns, and `&mut self`
        // means nothing else is using them while we write.
        unsafe { self.ptr.as_ptr().write_bytes(byte, PAGE_SIZE) }
    }

    /// The page's bytes, to read.
    pub fn as_bytes(&self) -> &[u8] {
        // SAFETY: ptr is not null, a u8 needs no alignment, all PAGE_SIZE bytes hold
        // values (zeroed, and only ever written as bytes), they live as long as the page,
        // and `&self` keeps anyone from writing them while this slice exists.
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), PAGE_SIZE) }
    }

    /// The page's bytes, to write.
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        // SAFETY: as in as_bytes, and `&mut self` makes this slice the only way in.
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), PAGE_SIZE) }
    }
}

impl Drop for AlignedPage {
    fn drop(&mut self) {
        // SAFETY: ptr came from alloc_zeroed with exactly this layout, and drop runs once,
        // so the page is given back exactly once.
        unsafe { alloc::dealloc(self.ptr.as_ptr(), Self::layout()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_page_is_aligned_and_zeroed() {
        let page = AlignedPage::allocate();
        assert_eq!(page.as_ptr().addr() % PAGE_ALIGN, 0);
        assert!(page.as_bytes().iter().all(|&b| b == 0));
    }

    #[test]
    fn fill_and_writes_reach_every_byte() {
        let mut page = AlignedPage::allocate();
        page.fill(0xAB);
        page.as_bytes_mut()[PAGE_SIZE - 1] = 7;
        assert!(page.as_bytes()[..PAGE_SIZE - 1].iter().all(|&b| b == 0xAB));
        assert_eq!(page.as_bytes()[PAGE_SIZE - 1], 7);
    }

    #[test]
    fn the_page_layout_is_never_zero_sized() {
        assert_eq!(AlignedPage::layout().size(), PAGE_SIZE);
        assert_eq!(AlignedPage::layout().align(), PAGE_ALIGN);
        assert_ne!(PAGE_SIZE, 0);
    }
}
