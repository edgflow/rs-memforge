# Safety

Every unsafe operation in MEMFORGE: where it is, and the invariant that makes it correct.
Only `memforge-memory` contains unsafe code. The other crates `#![forbid(unsafe_code)]`.

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

## Pointer arithmetic

- Every `add` stays inside its page, or one past its end, after a range check.
- `page_start` changes only the address, with `map_addr`. No pointer is ever made from an
  integer, and every test runs under strict provenance:
  `MIRIFLAGS=-Zmiri-strict-provenance cargo miri test --workspace --lib`.

## Examples that break the rules on purpose

`ch2_miri`, `ch3_contract`, `ch4_zeroed`, `ch4_counters`, `ch5_bugs`, `ch7_cast` and
`ch8_provenance` say `SAFETY: none` where they break a rule. They run only under Miri,
which stops at the bug and explains it.
