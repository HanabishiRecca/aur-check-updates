mod app;
mod error;
mod print;

use std::process::ExitCode;

use app::*;
use print::*;

fn main() -> ExitCode {
    set_color_mode(ColorMode::Auto);
    match run_app() {
        Err(e) => {
            print_error(e);
            ExitCode::FAILURE
        }
        _ => ExitCode::SUCCESS,
    }
}
