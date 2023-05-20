use std::collections::HashSet;

use crate::{error::*, print::*, E};

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Config {
    pub ignores: HashSet<String>,
    pub ignore_groups: HashSet<String>,
    pub color_mode: ColorMode,
}

impl Config {
    fn new() -> Self {
        Config {
            ignores: HashSet::new(),
            ignore_groups: HashSet::new(),
            color_mode: ColorMode::Auto,
        }
    }
}

pub fn read_args(mut args: impl Iterator<Item = String>) -> R<Option<Config>> {
    let mut config = Config::new();

    while let Some(arg) = args.next() {
        macro_rules! next {
            () => {
                match args.next() {
                    Some(value) => value,
                    _ => E!(ArgError::NoValue(arg)),
                }
            };
        }
        macro_rules! extend {
            ($h:expr) => {
                $h.extend(
                    next!()
                        .split(',')
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .map(String::from),
                )
            };
        }
        match arg.as_str().trim() {
            "--ignore" => extend!(config.ignores),
            "--ignoregroup" => extend!(config.ignore_groups),
            "--color" => {
                let value = next!();
                use ColorMode::*;
                config.color_mode = match value.as_str().trim() {
                    "auto" => Auto,
                    "always" => Always,
                    "never" => Never,
                    _ => E!(ArgError::InvalidValue(arg, value)),
                };
            }
            "-h" | "--help" => return Ok(None),
            _ => E!(ArgError::Unknown(arg)),
        }
    }

    Ok(Some(config))
}

#[cfg(test)]
mod tests;
