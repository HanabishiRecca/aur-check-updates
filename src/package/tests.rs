use super::*;

macro_rules! S {
    ($s: expr) => {
        Str::from($s)
    };
}

#[test]
fn state() {
    let pkgs = [
        (S!("nop"), S!("1.0.0")),
        (S!("foo"), S!("1.0.0")),
        (S!("bar"), S!("1.0.0")),
    ];

    let updates = HashMap::from([
        (S!("nop"), S!("1.0.0")),
        (S!("foo"), S!("2.0.0")),
        (S!("baz"), S!("1.0.0")),
    ]);

    let state = into_state(pkgs, updates, true, true);
    assert_eq!(count_updates(&state), 1);

    use Status::*;
    assert_eq!(
        state.as_ref(),
        [
            UpToDate(S!("nop"), S!("1.0.0")),
            HasUpdate(S!("foo"), S!("1.0.0"), S!("2.0.0")),
            NotInAUR(S!("bar"), S!("1.0.0")),
        ]
    );
}

#[test]
fn partial_state() {
    let pkgs = [
        (S!("nop"), S!("1.0.0")),
        (S!("foo"), S!("1.0.0")),
        (S!("bar"), S!("1.0.0")),
    ];

    let updates = HashMap::from([
        (S!("nop"), S!("1.0.0")),
        (S!("foo"), S!("2.0.0")),
        (S!("baz"), S!("1.0.0")),
    ]);

    let state = into_state(pkgs, updates, false, false);
    assert_eq!(count_updates(&state), 1);

    use Status::*;
    assert_eq!(
        state.as_ref(),
        [HasUpdate(S!("foo"), S!("1.0.0"), S!("2.0.0"))]
    );
}
