//! A page of memory that MEMFORGE owns: allocated at a 4096 boundary, given back once.

use crate::layout::{PAGE_ALIGN, PAGE_SIZE};
use std::alloc::{self, Layout};
use std::ptr::NonNull;

/// PAGE_SIZE bytes, starting at a multiple of PAGE_ALIGN.
pub struct AlignedPage {
    ptr: NonNull<u8>,
}

impl AlignedPage {
    /// The layout of every page: PAGE_SIZE bytes, aligned to PAGE_ALIGN.
    pub fn layout() -> Layout {
        Layout::from_size_align(PAGE_SIZE, PAGE_ALIGN)
            .expect("PAGE_ALIGN is a power of two")
    }

    /// Allocates a page. If the allocator has no memory left, the program stops.
    pub fn allocate() -> Self {
        let layout = Self::layout();
        // SAFETY: the layout's size, PAGE_SIZE, is not zero.
        let raw = unsafe { alloc::alloc(layout) };
        let ptr = NonNull::new(raw).unwrap_or_else(|| alloc::handle_alloc_error(layout));
        Self { ptr }
    }

    /// Where the page starts.
    pub fn as_ptr(&self) -> *const u8 {
        self.ptr.as_ptr()
    }
}

impl Drop for AlignedPage {
    fn drop(&mut self) {
        // SAFETY: ptr came from alloc with exactly this layout, and drop runs once,
        // so the page is given back exactly once.
        unsafe { alloc::dealloc(self.ptr.as_ptr(), Self::layout()) }
    }
}
