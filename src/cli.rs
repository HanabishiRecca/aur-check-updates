use crate::{
    print::ColorMode,
    types::{Arr, Str},
};
use std::{error, fmt};

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Config {
    ignores: Option<Arr<Str>>,
    ignore_groups: Option<Arr<Str>>,
    color_mode: Option<ColorMode>,
    dbpath: Option<Str>,
    repos: Option<Arr<Str>>,
    endpoint: Option<Str>,
    timeout: Option<u64>,
}

impl Config {
    fn new() -> Self {
        Config {
            ignores: None,
            ignore_groups: None,
            color_mode: None,
            dbpath: None,
            repos: None,
            endpoint: None,
            timeout: None,
        }
    }

    pub fn ignores(&self) -> Option<&[impl AsRef<str>]> {
        self.ignores.as_deref()
    }

    pub fn ignore_groups(&self) -> Option<&[impl AsRef<str>]> {
        self.ignore_groups.as_deref()
    }

    pub fn color_mode(&self) -> Option<ColorMode> {
        self.color_mode
    }

    pub fn dbpath(&self) -> Option<&str> {
        self.dbpath.as_deref()
    }

    pub fn repos(&self) -> Option<&[impl AsRef<str>]> {
        self.repos.as_deref()
    }

    pub fn endpoint(&self) -> Option<&str> {
        self.endpoint.as_deref()
    }

    pub fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

#[derive(Debug)]
pub enum Error {
    NoValue(Str),
    InvalidValue(Str, Str),
    Unknown(Str),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            NoValue(arg) => write!(f, "option '{arg}' requires value"),
            InvalidValue(arg, value) => write!(f, "invalid value '{value}' for option '{arg}'"),
            Unknown(arg) => write!(f, "unknown option '{arg}'"),
        }
    }
}

macro_rules! E {
    ($e: expr) => {{
        use Error::*;
        return Err($e);
    }};
}

fn parse_list<'a, T: FromIterator<impl From<&'a str>>>(str: &'a str) -> T {
    str.split(',')
        .filter(|s| !s.is_empty())
        .map(From::from)
        .collect()
}

macro_rules! F {
    ($s: expr) => {
        From::from($s.as_ref())
    };
}

pub fn read_args(mut args: impl Iterator<Item = impl AsRef<str>>) -> Result<Option<Config>, Error> {
    let mut config = Config::new();

    while let Some(arg) = args.next() {
        macro_rules! next {
            () => {
                match args.next() {
                    Some(value) => value,
                    _ => E!(NoValue(F!(arg))),
                }
            };
        }
        macro_rules! list {
            () => {
                parse_list(next!().as_ref())
            };
        }
        match arg.as_ref() {
            "" => {}
            "--ignore" => {
                config.ignores = Some(list!());
            }
            "--ignoregroup" => {
                config.ignore_groups = Some(list!());
            }
            "--color" => {
                let value = next!();
                use ColorMode::*;
                config.color_mode = Some(match value.as_ref() {
                    "auto" => Auto,
                    "always" => Always,
                    "never" => Never,
                    _ => E!(InvalidValue(F!(arg), F!(value))),
                });
            }
            "--dbpath" => {
                config.dbpath = Some(F!(next!()));
            }
            "--repos" => {
                config.repos = Some(list!());
            }
            "--endpoint" => {
                config.endpoint = Some(F!(next!()));
            }
            "--timeout" => {
                let value = next!();
                config.timeout = Some(match value.as_ref().parse() {
                    Ok(t) => t,
                    _ => E!(InvalidValue(F!(arg), F!(value))),
                });
            }
            "-h" | "--help" => return Ok(None),
            _ => E!(Unknown(F!(arg))),
        }
    }

    Ok(Some(config))
}

#[cfg(test)]
mod tests;
