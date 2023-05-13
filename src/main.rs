mod aur;
mod check;
mod error;
mod local;
mod print;

use std::collections::HashSet;
use std::env;
use std::process::ExitCode;

use crate::check::check_updates;
use crate::error::{ArgError, R};
use crate::local::find_foreign_packages;
use crate::print::ColorMode;

fn main() -> ExitCode {
    print::set_color_mode(ColorMode::Auto);
    match run() {
        Err(e) => {
            print::error(format_args!("{e}"));
            ExitCode::FAILURE
        }
        _ => ExitCode::SUCCESS,
    }
}

fn run() -> R<()> {
    let mut ignores = HashSet::new();
    let mut ignore_groups = HashSet::new();
    let mut args = env::args().skip(1);

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
                ignores.extend(next!().split(',').map(String::from));
            }
            "--ignoregroup" => {
                ignore_groups.extend(next!().split(',').map(String::from));
            }
            "--color" => {
                let value = next!();
                use ColorMode::*;
                print::set_color_mode(match value.as_str() {
                    "auto" => Auto,
                    "always" => Always,
                    "never" => Never,
                    _ => E!(ArgError::InvalidValue(arg, value)),
                });
            }
            _ => E!(ArgError::Unknown(arg)),
        }
    }

    print::header(format_args!("Checking AUR updates..."));
    check_updates(find_foreign_packages(ignores, ignore_groups)?)
}
