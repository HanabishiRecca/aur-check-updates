use std::collections::HashSet;

use crate::{error::*, print::*, E};

pub struct Config {
    pub ignores: HashSet<String>,
    pub ignore_groups: HashSet<String>,
}

pub fn read_args(mut args: impl Iterator<Item = String>) -> R<Option<Config>> {
    let mut config = Config {
        ignores: HashSet::new(),
        ignore_groups: HashSet::new(),
    };

    while let Some(arg) = args.next() {
        macro_rules! next {
            () => {
                match args.next() {
                    Some(value) => value,
                    _ => E!(ArgError::NoValue(arg)),
                }
            };
        }
        match arg.as_str() {
            "--ignore" => {
                config.ignores.extend(next!().split(',').map(String::from));
            }
            "--ignoregroup" => {
                config
                    .ignore_groups
                    .extend(next!().split(',').map(String::from));
            }
            "--color" => {
                let value = next!();
                use ColorMode::*;
                set_color_mode(match value.as_str() {
                    "auto" => Auto,
                    "always" => Always,
                    "never" => Never,
                    _ => E!(ArgError::InvalidValue(arg, value)),
                });
            }
            "-h" | "--help" => {
                println!(include_str!("help.in"));
                return Ok(None);
            }
            _ => E!(ArgError::Unknown(arg)),
        }
    }

    Ok(Some(config))
}
