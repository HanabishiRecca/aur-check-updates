use alpm::Error as AlpmError;
use curl::Error as CurlError;
use serde_json::Error as JsonError;
use std::{error, fmt, io::Error as IOError, string::FromUtf8Error};

#[macro_export]
macro_rules! E {
    ($e: expr) => {
        return Err($e.into())
    };
}

macro_rules! Error {
    ($($name:ident($err:tt)),+ $(,)?) => {
        #[derive(Debug)]
        pub enum Error {
            $($name($err)),+
        }

        impl error::Error for Error {}

        impl fmt::Display for Error {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                use Error::*;
                match self {
                    $($name(e) => e.fmt(f),)+
                }
            }
        }

        $(impl From<$err> for Error {
            fn from(e: $err) -> Self {
                Self::$name(e)
            }
        })+
    };
}

Error!(
    Arg(ArgError),
    IO(IOError),
    Alpm(AlpmError),
    Request(CurlError),
    Utf8(FromUtf8Error),
    Json(JsonError),
);

pub type R<T> = Result<T, Error>;

#[derive(Debug)]
pub enum ArgError {
    NoValue(String),
    InvalidValue(String, String),
    Unknown(String),
}

impl error::Error for ArgError {}

impl fmt::Display for ArgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ArgError::*;
        match self {
            NoValue(arg) => write!(f, "option '{arg}' requires value"),
            InvalidValue(arg, value) => write!(f, "invalid value '{value}' for option '{arg}'"),
            Unknown(arg) => write!(f, "unknown option '{arg}'"),
        }
    }
}
