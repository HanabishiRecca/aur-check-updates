mod check;
mod cli;
mod local;

use std::env::{args, current_exe};

use crate::{
    error::R,
    print::{print_header, set_color_mode, ColorMode},
};
use check::check_updates;
use cli::{read_args, Config};
use local::find_foreign_packages;

const DEFAULT_DBPATH: &str = "/var/lib/pacman";
const DEFAULT_ENDPOINT: &str = "https://aur.archlinux.org/rpc/v5/info";
const DEFAULT_TIMEOUT: u64 = 5000;

fn run(config: Config) -> R<()> {
    set_color_mode(config.color_mode().unwrap_or(ColorMode::Auto));
    print_header("Checking AUR updates...");

    let dbpath = config.dbpath().unwrap_or(DEFAULT_DBPATH);
    let ignores = config.ignores().unwrap_or(&[]);
    let ignore_groups = config.ignore_groups().unwrap_or(&[]);

    let packages = match config.repos() {
        Some(repos) => find_foreign_packages(dbpath, repos, ignores, ignore_groups)?,
        _ => find_foreign_packages(dbpath, &local::find_repos(dbpath)?, ignores, ignore_groups)?,
    };

    let endpoint = config.endpoint().unwrap_or(DEFAULT_ENDPOINT);
    let timeout = config.timeout().unwrap_or(DEFAULT_TIMEOUT);
    check_updates(packages, endpoint, timeout)
}

fn print_help() {
    let bin = current_exe().ok();
    println!(
        include_str!("app/help.in"),
        PKG = env!("CARGO_PKG_NAME"),
        VER = env!("CARGO_PKG_VERSION"),
        BIN_NAME = (|| bin.as_ref()?.file_name()?.to_str())().unwrap_or(env!("CARGO_BIN_NAME")),
    );
}

pub fn run_app() -> R<()> {
    match read_args(args().skip(1))? {
        Some(config) => run(config)?,
        _ => print_help(),
    }

    Ok(())
}
