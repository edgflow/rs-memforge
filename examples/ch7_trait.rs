//! Moving the promise from every cast to the type: an unsafe trait.

use memforge_storage::page::{PageHeader, SlottedPage};

/// Types that can be read straight from bytes, where they lie.
///
/// # Safety
/// Implement this only for a type where any `size_of::<Self>()` bytes, whatever their
/// values, form a valid `Self`: integers, arrays of them, and repr(C) structs made only of
/// those.
unsafe trait FromBytes: Sized {
    /// A `Self` at the start of `bytes`, if they are long enough, and aligned for it.
    fn ref_from(bytes: &[u8]) -> Option<&Self> {
        let start = bytes.as_ptr().cast::<Self>();
        if bytes.len() < size_of::<Self>() || !start.is_aligned() {
            return None;
        }
        // SAFETY: the length and the alignment were just checked, and the trait's
        // contract says that any bytes are a valid Self.
        Some(unsafe { &*start })
    }
}

// SAFETY: every field of PageHeader is an integer or an array of bytes, so any 24 bytes
// are a valid PageHeader. Its padding is never read through the reference.
unsafe impl FromBytes for PageHeader {}

fn main() {
    let mut page = SlottedPage::new(7);
    page.insert(b"user:1001", b"Behrouz").unwrap();
    let bytes = page.seal();
    let id = |h: &PageHeader| h.page_id;
    println!("at 0:      {:?}", PageHeader::ref_from(bytes).map(id));
    println!("at 1:      {:?}", PageHeader::ref_from(&bytes[1..]).map(id));
    println!("10 bytes:  {:?}", PageHeader::ref_from(&bytes[..10]).map(id));
}
