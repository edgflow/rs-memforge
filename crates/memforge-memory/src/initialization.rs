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

    /// Removes the last value and hands it over.
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        // SAFETY: items[len] held a valid value. len was lowered first, so the batch no
        // longer counts it: from here on it belongs to the caller, and is dropped once.
        Some(unsafe { self.items[self.len].assume_init_read() })
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    /// Counts its own drops in a shared cell.
    struct Counted<'a>(&'a Cell<usize>);

    impl Drop for Counted<'_> {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }

    #[test]
    fn every_value_is_dropped_exactly_once() {
        let drops = Cell::new(0);
        let mut batch: Batch<Counted, 8> = Batch::new();
        for _ in 0..8 {
            assert!(batch.push(Counted(&drops)).is_ok());
        }
        drop(batch);
        assert_eq!(drops.get(), 8);
    }

    #[test]
    fn a_partly_filled_batch_drops_only_what_was_written() {
        let drops = Cell::new(0);
        let mut batch: Batch<Counted, 8> = Batch::new();
        for _ in 0..3 {
            assert!(batch.push(Counted(&drops)).is_ok());
        }
        drop(batch);
        assert_eq!(drops.get(), 3);
    }

    #[test]
    fn a_full_batch_hands_the_value_back() {
        let mut batch: Batch<u32, 2> = Batch::new();
        assert_eq!(batch.push(1), Ok(()));
        assert_eq!(batch.push(2), Ok(()));
        assert_eq!(batch.push(3), Err(3));
        assert_eq!(batch.get(2), None);
    }

    #[test]
    fn a_popped_value_is_dropped_once_by_its_new_owner() {
        let drops = Cell::new(0);
        let mut batch: Batch<Counted, 4> = Batch::new();
        assert!(batch.push(Counted(&drops)).is_ok());
        let popped = batch.pop();
        drop(batch);
        assert_eq!(drops.get(), 0, "the batch must not drop what it handed over");
        drop(popped);
        assert_eq!(drops.get(), 1);
    }
}
