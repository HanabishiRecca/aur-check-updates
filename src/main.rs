mod alpm;
mod app;
mod aur;
mod cli;
mod io;
mod package;
mod print;
mod request;
mod types;
mod utils;

use std::process::ExitCode;

fn main() -> ExitCode {
    match app::run() {
        Ok(help) => {
            help.then(print::help);
            ExitCode::SUCCESS
        }
        Err(e) => {
            print::error(e);
            ExitCode::FAILURE
        }
    }
}
