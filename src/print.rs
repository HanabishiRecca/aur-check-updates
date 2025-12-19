use crate::package::Pkg;
use crate::utils;
use std::env;
use std::fmt::Display;
use std::io::{self, IsTerminal};
use std::sync::atomic::{AtomicBool, Ordering};

pub fn help() {
    let bin = env::current_exe().ok();
    println!(
        include_str!("help.in"),
        PKG = env!("CARGO_PKG_NAME"),
        VER = env!("CARGO_PKG_VERSION"),
        BIN_NAME = (|| bin.as_ref()?.file_name()?.to_str())().unwrap_or(env!("CARGO_BIN_NAME")),
    );
}

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
    let color = match mode {
        Auto => io::stdout().is_terminal(),
        Always => true,
        Never => false,
    };
    COLOR.store(color, Ordering::Relaxed);
}

macro_rules! print_to {
    ($p: ident, $n: expr, $c: expr $(, $rest: expr)* $(,)?) => {
        if COLOR.load(Ordering::Relaxed) { $p!($c $(, $rest)*) }
        else { $p!($n  $(, $rest)*) }
    };
}

macro_rules! stdout {
    ($n: expr, $c: expr  $(, $rest: expr)* $(,)?) => {
        print_to!(println, $n, $c $(, $rest)*)
    };
}

macro_rules! stderr {
    ($n: expr, $c: expr  $(, $rest: expr)* $(,)?) => {
        print_to!(eprintln, $n, $c $(, $rest)*)
    };
}

pub fn header(s: &str) {
    stdout!(":: {s}", "\x1b[34;1m::\x1b[0;1m {s}\x1b[0m");
}

pub fn message(s: &str) {
    stdout!(" {s}", "\x1b[0m {s}\x1b[0m");
}

pub fn package(pkg: &Pkg, nlen: usize) {
    stdout!("{:nlen$} {}", "\x1b[0;1m{:nlen$} \x1b[32;1m{}\x1b[0m", pkg.name(), pkg.ver());
}

pub fn update(pkg: &Pkg, ver: &str, nlen: usize, vlen: usize) {
    if !COLOR.load(Ordering::Relaxed) {
        println!("{:nlen$} {:vlen$} -> {ver}", pkg.name(), pkg.ver());
        return;
    }

    let pos = utils::str_diff(pkg.ver(), ver);
    let (va, vb) = pkg.ver().split_at(pos);
    let (na, nb) = ver.split_at(pos);

    println!(
        "\x1b[0;1m{:nlen$} {va}\x1b[31;1m{vb:1$}\x1b[0m -> \x1b[0;1m{na}\x1b[32;1m{nb}\x1b[0m",
        pkg.name(),
        vlen - va.len(),
    );
}

pub fn not_found(pkg: &Pkg, nlen: usize, vlen: usize) {
    const MESSAGE: &str = "[not found in AUR]";
    stdout!(
        "{:nlen$} {:vlen$} -> {MESSAGE}",
        "\x1b[0;1m{:nlen$} \x1b[31;1m{:vlen$}\x1b[0m -> \x1b[2m{MESSAGE}\x1b[0m",
        pkg.name(),
        pkg.ver(),
    );
}

#[cold]
pub fn error(e: impl Display) {
    stderr!("error: {e}", "\x1b[31;1merror:\x1b[0m {e}\x1b[0m");
}

pub fn warning(w: impl Display) {
    stderr!("warning: {w}", "\x1b[33;1mwarning:\x1b[0m {w}\x1b[0m");
}
