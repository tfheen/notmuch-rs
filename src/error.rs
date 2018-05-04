use std;
use std::{
    error,
    fmt,
    io,
    result,
};
use std::sync::TryLockError;

use ffi;

pub type Result<T> = result::Result<T, Error>;


#[derive(Debug)]
pub struct LockError{
    description: String
}

impl fmt::Display for LockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

// This is important for other errors to wrap this one.
impl error::Error for LockError {
    fn description(&self) -> &str {
        self.description.as_str()
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}


#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    NotmuchError(ffi::Status),
    RwLockError(LockError)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", error::Error::description(self))
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IoError(ref e) => error::Error::description(e),
            Error::NotmuchError(ref e) => e.description(),
            Error::RwLockError(ref e) => error::Error::description(e),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::IoError(ref e) => Some(e),
            Error::NotmuchError(ref e) => Some(e),
            Error::RwLockError(ref e) => Some(e),
        }
    }
}

impl std::convert::From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl std::convert::From<ffi::Status> for Error {
    fn from(err: ffi::Status) -> Error {
        Error::NotmuchError(err)
    }
}

impl std::convert::From<ffi::notmuch_status_t> for Error {
    fn from(err: ffi::notmuch_status_t) -> Error {
        Error::NotmuchError(ffi::Status::from(err))
    }
}

impl<T> std::convert::From<TryLockError<T>> for Error {
    fn from(err: TryLockError<T>) -> Error {
        Error::RwLockError(LockError{description: error::Error::description(&err).to_string()})
    }
}
