//! A batch that fills up one value at a time: room that becomes values, in order,
//! tracked by one counter.

use std::mem::MaybeUninit;

/// Up to N values of type T. Room for all N exists from the start; values arrive
/// one at a time.
///
/// Invariant: `items[..len]` hold valid values, and `items[len..]` hold none.
pub struct Batch<T, const N: usize> {
    items: [MaybeUninit<T>; N],
    len: usize,
}

impl<T, const N: usize> Batch<T, N> {
    /// An empty batch: room for N values, and none written yet.
    pub const fn new() -> Self {
        Self { items: [const { MaybeUninit::uninit() }; N], len: 0 }
    }

    /// How many values the batch holds.
    pub fn len(&self) -> usize {
        self.len
    }

    /// True when the batch holds no values.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Adds `value` at the end, or hands it back if the batch is full.
    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.len == N {
            return Err(value);
        }
        self.items[self.len].write(value);
        self.len += 1;
        Ok(())
    }

    /// The value at `index`, if one has been written there.
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        // SAFETY: index < len, and the invariant says items[..len] hold valid values.
        Some(unsafe { self.items[index].assume_init_ref() })
    }
}

impl<T, const N: usize> Default for Batch<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> Drop for Batch<T, N> {
    fn drop(&mut self) {
        let len = self.len;
        // len goes to zero first: if one value's drop panics, the rest leak, which is
        // safe, instead of being dropped a second time.
        self.len = 0;
        for item in &mut self.items[..len] {
            // SAFETY: items[..len] held valid values, and each one is dropped once.
            unsafe { item.assume_init_drop() }
        }
    }
}
