//! The types every MEMFORGE layer shares. Nothing here needs unsafe code,
//! and `forbid` makes the compiler hold us to that.
#![forbid(unsafe_code)]

pub mod error;
pub mod record;

pub use error::{Error, Result};
pub use record::Record;
