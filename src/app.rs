mod check;
mod cli;
mod local;

use crate::{error::*, print::*};
use check::*;
use cli::*;
use local::*;

pub fn run_app() -> R<()> {
    let Config {
        ignores,
        ignore_groups,
    } = match read_args(std::env::args().skip(1))? {
        Some(c) => c,
        _ => return Ok(()),
    };

    header(format_args!("Checking AUR updates..."));
    check_updates(find_foreign_packages(ignores, ignore_groups)?)
}
