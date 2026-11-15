//! MEMFORGE: a small persistent key-value storage engine.
//! This crate is the front door: it re-exports what an application needs.
#![forbid(unsafe_code)]

pub use memforge_core::{Error, Result};
pub use memforge_engine::Database;
