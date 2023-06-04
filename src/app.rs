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

pub fn run_app() -> R<()> {
    if let Some(config) = read_args(args().skip(1))? {
        return run(config);
    }

    let bin = current_exe().ok();
    println!(
        include_str!("app/help.in"),
        BIN_NAME = (|| bin.as_ref()?.file_name()?.to_str())().unwrap_or(env!("CARGO_BIN_NAME")),
    );

    Ok(())
}
