//! The database engine. It is built only from safe abstractions:
//! the unsafe code lives in the memory layer, never here.
#![forbid(unsafe_code)]

mod database;
pub mod datafile;
pub mod entry;
pub mod index;
pub mod recovery;

pub use database::Database;
pub use recovery::Recovery;
