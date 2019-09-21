use abi_utils::Error as AbiError;
use ethabi::Error as AbiDecodeError;
use failure::{Backtrace, Context, Fail};
use plasma_core::data_structure::error::Error as PlasmaCoreError;
use plasma_db::error::Error as PlasmaDbError;
use std::fmt;
use std::fmt::Display;
use std::io::Error as IoError;

/// error definition for plasma core.
#[derive(Fail, Debug)]
pub enum ErrorKind {
    #[fail(display = "IO error")]
    Io,
    #[fail(display = "ABI Decode error")]
    AbiDecode,
    #[fail(display = "ABI error")]
    AbiError,
    #[fail(display = "Plasma Core error")]
    PlasmaCoreError,
    #[fail(display = "Plasma Db error")]
    PlasmaDbError,
    #[fail(display = "Invalid Preimage")]
    InvalidPreimage,
    #[fail(display = "Undecided")]
    Undecided,
    #[fail(display = "CannotDecide")]
    CannotDecide,
}

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

impl Fail for Error {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl Error {
    pub fn new(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }

    pub fn kind(&self) -> &ErrorKind {
        self.inner.get_context()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Error {
        Error {
            inner: error.context(ErrorKind::Io),
        }
    }
}

impl From<AbiDecodeError> for Error {
    fn from(_error: AbiDecodeError) -> Error {
        Error {
            inner: Context::from(ErrorKind::AbiDecode),
        }
    }
}

impl From<AbiError> for Error {
    fn from(_error: AbiError) -> Error {
        Error {
            inner: Context::from(ErrorKind::AbiError),
        }
    }
}

impl From<PlasmaDbError> for Error {
    fn from(error: PlasmaDbError) -> Error {
        Error {
            inner: error.context(ErrorKind::PlasmaDbError),
        }
    }
}

impl From<PlasmaCoreError> for Error {
    fn from(error: PlasmaCoreError) -> Error {
        Error {
            inner: error.context(ErrorKind::PlasmaCoreError),
        }
    }
}
