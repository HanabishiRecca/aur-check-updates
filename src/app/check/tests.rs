use super::*;

macro_rules! S {
    ($s: expr) => {
        String::from($s)
    };
}

#[test]
fn state() {
    let pkgs = Vec::from([
        (S!("nop"), S!("1.0.0")),
        (S!("foo"), S!("1.0.0")),
        (S!("bar"), S!("1.0.0")),
    ]);

    let updates = HashMap::from([
        (S!("nop"), S!("1.0.0")),
        (S!("foo"), S!("2.0.0")),
        (S!("baz"), S!("1.0.0")),
    ]);

    let state = gen_state(pkgs, updates);
    assert_eq!(count_updates(&state), 1);

    assert_eq!(
        &state,
        &[
            UpToDate,
            HasUpdate(S!("foo"), S!("1.0.0"), S!("2.0.0")),
            NotInAUR(S!("bar")),
        ]
    );
}
