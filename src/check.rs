use alpm::vercmp;
use std::cmp::Ordering::*;
use std::collections::HashMap;

use crate::aur::request_updates;
use crate::error::R;
use crate::print;

#[cfg_attr(test, derive(Debug, PartialEq))]
enum Status {
    UpToDate,
    HasUpdate(String, String, String),
    NotInAUR(String),
}

use Status::*;

fn print_status(status: Status) {
    match status {
        HasUpdate(name, ver, new_ver) => print::update(
            format_args!("{name}"),
            format_args!("{ver}"),
            format_args!("{new_ver}"),
        ),
        NotInAUR(name) => print::package(format_args!("{name}"), format_args!("is not in AUR")),
        _ => {}
    }
}

fn gen_state(pkgs: Vec<(String, String)>, mut updates: HashMap<String, String>) -> Vec<Status> {
    pkgs.into_iter()
        .map(|(name, ver)| match updates.remove(name.as_str()) {
            Some(new_ver) => match vercmp(new_ver.as_str(), ver.as_str()) {
                Greater => HasUpdate(name, ver, new_ver),
                _ => UpToDate,
            },
            _ => NotInAUR(name),
        })
        .collect()
}

fn count_updates(state: &[Status]) -> usize {
    state
        .iter()
        .filter(|status| matches!(status, HasUpdate(_, _, _)))
        .count()
}

pub fn check_updates(pkgs: Vec<(String, String)>) -> R<()> {
    if pkgs.is_empty() {
        print::message(format_args!("no packages to check"));
        return Ok(());
    }

    let updates = request_updates(pkgs.iter().map(|(name, _)| name.as_str()))?;
    let state = gen_state(pkgs, updates);

    if count_updates(&state) == 0 {
        print::message(format_args!("no updates"));
    }

    state.into_iter().for_each(print_status);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{count_updates, gen_state, Status::*};

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
                NotInAUR(S!("bar"))
            ]
        );
    }
}
