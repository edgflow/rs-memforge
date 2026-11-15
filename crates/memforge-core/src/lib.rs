//! The types every MEMFORGE layer shares. Nothing here needs unsafe code,
//! and `forbid` makes the compiler hold us to that.
#![forbid(unsafe_code)]

pub mod error;

pub use error::{Error, Result};
