# Safety

Every unsafe operation in MEMFORGE: where it is, and the invariant that makes it correct.
Only `memforge-memory` and the C door, `memforge-ffi`, contain unsafe code. The other crates
`#![forbid(unsafe_code)]`.

## allocation.rs: AlignedPage

- `allocate`, `alloc_zeroed`: the layout is never zero-sized; PAGE_SIZE is 4096.
- `fill`, `write_bytes`: the page owns its PAGE_SIZE bytes, and `&mut self` keeps every
  other user out while it writes.
- `as_bytes` and `as_bytes_mut`, `from_raw_parts`: every byte holds a value (zeroed at
  the start, and only ever written as bytes), and the slice borrows the page.
- `drop`, `dealloc`: the same layout as `alloc_zeroed`, and drop runs once.
- `unsafe impl Send`: the page is the only owner of its memory, like a `Box<[u8]>`.

## initialization.rs: Batch

- `get` with `assume_init_ref`, and `pop` with `assume_init_read`: only below `len`.
  The invariant: `items[..len]` hold valid values, and `items[len..]` hold none.
- `drop` with `assume_init_drop`: `len` goes to zero first, so nothing is dropped twice.

## pointer.rs: PageView and PageViewMut

- `bytes`, `add` and `from_raw_parts`: the range is checked first, with `checked_add`.
- `write`, `add` and `copy_nonoverlapping`: the range is checked first, and `data` cannot
  point into the page, because the view holds the page's mutable borrow.

## memforge-ffi: the C door

Rust cannot check what a C caller passes in. Each function checks what it can, and its
`# Safety` section says what the caller must promise.

- `mf_open`, `CStr::from_ptr` and `write`: `dir` must be a NUL-terminated string, and `out`
  writable. Null is refused first.
- The handle: `Box::into_raw` in `mf_open`, and `Box::from_raw` in `mf_close`, exactly once.
  Using a handle after `mf_close` is the caller's bug, and Rust cannot see it.
- `as_mut` and `as_ref` on the handle: null is refused; anything else must come from
  `mf_open`.
- `bytes`, `from_raw_parts`: null with a non-zero length is refused, and so is a length over
  `isize::MAX`. That the bytes are really there is the caller's promise.
- `mf_get`, `copy_nonoverlapping`: the value fits in `buf_len`, and a C buffer cannot
  overlap a page that MEMFORGE owns.
- Every body runs inside `catch_unwind`, so no panic ever unwinds into C.

## Pointer arithmetic

- Every `add` stays inside its page, or one past its end, after a range check.
- `page_start` changes only the address, with `map_addr`. No pointer is ever made from an
  integer, and every test runs under strict provenance, in `scripts/miri.sh`.

## Examples that break the rules on purpose

`ch2_miri`, `ch3_contract`, `ch4_zeroed`, `ch4_counters`, `ch5_bugs`, `ch7_cast`,
`ch8_provenance`, `ch10_handles`, `ch11_limits` and `ch11_race` say `SAFETY: none` where
they break a rule. They run only under Miri, which stops at the bug and explains it.

## How it is checked

`scripts/check.sh` runs the tests, then `scripts/miri.sh` (every library and test under
Miri, with file I/O allowed and strict provenance), then `scripts/asan.sh` (the C client
with AddressSanitizer). On macOS, MemorySanitizer and LeakSanitizer are not supported, so
uninitialized memory and Rust-side leaks are Miri's job. ThreadSanitizer needs
`-Zbuild-std`, and MEMFORGE shares no mutable state between threads.
