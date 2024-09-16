#[cfg(test)]
mod tests;

use crate::{
    print,
    types::{Arr, Str},
};
use std::{cmp::Ordering, collections::HashMap};

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum Status {
    UpToDate,
    HasUpdate(Str),
    NotInAUR,
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Pkg {
    name: Str,
    ver: Str,
    status: Status,
}

fn pkg(name: Str, ver: Str, status: Status) -> Pkg {
    Pkg { name, ver, status }
}

pub fn print_status(pkg: Pkg, nlen: usize, vlen: usize) {
    use Status::*;
    match pkg.status {
        UpToDate => print::package(pkg.name, pkg.ver, nlen),
        HasUpdate(new_ver) => print::update(pkg.name, pkg.ver, new_ver, nlen, vlen, true),
        NotInAUR => print::update(pkg.name, pkg.ver, "[not found in AUR]", nlen, vlen, false),
    }
}

pub fn into_state(
    packages: impl IntoIterator<Item = (Str, Str)>,
    mut updates: HashMap<Str, Str>,
    keep_updated: bool,
    keep_failed: bool,
) -> Arr<Pkg> {
    use Status::*;
    packages
        .into_iter()
        .filter_map(|(name, ver)| match updates.remove(&name) {
            Some(new_ver) => match alpm::vercmp(new_ver.as_ref(), ver.as_ref()) {
                Ordering::Greater => Some(pkg(name, ver, HasUpdate(new_ver))),
                _ => keep_updated.then_some(pkg(name, ver, UpToDate)),
            },
            _ => keep_failed.then_some(pkg(name, ver, NotInAUR)),
        })
        .collect()
}

pub fn count_updates(state: &[Pkg]) -> usize {
    use Status::*;
    state
        .iter()
        .filter(|pkg| matches!(pkg.status, HasUpdate(_)))
        .count()
}

pub fn calc_lengths(state: &[Pkg]) -> (usize, usize) {
    state.iter().fold((0, 0), |prev, pkg| {
        (pkg.name.len().max(prev.0), pkg.ver.len().max(prev.1))
    })
}
