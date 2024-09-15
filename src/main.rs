mod alpm;
mod aur;
mod cli;
mod io;
mod package;
mod print;
mod request;
mod types;
mod utils;

use crate::print::ColorMode;
use std::{env, error::Error, process::ExitCode};

const DEFAULT_COLOR_MODE: ColorMode = ColorMode::Auto;
const DEFAULT_DBPATH: &str = "/var/lib/pacman";
const DEFAULT_IGNORES: &[&str] = &[];
const DEFAULT_IGNORE_GROUPS: &[&str] = &[];
const DEFAULT_IGNORE_SUFFIXES: &[&str] = &["-debug"];
const DEFAULT_ENDPOINT: &str = "https://aur.archlinux.org/rpc/v5/info";
const DEFAULT_TIMEOUT: u64 = 5000;
const DEFAULT_SHOW_UPDATED: bool = false;
const DEFAULT_SHOW_FAILED: bool = true;

type R = Result<(), Box<dyn Error>>;

macro_rules! default {
    ($option: expr, $default: expr $(,)?) => {
        match $option {
            Some(value) => value,
            _ => $default,
        }
    };
}

fn print_help() {
    let bin = env::current_exe().ok();
    println!(
        include_str!("help.in"),
        PKG = env!("CARGO_PKG_NAME"),
        VER = env!("CARGO_PKG_VERSION"),
        BIN_NAME = default!(
            (|| bin.as_ref()?.file_name()?.to_str())(),
            env!("CARGO_BIN_NAME")
        ),
    );
}

fn run() -> R {
    let Some(config) = cli::read_args(env::args().skip(1))? else {
        print_help();
        return Ok(());
    };

    print::set_color_mode(default!(config.color_mode(), DEFAULT_COLOR_MODE));
    print::header("Checking AUR updates...");

    let dbpath = default!(config.dbpath(), DEFAULT_DBPATH);
    let repos = default!(config.repos(), &io::find_repos(dbpath)?);
    let ignores = default!(config.ignores(), &utils::copy(DEFAULT_IGNORES));
    let ignore_groups = default!(config.ignore_groups(), &utils::copy(DEFAULT_IGNORE_GROUPS));
    let ignore_suffixes = default!(
        config.ignore_suffixes(),
        &utils::copy(DEFAULT_IGNORE_SUFFIXES)
    );

    let packages =
        alpm::find_foreign_packages(dbpath, repos, ignores, ignore_groups, ignore_suffixes)?;

    if packages.is_empty() {
        print::message("no packages to check");
        return Ok(());
    }

    let url = aur::url(default!(config.endpoint(), DEFAULT_ENDPOINT), &packages);
    let response = request::send(&url, default!(config.timeout(), DEFAULT_TIMEOUT))?;
    let updates = aur::parse(core::str::from_utf8(&response)?)?;

    let state = package::into_state(
        packages,
        updates,
        default!(config.show_updated(), DEFAULT_SHOW_UPDATED),
        default!(config.show_failed(), DEFAULT_SHOW_FAILED),
    );

    if package::count_updates(&state) == 0 {
        print::message("no updates");
    }

    let (nlen, vlen) = package::calc_lengths(&state);

    for pkg in state {
        package::print_status(pkg, nlen, vlen);
    }

    Ok(())
}

fn main() -> ExitCode {
    print::set_color_mode(DEFAULT_COLOR_MODE);
    match run() {
        Err(e) => {
            print::error(e);
            ExitCode::FAILURE
        }
        _ => ExitCode::SUCCESS,
    }
}
