use super::*;
use std::iter;

macro_rules! read_args {
    ($a: expr) => {
        read_args($a.into_iter())
    };
}

fn cmp(a: &[impl AsRef<str>], b: &[impl AsRef<str>]) {
    assert_eq!(a.len(), b.len());
    assert!(iter::zip(a, b).all(|(a, b)| a.as_ref() == b.as_ref()));
}

#[test]
fn args() {
    let ignores = ["foo", "bar", "baz"];
    let ignore_groups = ["custom"];
    let ignore_suffixes = ["-debug"];
    let dbpath = "/path/to/db";
    let repos = ["core", "extra", "multilib"];
    let endpoint = "https://";
    let timeout = 1234;

    let args = [
        "--ignore",
        &ignores.join(","),
        "--ignoregroup",
        &ignore_groups.join(","),
        "--ignoresuffix",
        &ignore_suffixes.join(","),
        "--color",
        "never",
        "--dbpath",
        dbpath,
        "--repos",
        &repos.join(","),
        "--endpoint",
        endpoint,
        "--timeout",
        &timeout.to_string(),
        "",
    ];

    let config = read_args!(args).unwrap().unwrap();
    cmp(config.ignores().unwrap(), &ignores);
    cmp(config.ignore_groups().unwrap(), &ignore_groups);
    cmp(config.ignore_suffixes().unwrap(), &ignore_suffixes);
    assert_eq!(config.color_mode(), Some(ColorMode::Never));
    assert_eq!(config.dbpath(), Some(dbpath));
    cmp(config.repos().unwrap(), &repos);
    assert_eq!(config.endpoint(), Some(endpoint));
    assert_eq!(config.timeout(), Some(timeout));
}

macro_rules! test_args {
    ($a: expr, $r: expr) => {
        assert_eq!(read_args!($a).unwrap(), $r)
    };
}

#[test]
fn no_args() {
    test_args!([""; 0], Some(Config::default()));
}

#[test]
fn help() {
    test_args!(["--ignore", "foo", "-h", "--foo"], None);
}

macro_rules! test_error {
    ($a: expr, $r: pat) => {
        assert!(matches!(read_args!($a), Err($r)))
    };
}

#[test]
fn no_value() {
    test_error!(["--ignore"], CliError::NoValue(_));
}

#[test]
fn invalid_value() {
    test_error!(["--color", "foo"], CliError::InvalidValue(_, _));
}

#[test]
fn unknown_arg() {
    test_error!(["--foo"], CliError::Unknown(_));
}
