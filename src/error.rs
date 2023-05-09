use alpm::Error as AlpmError;
use curl::Error as CurlError;
use serde_json::Error as JsonError;
use std::{error, fmt, string::FromUtf8Error};

#[macro_export]
macro_rules! E {
    ($e: expr) => {
        return Err($e.into())
    };
}

#[derive(Debug)]
pub enum Error {
    Arg(ArgError),
    Plain(String),
    Alpm(AlpmError),
    Request(CurlError),
    Utf8(FromUtf8Error),
    Json(JsonError),
}

pub type R<T> = Result<T, Error>;

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            Arg(e) => e.fmt(f),
            Plain(e) => e.fmt(f),
            Alpm(e) => e.fmt(f),
            Request(e) => e.fmt(f),
            Utf8(e) => e.fmt(f),
            Json(e) => e.fmt(f),
        }
    }
}

impl From<ArgError> for Error {
    fn from(e: ArgError) -> Self {
        Self::Arg(e)
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Self::Plain(e)
    }
}

impl From<AlpmError> for Error {
    fn from(e: AlpmError) -> Self {
        Self::Alpm(e)
    }
}

impl From<CurlError> for Error {
    fn from(e: CurlError) -> Self {
        Self::Request(e)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(e: FromUtf8Error) -> Self {
        Self::Utf8(e)
    }
}

impl From<JsonError> for Error {
    fn from(e: JsonError) -> Self {
        Self::Json(e)
    }
}

#[derive(Debug)]
pub enum ArgError {
    NoValue(String),
    Unknown(String),
}

impl error::Error for ArgError {}

impl fmt::Display for ArgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ArgError::*;
        match self {
            NoValue(arg) => write!(f, "option '{arg}' requires value"),
            Unknown(arg) => write!(f, "unknown option '{arg}'"),
        }
    }
}
