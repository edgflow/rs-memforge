//! What a wrong counter does: the drop loop of Batch, given the wrong length.
//! Run this only under Miri.

use std::mem::MaybeUninit;

/// Drops the first `len` values in `items`.
///
/// # Safety
///
/// `items[..len]` must hold valid values, and none of them may be used or dropped again.
unsafe fn drop_first(items: &mut [MaybeUninit<Vec<u8>>], len: usize) {
    for item in &mut items[..len] {
        // SAFETY: the caller promises that items[..len] hold valid values.
        unsafe { item.assume_init_drop() }
    }
}

fn main() {
    let case = std::env::args().nth(1).unwrap_or_default();
    let mut items: [MaybeUninit<Vec<u8>>; 3] = [const { MaybeUninit::uninit() }; 3];
    items[0].write(vec![1; 8]);
    items[1].write(vec![2; 8]);
    // Two values written. Each case claims a different count.
    match case.as_str() {
        // SAFETY: items[..2] hold the two values written above, dropped once.
        "right" => unsafe { drop_first(&mut items, 2) },
        // SAFETY: none. Three is one too many: items[2] holds no value.
        "too-long" => unsafe { drop_first(&mut items, 3) },
        // SAFETY: none. The right count, used twice: both values are dropped twice.
        "twice" => unsafe {
            drop_first(&mut items, 2);
            drop_first(&mut items, 2);
        },
        // SAFETY: none. One is one too few: the second value is never dropped.
        "too-short" => unsafe { drop_first(&mut items, 1) },
        _ => eprintln!("usage: ch4_counters right | too-long | twice | too-short"),
    }
}
