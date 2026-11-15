//! The database engine. It is built only from safe abstractions:
//! the unsafe code lives in the memory layer, never here.
#![forbid(unsafe_code)]

mod database;

pub use database::Database;
