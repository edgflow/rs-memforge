//! Views into a page: checked ranges of its bytes, which cannot outlive the page.

use crate::allocation::AlignedPage;
use crate::layout::PAGE_SIZE;
use std::marker::PhantomData;
use std::ptr::NonNull;

/// A read-only view of a page. It holds only a raw pointer; PhantomData tells the
/// compiler that it borrows the page's bytes for 'a, so it can never outlive the page.
pub struct PageView<'a> {
    base: NonNull<u8>,
    _bytes: PhantomData<&'a [u8]>,
}

impl AlignedPage {
    /// A read-only view of this page, borrowed from it.
    pub fn view(&self) -> PageView<'_> {
        PageView { base: self.non_null(), _bytes: PhantomData }
    }
}

impl<'a> PageView<'a> {
    /// `len` bytes from `offset`, or None if any of them would lie outside the page.
    pub fn bytes(&self, offset: usize, len: usize) -> Option<&'a [u8]> {
        let end = offset.checked_add(len)?;
        if end > PAGE_SIZE {
            return None;
        }
        // SAFETY: offset..end lies inside the page (checked above), so `add` stays in its
        // allocation, and the slice covers only page bytes, which all hold values. The view
        // borrows the page for 'a, so nothing can write those bytes while the slice lives.
        Some(unsafe { std::slice::from_raw_parts(self.base.as_ptr().add(offset), len) })
    }
}
