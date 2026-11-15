use std::fmt;

/// Everything that can go wrong in MEMFORGE.
#[derive(Debug)]
pub enum Error {
    /// The operating system refused a file operation.
    Io(std::io::Error),
    /// This part of the engine is not built yet.
    NotImplemented(&'static str),
    /// A record is larger than any page can hold.
    RecordTooLarge { len: usize },
    /// A page read back from the file failed its checks.
    CorruptPage { page: u64, reason: String },
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {e}"),
            Error::NotImplemented(what) => write!(f, "{what} is not implemented yet"),
            Error::RecordTooLarge { len } => {
                write!(f, "a record of {len} bytes is larger than a page")
            }
            Error::CorruptPage { page, reason } => {
                write!(f, "page {page} is corrupt: {reason}")
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}
