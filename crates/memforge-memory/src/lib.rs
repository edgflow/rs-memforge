//! The memory layer. Besides the C door, this is the only MEMFORGE crate that may
//! contain unsafe code, and everything it offers to the layers above is safe.

pub mod layout;
