//! Views into a page: checked ranges of its bytes, which cannot outlive the page.

use crate::allocation::AlignedPage;
use crate::layout::{PAGE_ALIGN, PAGE_SIZE};
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

/// A range of bytes that does not fit inside a page.
#[derive(Debug, PartialEq, Eq)]
pub struct OutOfPage {
    pub offset: usize,
    pub len: usize,
}

/// A writable view of a page. While it lives, it is the only way into the page's bytes.
pub struct PageViewMut<'a> {
    base: NonNull<u8>,
    _bytes: PhantomData<&'a mut [u8]>,
}

impl AlignedPage {
    /// A writable view of this page, borrowed mutably from it.
    pub fn view_mut(&mut self) -> PageViewMut<'_> {
        PageViewMut { base: self.non_null(), _bytes: PhantomData }
    }
}

impl PageViewMut<'_> {
    /// Copies `data` into the page at `offset`, or refuses if it would not fit.
    pub fn write(&mut self, offset: usize, data: &[u8]) -> Result<(), OutOfPage> {
        let fits = offset.checked_add(data.len()).is_some_and(|end| end <= PAGE_SIZE);
        if !fits {
            return Err(OutOfPage { offset, len: data.len() });
        }
        let dst = self.base.as_ptr();
        // SAFETY: offset..offset + len lies inside the page (checked above). `data` cannot
        // overlap it: the view borrows the page mutably, so no other reference into it,
        // `data` included, can exist while the view lives.
        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr(), dst.add(offset), data.len())
        };
        Ok(())
    }
}

/// The first byte of the page that the byte at `p` belongs to: `p`'s address, rounded
/// down to a multiple of PAGE_ALIGN.
///
/// Only the address changes: `map_addr` keeps `p`'s provenance. So the result reaches only
/// as far as `p` did: a pointer from `AlignedPage::as_ptr` reaches the whole page, and a
/// pointer from a slice reaches only that slice.
pub fn page_start(p: *const u8) -> *const u8 {
    p.map_addr(|addr| addr & !(PAGE_ALIGN - 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn written_bytes_read_back() {
        let mut page = AlignedPage::allocate();
        assert_eq!(page.view_mut().write(100, b"value"), Ok(()));
        assert_eq!(page.view().bytes(100, 5), Some(&b"value"[..]));
    }

    #[test]
    fn the_last_byte_is_in_and_one_more_is_out() {
        let mut page = AlignedPage::allocate();
        assert_eq!(page.view_mut().write(PAGE_SIZE - 1, b"x"), Ok(()));
        let past = page.view_mut().write(PAGE_SIZE, b"x");
        assert_eq!(past, Err(OutOfPage { offset: PAGE_SIZE, len: 1 }));
        assert_eq!(page.view().bytes(PAGE_SIZE - 1, 2), None);
    }

    #[test]
    fn huge_offsets_are_refused_not_wrapped_around() {
        let page = AlignedPage::allocate();
        assert_eq!(page.view().bytes(usize::MAX, 2), None);
    }

    #[test]
    fn page_start_finds_the_page_from_any_byte_in_it() {
        let page = AlignedPage::allocate();
        for offset in [0, 1, 1234, PAGE_SIZE - 1] {
            let byte = page.as_ptr().wrapping_add(offset);
            assert_eq!(page_start(byte), page.as_ptr(), "offset {offset}");
        }
    }

    #[test]
    fn page_start_keeps_the_permission_to_read() {
        let mut page = AlignedPage::allocate();
        page.as_bytes_mut()[0] = 42;
        let record = page.as_ptr().wrapping_add(1234);
        // SAFETY: record comes from the page's own pointer, which may reach the whole
        // page; page_start keeps that permission, and the page is alive and unchanged.
        assert_eq!(unsafe { page_start(record).read() }, 42);
    }
}
