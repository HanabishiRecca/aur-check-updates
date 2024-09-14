mod alpm;
mod aur;
mod cli;
mod io;
mod package;
mod print;
mod request;
mod types;
mod utils;

use crate::{
    print::ColorMode,
    types::{Arr, Str},
};
use std::{env, error::Error, process::ExitCode};

const DEFAULT_DBPATH: &str = "/var/lib/pacman";
const DEFAULT_IGNORE_SUFFIXES: &[&str] = &["-debug"];
const DEFAULT_ENDPOINT: &str = "https://aur.archlinux.org/rpc/v5/info";
const DEFAULT_TIMEOUT: u64 = 5000;

type R = Result<(), Box<dyn Error>>;

pub fn check_updates(pkgs: Arr<(Str, Str)>, endpoint: &str, timeout: u64) -> R {
    if pkgs.is_empty() {
        print::message("no packages to check");
        return Ok(());
    }

    let url = aur::url(endpoint, &pkgs);
    let response = request::send(&url, timeout)?;
    let updates = aur::parse(core::str::from_utf8(&response)?)?;
    let state = package::into_state(pkgs, updates);

    if package::count_updates(&state) == 0 {
        print::message("no updates");
    }

    let (nlen, vlen) = package::calc_lengths(&state);

    for status in state {
        package::print_status(status, nlen, vlen);
    }

    Ok(())
}

fn print_help() {
    let bin = env::current_exe().ok();
    println!(
        include_str!("help.in"),
        PKG = env!("CARGO_PKG_NAME"),
        VER = env!("CARGO_PKG_VERSION"),
        BIN_NAME = (|| bin.as_ref()?.file_name()?.to_str())().unwrap_or(env!("CARGO_BIN_NAME")),
    );
}

fn run() -> R {
    let Some(config) = cli::read_args(std::env::args().skip(1))? else {
        print_help();
        return Ok(());
    };

    print::set_color_mode(config.color_mode().unwrap_or(ColorMode::Auto));
    print::header("Checking AUR updates...");

    let dbpath = config.dbpath().unwrap_or(DEFAULT_DBPATH);
    let ignores = config.ignores().unwrap_or(&[]);
    let ignore_groups = config.ignore_groups().unwrap_or(&[]);

    let ignore_suffixes = match config.ignore_suffixes() {
        Some(s) => s,
        _ => &utils::copy(DEFAULT_IGNORE_SUFFIXES),
    };

    let repos = match config.repos() {
        Some(r) => r,
        _ => &io::find_repos(dbpath)?,
    };

    let packages =
        alpm::find_foreign_packages(dbpath, repos, ignores, ignore_groups, ignore_suffixes)?;

    let endpoint = config.endpoint().unwrap_or(DEFAULT_ENDPOINT);
    let timeout = config.timeout().unwrap_or(DEFAULT_TIMEOUT);
    check_updates(packages, endpoint, timeout)
}

fn main() -> ExitCode {
    print::set_color_mode(ColorMode::Auto);
    match run() {
        Err(e) => {
            print::error(e);
            ExitCode::FAILURE
        }
        _ => ExitCode::SUCCESS,
    }
}
