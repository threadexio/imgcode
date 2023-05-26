use std::io;

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    SizeLimit,
    UnsupportedFormat,
    Io(io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SizeLimit => write!(f, "size limit exceeded"),
            Self::UnsupportedFormat => write!(f, "unsupported image format"),
            Self::Io(io_err) => write!(f, "{io_err}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
