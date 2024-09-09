mod check;
mod cli;
mod local;

use std::env::{args, current_exe};

use crate::{
    error::R,
    print::{print_header, set_color_mode},
};
use check::check_updates;
use cli::{read_args, Config};
use local::find_foreign_packages;

fn run(config: Config) -> R<()> {
    set_color_mode(config.color_mode);
    print_header("Checking AUR updates...");

    let dbpath = config
        .dbpath
        .as_deref()
        .unwrap_or(crate::consts::DEFAULT_DBPATH);

    let mut repos = config.repos;
    if repos.is_empty() {
        repos = local::find_repos(dbpath)?;
    }

    check_updates(
        find_foreign_packages(dbpath, repos, config.ignores, config.ignore_groups)?,
        config.timeout,
    )
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
