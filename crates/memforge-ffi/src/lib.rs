//! MEMFORGE's C door: `extern "C"` functions that a C program can call, declared for C in
//! `include/memforge.h`. Every function returns MF_OK, MF_NOT_FOUND, or a negative error
//! code, and never lets a panic cross into C.

use memforge_core::Error;
use memforge_engine::Database;
use std::ffi::{CStr, OsStr, c_char, c_int};
use std::os::unix::ffi::OsStrExt;
use std::panic::{self, AssertUnwindSafe};
use std::path::Path;

pub const MF_OK: c_int = 0;
pub const MF_NOT_FOUND: c_int = 1;
pub const MF_NULL: c_int = -1;
pub const MF_BAD_LENGTH: c_int = -2;
pub const MF_TOO_LARGE: c_int = -3;
pub const MF_IO: c_int = -4;
pub const MF_CORRUPT: c_int = -5;
pub const MF_BUFFER_TOO_SMALL: c_int = -6;
pub const MF_PANIC: c_int = -7;

/// The error code C sees for a MEMFORGE error.
fn code(e: &Error) -> c_int {
    match e {
        Error::RecordTooLarge { .. } => MF_TOO_LARGE,
        Error::CorruptPage { .. } => MF_CORRUPT,
        Error::Io(_) | Error::NotImplemented(_) => MF_IO,
    }
}

/// Runs `f`, and turns a panic into MF_PANIC: unwinding must never cross into C.
fn guard(f: impl FnOnce() -> c_int) -> c_int {
    panic::catch_unwind(AssertUnwindSafe(f)).unwrap_or(MF_PANIC)
}

/// Opens the database in the directory `dir`, and stores its handle in `*out`.
///
/// # Safety
/// `dir` must be a NUL-terminated string, and `out` must point to a writable pointer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mf_open(dir: *const c_char, out: *mut *mut Database) -> c_int {
    guard(|| {
        if dir.is_null() || out.is_null() {
            return MF_NULL;
        }
        // SAFETY: dir is not null, and the caller promises a NUL-terminated string.
        let dir = unsafe { CStr::from_ptr(dir) };
        match Database::open(Path::new(OsStr::from_bytes(dir.to_bytes()))) {
            Ok(db) => {
                // SAFETY: out is not null, and the caller promises it is writable.
                unsafe { out.write(Box::into_raw(Box::new(db))) };
                MF_OK
            }
            Err(e) => code(&e),
        }
    })
}

/// Flushes and closes the database, and frees its handle.
///
/// # Safety
/// `db` must come from `mf_open`, and must never be used again after this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mf_close(db: *mut Database) -> c_int {
    guard(|| {
        if db.is_null() {
            return MF_NULL;
        }
        // SAFETY: the caller promises db came from mf_open and is never used again, so
        // Box::from_raw takes ownership back exactly once.
        let db = unsafe { Box::from_raw(db) };
        match db.close() {
            Ok(()) => MF_OK,
            Err(e) => code(&e),
        }
    })
}
