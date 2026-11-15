//! The sizes MEMFORGE's memory is measured in.

/// One MEMFORGE page: the unit we allocate, fill with records, and write to disk.
pub const PAGE_SIZE: usize = 4096;

/// Where every page must start: at a multiple of its own size. Then a page never
/// straddles two of the operating system's memory pages, and the address of any
/// byte inside a page tells us which page it belongs to.
pub const PAGE_ALIGN: usize = 4096;
