mod check;
mod cli;
mod local;

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
    match read_args(std::env::args().skip(1))? {
        Some(config) => run(config),
        _ => {
            println!(include_str!("app/help.in"));
            Ok(())
        }
    }
}
