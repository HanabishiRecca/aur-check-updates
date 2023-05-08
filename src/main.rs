mod aur;
mod error;
mod local;

use alpm::vercmp;
use std::cmp::Ordering;
use std::process::ExitCode;

use crate::aur::request_updates;
use crate::error::R;
use crate::local::find_foreign_packages;

fn main() -> ExitCode {
    match run() {
        Err(e) => {
            println!("\x1b[1;31merror:\x1b[0m {e}");
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
    println!("\x1b[1;34m::\x1b[0;1m Checking AUR updates...");

    let pkgs = find_foreign_packages()?;
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

    Ok(())
}
