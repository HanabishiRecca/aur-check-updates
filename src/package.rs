#[cfg(test)]
mod tests;

use crate::{
    print,
    types::{Arr, Str},
};
use alpm::vercmp;
use std::{cmp::Ordering, collections::HashMap};

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum Status {
    UpToDate(Str, Str),
    HasUpdate(Str, Str, Str),
    NotInAUR(Str, Str),
}

pub fn print_status(status: Status, nlen: usize, vlen: usize) {
    use Status::*;
    match status {
        UpToDate(name, ver) => print::package(name, ver, nlen),
        HasUpdate(name, ver, new_ver) => print::update(name, ver, new_ver, nlen, vlen, true),
        NotInAUR(name, ver) => print::update(name, ver, "[not found in AUR]", nlen, vlen, false),
    }
}

pub fn into_state(
    packages: impl IntoIterator<Item = (Str, Str)>,
    mut updates: HashMap<Str, Str>,
    keep_updated: bool,
    keep_failed: bool,
) -> Arr<Status> {
    use Status::*;
    packages
        .into_iter()
        .filter_map(|(name, ver)| match updates.remove(&name) {
            Some(new_ver) => match vercmp(new_ver.as_ref(), ver.as_ref()) {
                Ordering::Greater => Some(HasUpdate(name, ver, new_ver)),
                _ => keep_updated.then_some(UpToDate(name, ver)),
            },
            _ => keep_failed.then_some(NotInAUR(name, ver)),
        })
        .collect()
}

pub fn count_updates(state: &[Status]) -> usize {
    use Status::*;
    state
        .iter()
        .filter(|status| matches!(status, HasUpdate(_, _, _)))
        .count()
}

pub fn calc_lengths(state: &[Status]) -> (usize, usize) {
    use Status::*;
    state.iter().fold((0, 0), |prev, status| match status {
        UpToDate(name, ver) | HasUpdate(name, ver, _) | NotInAUR(name, ver) => {
            (name.len().max(prev.0), ver.len().max(prev.1))
        }
    })
}
