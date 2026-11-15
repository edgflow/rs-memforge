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

/// The `len` bytes at `ptr`, borrowed for the length of a call.
///
/// # Safety
/// If `len` is not zero, `ptr` must point to `len` readable bytes that nothing changes
/// while the returned slice is in use.
unsafe fn bytes<'a>(ptr: *const u8, len: usize) -> Result<&'a [u8], c_int> {
    if len == 0 {
        return Ok(&[]);
    }
    if ptr.is_null() {
        return Err(MF_NULL);
    }
    if len > isize::MAX as usize {
        return Err(MF_BAD_LENGTH);
    }
    // SAFETY: ptr is not null, a u8 needs no alignment, len fits in an isize, and the
    // caller promises len readable bytes that stay unchanged.
    Ok(unsafe { std::slice::from_raw_parts(ptr, len) })
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

/// Stores `value` under `key`.
///
/// # Safety
/// `db` must come from `mf_open` and not be closed. `key` and `value` must point to
/// `key_len` and `value_len` readable bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mf_put(
    db: *mut Database,
    key: *const u8,
    key_len: usize,
    value: *const u8,
    value_len: usize,
) -> c_int {
    guard(|| {
        // SAFETY: the caller promises that db came from mf_open, is not closed, and is not
        // used by anyone else during this call.
        let Some(db) = (unsafe { db.as_mut() }) else {
            return MF_NULL;
        };
        // SAFETY: the caller promises key_len readable bytes at key.
        let key = match unsafe { bytes(key, key_len) } {
            Ok(key) => key,
            Err(code) => return code,
        };
        // SAFETY: the caller promises value_len readable bytes at value.
        let value = match unsafe { bytes(value, value_len) } {
            Ok(value) => value,
            Err(code) => return code,
        };
        match db.put(key, value) {
            Ok(()) => MF_OK,
            Err(e) => code(&e),
        }
    })
}

/// Copies `key`'s value into `buf`, and its length into `*value_len`, even when `buf` is
/// too small for it.
///
/// # Safety
/// `db` must come from `mf_open` and not be closed. `key` must point to `key_len` readable
/// bytes, `buf` to `buf_len` writable bytes, and `value_len` to a writable size_t.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mf_get(
    db: *const Database,
    key: *const u8,
    key_len: usize,
    buf: *mut u8,
    buf_len: usize,
    value_len: *mut usize,
) -> c_int {
    guard(|| {
        // SAFETY: the caller promises that db came from mf_open and is not closed.
        let Some(db) = (unsafe { db.as_ref() }) else {
            return MF_NULL;
        };
        if value_len.is_null() {
            return MF_NULL;
        }
        // SAFETY: the caller promises key_len readable bytes at key.
        let key = match unsafe { bytes(key, key_len) } {
            Ok(key) => key,
            Err(code) => return code,
        };
        let copied = db.get_with(key, |value| {
            // SAFETY: value_len is not null, and the caller promises it is writable.
            unsafe { value_len.write(value.len()) };
            if value.len() > buf_len {
                return MF_BUFFER_TOO_SMALL;
            }
            if value.is_empty() {
                return MF_OK;
            }
            if buf.is_null() {
                return MF_NULL;
            }
            // SAFETY: buf is not null, the caller promises buf_len writable bytes, the
            // value fits in them, and a C buffer cannot overlap a page MEMFORGE owns.
            unsafe { std::ptr::copy_nonoverlapping(value.as_ptr(), buf, value.len()) };
            MF_OK
        });
        match copied {
            Ok(Some(code)) => code,
            Ok(None) => MF_NOT_FOUND,
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

/// Panics inside MEMFORGE, to show what a panic does at the boundary. Guarded (non-zero),
/// it returns MF_PANIC; unguarded, the process aborts.
#[unsafe(no_mangle)]
pub extern "C" fn mf_demo_panic(guarded: c_int) -> c_int {
    if guarded != 0 {
        guard(|| panic!("a bug inside MEMFORGE"))
    } else {
        panic!("a bug inside MEMFORGE")
    }
}
