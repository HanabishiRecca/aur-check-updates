use std::{
    fmt::Display,
    io::{stdout, IsTerminal},
    sync::atomic::{AtomicBool, Ordering::Relaxed},
};

static COLOR: AtomicBool = AtomicBool::new(false);

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum ColorMode {
    Auto,
    Always,
    Never,
}

pub fn set_color_mode(mode: ColorMode) {
    use ColorMode::*;
    COLOR.store(
        match mode {
            Auto => stdout().is_terminal(),
            Always => true,
            Never => false,
        },
        Relaxed,
    );
}

macro_rules! print_to {
    ($p: ident, $n: expr, $c: expr) => {
        match COLOR.load(Relaxed) {
            true => $p!($c),
            false => $p!($n),
        }
    };
}

macro_rules! P {
    ($n: expr, $c: expr $(,)?) => {
        print_to!(println, $n, $c)
    };
}

macro_rules! PE {
    ($n: expr, $c: expr $(,)?) => {
        print_to!(eprintln, $n, $c)
    };
}

pub fn print_header(s: impl Display) {
    P!(":: {s}", "\x1b[34;1m::\x1b[0;1m {s}\x1b[0m");
}

pub fn print_message(s: impl Display) {
    P!(" {s}", "\x1b[0m {s}\x1b[0m");
}

pub fn print_update(name: impl Display, ver: impl Display, new_ver: impl Display) {
    P!(
        "{name} {ver} => {new_ver}",
        "\x1b[0;1m{name} \x1b[31;1m{ver}\x1b[0m => \x1b[32;1m{new_ver}\x1b[0m",
    );
}

pub fn print_package(name: impl Display, s: impl Display) {
    P!("{name} {s}", "\x1b[0;1m{name}\x1b[0m {s}\x1b[0m");
}

pub fn print_error(e: impl Display) {
    PE!("error: {e}", "\x1b[31;1merror:\x1b[0m {e}\x1b[0m");
}

pub fn print_warning(w: impl Display) {
    PE!("warning: {w}", "\x1b[33m;1mwarning:\x1b[0m {w}\x1b[0m");
}
