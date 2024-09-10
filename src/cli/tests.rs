use super::*;

macro_rules! read_args {
    ($a: expr) => {
        read_args($a.into_iter())
    };
}

fn cmp(a: &[impl AsRef<str>], b: &[impl AsRef<str>]) {
    assert_eq!(a.len(), b.len());
    for i in 0..a.len() {
        assert_eq!(a[i].as_ref(), b[i].as_ref());
    }
}

#[test]
fn args() {
    let ignores = &["foo", "bar", "baz"];
    let ignore_groups = &["custom"];
    let dbpath = "/path/to/db";
    let repos = &["core", "extra", "multilib"];
    let endpoint = "https://";
    let timeout = 1234;

    let args = [
        "--ignore",
        &ignores.join(","),
        "--ignoregroup",
        &ignore_groups.join(","),
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
    cmp(config.ignores().unwrap(), ignores);
    cmp(config.ignore_groups().unwrap(), &["custom"]);
    assert_eq!(config.color_mode(), Some(ColorMode::Never));
    assert_eq!(config.dbpath(), Some(dbpath));
    cmp(config.repos().unwrap(), repos);
    assert_eq!(config.endpoint(), Some(endpoint));
    assert_eq!(config.timeout(), Some(timeout));
}

macro_rules! test_args {
    ($a: expr, $r: expr $(,)?) => {
        assert_eq!(read_args!($a).unwrap(), $r)
    };
}

#[test]
fn no_args() {
    test_args!([""; 0], Some(Config::new()));
}

#[test]
fn help() {
    test_args!(["--ignore", "foo", "-h", "--ignore", "foo",], None);
}

macro_rules! test_error {
    ($a: expr, $r: pat $(,)?) => {
        assert!(matches!(read_args!($a), Err($r)))
    };
}

use Error::*;

#[test]
fn no_value() {
    test_error!(["--ignore"], NoValue(_));
}

#[test]
fn invalid_value() {
    test_error!(["--color", "foo"], InvalidValue(_, _));
}

#[test]
fn unknown_arg() {
    test_error!(["--foo"], Unknown(_));
}
