mod aur;
mod error;
mod local;

use alpm::vercmp;
use std::cmp::Ordering;
use std::env;
use std::process::ExitCode;

use crate::aur::request_updates;
use crate::error::{ArgError, R};
use crate::local::find_foreign_packages;

fn main() -> ExitCode {
    match run() {
        Err(e) => {
            eprintln!("\x1b[1;31merror:\x1b[0m {e}");
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
            _ => E!(ArgError::Unknown(arg)),
        }
    }

    println!("\x1b[1;34m::\x1b[0;1m Checking AUR updates...");

    let pkgs = find_foreign_packages(ignore_group.as_deref(), ignore_ends.as_deref())?;
    if pkgs.is_empty() {
        println!("\x1b[0m no packages to check");
        return Ok(());
    }

    let mut updates = request_updates(pkgs.iter().map(|(name, _)| name.as_str()))?;
    let stat: Vec<_> = pkgs
        .into_iter()
        .map(|(name, ver)| match updates.remove(name.as_str()) {
            Some(new_ver) => match vercmp(new_ver.as_str(), ver.as_str()) {
                Ordering::Greater => Status::HasUpdate(name, ver, new_ver),
                _ => Status::UpToDate,
            },
            _ => Status::NotInAUR(name),
        })
        .collect();

    let count = stat
        .iter()
        .filter(|s| matches!(s, Status::HasUpdate(_, _, _)))
        .count();

    if count == 0 {
        println!("\x1b[0m no updates");
    }

    for s in stat {
        match s {
            Status::HasUpdate(name, ver, new_ver) => {
                println!("\x1b[0;1m{name} \x1b[1;31m{ver}\x1b[0m => \x1b[1;32m{new_ver}")
            }
            Status::NotInAUR(name) => println!("\x1b[0;1m{name}\x1b[0m is not in AUR"),
            _ => {}
        }
    }

    print!("\x1b[0m");
    Ok(())
}
