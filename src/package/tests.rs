use super::*;

macro_rules! str {
    ($s: expr) => {
        Str::from($s)
    };
}

macro_rules! pkg {
    ($n: expr, $v: expr) => {
        Pkg::new(str!($n), str!($v))
    };
}

macro_rules! kv {
    ($k: expr, $v: expr) => {
        (str!($k), str!($v))
    };
}

macro_rules! upd {
    ($n: expr, $v: expr, $s: expr) => {
        Record::new(pkg!($n, $v), $s)
    };
}

#[test]
fn state() {
    let pkgs = Arr::from([
        pkg!("nop", "1.0.0"), //
        pkg!("foo", "1.0.0"),
        pkg!("bar", "1.0.0"),
    ]);

    let updates = HashMap::from([
        kv!("nop", "1.0.0"), //
        kv!("foo", "2.0.0"),
        kv!("baz", "1.0.0"),
    ]);

    let state = into_state(pkgs, updates, true, true);
    assert!(state.has_updates());

    use Status::*;
    let result = [
        upd!("nop", "1.0.0", UpToDate), //
        upd!("foo", "1.0.0", HasUpdate(str!("2.0.0"))),
        upd!("bar", "1.0.0", NotInAUR),
    ];

    assert_eq!(state.into_records().as_ref(), result);
}

#[test]
fn partial_state() {
    let pkgs = Arr::from([
        pkg!("nop", "1.0.0"), //
        pkg!("foo", "1.0.0"),
        pkg!("bar", "1.0.0"),
    ]);

    let updates = HashMap::from([
        kv!("nop", "1.0.0"), //
        kv!("foo", "2.0.0"),
        kv!("baz", "1.0.0"),
    ]);

    let state = into_state(pkgs, updates, false, false);
    assert!(state.has_updates());

    use Status::*;
    let result = [
        upd!("foo", "1.0.0", HasUpdate(str!("2.0.0"))), //
    ];

    assert_eq!(state.into_records().as_ref(), result);
}
