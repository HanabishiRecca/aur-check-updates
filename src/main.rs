mod aur;
mod error;
mod local;
mod print;

use alpm::vercmp;
use std::cmp::Ordering;
use std::env;
use std::process::ExitCode;

use crate::aur::request_updates;
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

enum Status {
    UpToDate,
    HasUpdate(String, String, String),
    NotInAUR(String),
}

fn run() -> R<()> {
    let mut ignore_group = None;
    let mut ignore_ends = None;
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
            "--ignore-group" => {
                ignore_group = Some(next!());
            }
            "--ignore-ends" => {
                ignore_ends = Some(next!());
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

    let pkgs = find_foreign_packages(ignore_group.as_deref(), ignore_ends.as_deref())?;
    if pkgs.is_empty() {
        print::message(format_args!("no packages to check"));
        return Ok(());
    }

    let mut updates = request_updates(pkgs.iter().map(|(name, _)| name.as_str()))?;

    use Status::*;
    let stat: Vec<_> = pkgs
        .into_iter()
        .map(|(name, ver)| match updates.remove(name.as_str()) {
            Some(new_ver) => match vercmp(new_ver.as_str(), ver.as_str()) {
                Ordering::Greater => HasUpdate(name, ver, new_ver),
                _ => UpToDate,
            },
            _ => NotInAUR(name),
        })
        .collect();

    let count = stat
        .iter()
        .filter(|s| matches!(s, HasUpdate(_, _, _)))
        .count();

    if count == 0 {
        print::message(format_args!("no updates"));
    }

    for s in stat {
        match s {
            HasUpdate(name, ver, new_ver) => print::update(
                format_args!("{name}"),
                format_args!("{ver}"),
                format_args!("{new_ver}"),
            ),
            NotInAUR(name) => print::package(format_args!("{name}"), format_args!("is not in AUR")),
            _ => {}
        }
    }

    Ok(())
}
