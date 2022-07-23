use std::{error, result};
use std::fmt::{Display, Formatter};
use error_code::{self, ErrorCode, ErrorCodeExt};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    // Engine uses plain string as the error.
    #[error("Storage Engine {0}")]
    Engine( String ),
    #[error("Io {0}")]
    Io(#[from] std::io::Error),
    #[error("{0:?}")]
    Other(#[from] Box<dyn error::Error + Sync + Send>),
    // #[error("CF {0} not found")]
    // CFName(String),
    // #[error("The entries of region is unavailable")]
    // EntriesUnavailable,
    // #[error("The entries of region is compacted")]
    // EntriesCompacted,
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::Engine(err)
    }
}

pub type Result<T> = result::Result<T, Error>;

impl ErrorCodeExt for Error {
    fn error_code(&self) -> ErrorCode {
        match self {
            Error::Engine(_) => error_code::engine::ENGINE,
            Error::Io(_) => error_code::engine::IO,
            // Error::CFName(_) => error_code::engine::CF_NAME,
            Error::Other(_) => error_code::UNKNOWN,
            // Error::EntriesUnavailable => error_code::engine::DATALOSS,
            // Error::EntriesCompacted => error_code::engine::DATACOMPACTED,
        }
    }
}

impl From<Error> for String {
    fn from(e: Error) -> String {
        format!("{:?}", e)
    }
}

