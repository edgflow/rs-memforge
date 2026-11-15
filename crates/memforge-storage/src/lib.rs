//! The storage layer: what the bytes of a page mean. It uses the memory layer's safe
//! views, and never needs unsafe code itself.
#![forbid(unsafe_code)]

pub mod codec;
pub mod page;
