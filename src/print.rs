use std::{
    fmt::Display,
    io::{stdout, IsTerminal},
    sync::atomic::{AtomicBool, Ordering::Relaxed},
};

static COLOR: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Copy)]
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
    ($p: ident, $n: expr, $c: expr $(, $rest: expr)* $(,)?) => {
        match COLOR.load(Relaxed) {
            true => $p!($c $(, $rest)*),
            false => $p!($n  $(, $rest)*),
        }
    };
}

macro_rules! P {
    ($n: expr, $c: expr  $(, $rest: expr)* $(,)?) => {
        print_to!(println, $n, $c $(, $rest)*)
    };
}

macro_rules! PE {
    ($n: expr, $c: expr  $(, $rest: expr)* $(,)?) => {
        print_to!(eprintln, $n, $c $(, $rest)*)
    };
}

pub fn header(s: impl Display) {
    P!(":: {s}", "\x1b[34;1m::\x1b[0;1m {s}\x1b[0m");
}

pub fn message(s: impl Display) {
    P!(" {s}", "\x1b[0m {s}\x1b[0m");
}

pub fn package(name: impl Display, ver: impl Display, nlen: usize) {
    P!(
        "{name:0$} {ver}",
        "\x1b[0;1m{name:0$} \x1b[32;1m{ver}\x1b[0m",
        nlen,
    );
}

pub fn update(
    name: impl Display,
    ver: impl Display,
    new_ver: impl Display,
    nlen: usize,
    vlen: usize,
    active: bool,
) {
    let color = match active {
        true => "\x1b[32;1m",
        _ => "\x1b[2m",
    };

    P!(
        "{name:0$} {ver:1$} => {new_ver}",
        "\x1b[0;1m{name:0$} \x1b[31;1m{ver:1$}\x1b[0m => {color}{new_ver}\x1b[0m",
        nlen,
        vlen,
    );
}

pub fn error(e: impl Display) {
    PE!("error: {e}", "\x1b[31;1merror:\x1b[0m {e}\x1b[0m");
}

pub fn warning(w: impl Display) {
    PE!("warning: {w}", "\x1b[33;1mwarning:\x1b[0m {w}\x1b[0m");
}
