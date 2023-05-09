use std::{fmt::Arguments, sync::RwLock};

static COLOR: RwLock<bool> = RwLock::new(false);

pub enum ColorMode {
    Auto,
    Always,
    Never,
}

fn isatty() -> bool {
    // SAFETY: This call should not have side effects.
    unsafe { libc::isatty(libc::STDOUT_FILENO) == 1 }
}

pub fn set_color_mode(mode: ColorMode) {
    use ColorMode::*;
    *COLOR.write().unwrap() = match mode {
        Auto => isatty(),
        Always => true,
        Never => false,
    };
}

macro_rules! print_to {
    ($p: ident, $pl: ident, $c: expr, $n: expr) => {{
        match *COLOR.read().unwrap() {
            true => {
                $p!($c);
                $pl!("\x1b[0m");
            }
            false => $pl!($n),
        };
    }};
}

macro_rules! P {
    ($c: expr, $n: expr $(,)?) => {
        print_to!(print, println, $c, $n)
    };
}

macro_rules! E {
    ($c: expr, $n: expr $(,)?) => {
        print_to!(eprint, eprintln, $c, $n)
    };
}

pub fn header(s: Arguments) {
    P!("\x1b[1;34m::\x1b[0;1m {s}", ":: {s}")
}

pub fn message(s: Arguments) {
    P!("\x1b[0m {s}", " {s}")
}

pub fn update(name: Arguments, ver: Arguments, new_ver: Arguments) {
    P!(
        "\x1b[0;1m{name} \x1b[1;31m{ver}\x1b[0m => \x1b[1;32m{new_ver}",
        "{name} {ver} => {new_ver}"
    )
}

pub fn package(name: Arguments, s: Arguments) {
    P!("\x1b[0;1m{name}\x1b[0m {s}", "{name} {s}")
}

pub fn error(e: Arguments) {
    E!("\x1b[1;31merror:\x1b[0m {e}", "error: {e}")
}

pub fn warning(w: Arguments) {
    E!("\x1b[1;33mwarning:\x1b[0m {w}", "warning: {w}")
}
