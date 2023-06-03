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

fn run(
    Config {
        ignores,
        ignore_groups,
        color_mode,
    }: Config,
) -> R<()> {
    set_color_mode(color_mode);
    print_header("Checking AUR updates...");
    check_updates(find_foreign_packages(ignores, ignore_groups)?)
}

fn get_bin_name() -> Option<String> {
    Some(String::from(current_exe().ok()?.file_name()?.to_str()?))
}

pub fn run_app() -> R<()> {
    if let Some(config) = read_args(args().skip(1))? {
        return run(config);
    }

    println!(
        include_str!("app/help.in"),
        BIN_NAME = get_bin_name().as_deref().unwrap_or(env!("CARGO_BIN_NAME")),
    );

    Ok(())
}
