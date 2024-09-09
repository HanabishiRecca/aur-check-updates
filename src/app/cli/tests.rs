use std::collections::HashSet;

use crate::{
    error::{ArgError, Error, R},
    print::ColorMode,
};

use super::{read_args, Config};

macro_rules! S {
    ($s: expr) => {
        String::from($s)
    };
}

macro_rules! test_args {
    ($a: expr, $r: expr $(,)?) => {
        assert_eq!(read_args($a.into_iter())?, $r)
    };
}

#[test]
fn no_args() -> R<()> {
    test_args!([], Some(Config::new()));
    Ok(())
}

#[test]
fn args() -> R<()> {
    test_args!(
        [
            S!("--ignore"),
            S!("foo"),
            S!("--ignoregroup"),
            S!("  ,\t  custom  \n,  "),
            S!("  --color  "),
            S!("  never  "),
            S!("--ignore"),
            S!("bar,baz"),
            S!("--timeout"),
            S!("1234"),
            S!("--dbpath"),
            S!("/path/to/db"),
            S!("--repos"),
            S!("core,extra,multilib"),
        ],
        Some(Config {
            ignores: HashSet::from([S!("foo"), S!("bar"), S!("baz")]),
            ignore_groups: HashSet::from([S!("custom")]),
            color_mode: ColorMode::Never,
            timeout: Some(1234),
            dbpath: Some(S!("/path/to/db")),
            repos: HashSet::from([S!("core"), S!("extra"), S!("multilib")]),
        }),
    );
    Ok(())
}

#[test]
fn help() -> R<()> {
    test_args!(
        [
            S!("--ignore"),
            S!("foo"),
            S!("-h"),
            S!("--ignore"),
            S!("foo"),
        ],
        None,
    );
    Ok(())
}

macro_rules! test_error {
    ($a:expr, $r:pat $(,)?) => {
        assert!(matches!(read_args($a.into_iter()), Err(Error::Arg($r)),))
    };
}

use ArgError::*;

#[test]
fn no_value() {
    test_error!([S!("--ignore")], NoValue(_));
}

#[test]
fn invalid_value() {
    test_error!([S!("--color"), S!("foo")], InvalidValue(_, _));
}

#[test]
fn unknown_arg() {
    test_error!([S!("--foo")], Unknown(_));
}
